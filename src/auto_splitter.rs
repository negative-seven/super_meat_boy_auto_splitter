use crate::{
    smb_process::{GameState, SmbProcess},
    Settings,
};
use asr::{settings::Gui, timer::TimerState};
use core::f32;

pub(crate) struct AutoSplitter<'settings> {
    process: SmbProcess,
    settings: &'settings mut Settings,
    timer_previous_state: TimerState,
    level_time: f32,
    death_count_offset: i32,
}

impl<'settings> AutoSplitter<'settings> {
    const DUMMY_LEVEL_TIME: f32 = 1e8;

    pub(crate) fn new(process: SmbProcess, settings: &'settings mut Settings) -> Self {
        let mut this = Self {
            process,
            settings,
            level_time: f32::NAN,
            death_count_offset: 0,
            timer_previous_state: TimerState::Unknown,
        };
        this.init();
        this
    }

    pub(crate) fn is_process_running(&self) -> bool {
        self.process.is_running()
    }

    pub(crate) fn run_tick(&mut self) {
        self.settings.update();
        self.process.update_values();

        self.update();

        if matches!(
            asr::timer::state(),
            TimerState::Running | TimerState::Paused
        ) && self.reset()
        {
            asr::timer::reset();
            return;
        }

        if self.split() {
            asr::timer::split();
        }

        if asr::timer::state() == TimerState::NotRunning && self.start() {
            asr::timer::start();

            if asr::timer::state() == TimerState::Running {
                // TODO: run this even if the timer is started manually
            }
        }

        if self.timer_previous_state == TimerState::NotRunning
            && asr::timer::state() == TimerState::Running
        {
            self.on_start();
        }

        self.timer_previous_state = asr::timer::state();
    }

    fn init(&mut self) {
        asr::timer::set_variable_int("Deaths", self.process.death_count.current);
        self.death_count_offset = 0;

        if self.settings.level_time {
            asr::timer::set_variable_float("Last IL Time", 0.0);
        }

        // In 1.2.5 watching a replay still counts as playing (playing == 1), because of
        // that exiting to the map after completing the level doesn't split
        //
        // This variable is set when ingame variable changes from
        // `Self::DUMMY_LEVEL_TIME` to an IL time and resets back to 1e8 when exiting
        // the main game (playing == 0), going to the next level (levelBeaten == 1), or
        // entering a cutscene (notCutscene == 0)
        self.level_time = Self::DUMMY_LEVEL_TIME;
    }

    fn update(&mut self) {
        // Update the death counter
        if !self.is_death_counter_frozen() && self.process.death_count.increased() {
            asr::timer::set_variable_int(
                "Deaths",
                self.process.death_count.current - self.death_count_offset,
            );
        }

        // Update the level time display. The level time stays at
        // `Self::DUMMY_LEVEL_TIME` while playing the level.
        if self.is_level_time_display_active()
            && self
                .process
                .level_time
                .bytes_changed_from(&Self::DUMMY_LEVEL_TIME)
        {
            // The timer glitch may cause the level time to be 0.0 here.
            asr::timer::set_variable_float("Last IL Time", self.process.level_time.current);
        }

        // Update the level time
        if self
            .process
            .level_time
            .bytes_changed_from(&Self::DUMMY_LEVEL_TIME)
        {
            self.level_time = self.process.level_time.current;
        }

        if self.process.level_beaten.changed_from_to(&0, &1)
            || self.process.playing.current == 0
            || self.process.not_in_cutscene.current == 0
        {
            self.level_time = Self::DUMMY_LEVEL_TIME;
        }
    }

    fn reset(&self) -> bool {
        if self.process.game_state.current == GameState::TitleScreen {
            return true;
        }

        if self.settings.reset_on_main_menu
            && self.process.game_state.current == GameState::MainMenu
        {
            return true;
        }

        false
    }

    fn split(&self) -> bool {
        // Boss completion splits
        if self.process.game_state.current == GameState::Playing
            && self.process.not_in_cutscene.changed_from_to(&1, &0)
            && (self.process.world.current != 6 || self.settings.split_after_level)
            && self.process.level.current == 99
        {
            return true;
        }

        // Final cutscene splits
        if self.process.fetus.changed_to(&0x8000_0000)
            && (!(self.process.level_type.current == 0 && self.settings.dark_ending)
                || self.settings.split_after_level)
        {
            return true;
        }

        // IL splits
        if self.settings.split_after_level {
            if self.process.level_beaten.changed_from_to(&0, &1) {
                return true;
            }

            if self.process.level_transition.changed_from_to(&0, &1)
                && self.process.game_state.current == GameState::Playing
                && (self.level_time != Self::DUMMY_LEVEL_TIME || self.process.playing.old == 0)
            {
                return true;
            }

            if self.process.game_state.current == GameState::Playing
                && ([0, 1].contains(&self.process.level_type.old))
                && (2..=5).contains(&self.process.level_type.current)
            {
                return true;
            }

            if ([0, 1].contains(&self.process.level_type.current))
                && ((self.process.level_type.old >= 2 && self.process.level.old == 2)
                    || (self.process.level_type.old == 6 && self.process.level.old == 0))
                && self.process.level_time.current != Self::DUMMY_LEVEL_TIME
            {
                return true;
            }
        }

        // Boss entrance split
        if self.settings.split_before_boss(self.process.world.current)
            && self.process.game_state.current == GameState::EnteringLevel
            && self.process.in_special_level.changed_from_to(&0, &1)
        {
            return true;
        }

        // IW ending split
        if self.settings.iw_mode
            && ((self.process.world.current == 6 && self.process.level.current == 4)
                || self.process.level.current == 19)
            && self.process.playing.old == 1
            && self.process.level_time.old == Self::DUMMY_LEVEL_TIME
            && self.process.level_time.current != Self::DUMMY_LEVEL_TIME
        {
            return true;
        }

        // Dark Ending splits
        if self.settings.dark_ending
            && !self.settings.split_after_level
            && self.process.game_state.changed_from_to(
                &GameState::Playing,
                &GameState::LevelSelectionWithBossUnlocking,
            )
            && (1..=5).contains(&self.process.world.current)
        {
            return true;
        }

        false
    }

    fn start(&self) -> bool {
        if !self.settings.iw_mode
            && self.process.game_state.current == GameState::EnteringChapterSelection
        {
            return true;
        }

        if self.settings.iw_mode
            && ((self.process.characters.current != 1
                && self.process.game_state.changed_from_to(
                    &GameState::CharacterSelection,
                    &GameState::CharacterSelectionWithCharacterSelected,
                ))
                || ((self.process.characters.current == 1
                    || ([6, 7].contains(&self.process.world.current)))
                    && self
                        .process
                        .game_state
                        .changed_from_to(&GameState::LevelSelection, &GameState::EnteringLevel)))
            && (self.process.level.current == 0 || !self.settings.iw_mode_split_on_first_level)
        {
            return true;
        }

        false
    }

    fn on_start(&mut self) {
        self.death_count_offset = self.process.death_count.old;
        asr::timer::set_variable_int(
            "Deaths",
            self.process.death_count.current - self.death_count_offset,
        );
    }

    fn is_death_counter_frozen(&self) -> bool {
        self.settings.freeze_death_counter_on_finish && asr::timer::state() == TimerState::Ended
    }

    fn is_level_time_display_active(&self) -> bool {
        self.settings.level_time
    }
}
