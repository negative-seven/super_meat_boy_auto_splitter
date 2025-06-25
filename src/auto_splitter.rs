use crate::{smb_process::SmbProcess, Settings};
use asr::{settings::Gui, timer::TimerState};
use core::f32;

pub(crate) struct AutoSplitter<'settings> {
    process: SmbProcess,
    settings: &'settings mut Settings,
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

        if self.reset() {
            asr::timer::reset();
            return;
        }

        if self.split() {
            asr::timer::split();
        }

        if asr::timer::state() == TimerState::NotRunning {
            if self.start() {
                asr::timer::start();
            }

            // TODO: run this even if the timer is started manually
            if self.settings.death_counter {
                self.death_count_offset = self.process.death_count.old;
                asr::timer::set_variable_int(
                    "Deaths",
                    self.process.death_count.current - self.death_count_offset,
                );
            }
        }
    }

    fn init(&mut self) {
        if self.settings.death_counter {
            asr::timer::set_variable_int("Deaths", self.process.death_count.current);
            self.death_count_offset = 0;
        }

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
        if self.settings.death_counter
            && self.process.death_count.increased()
            && (asr::timer::state() != TimerState::Ended
                || !self.settings.freeze_death_counter_on_finish)
        {
            asr::timer::set_variable_int(
                "Deaths",
                self.process.death_count.current - self.death_count_offset,
            );
        }

        // Update the level time display. The level time stays at
        // `Self::DUMMY_LEVEL_TIME` while playing the level.
        if self.settings.level_time
            && self.process.level_time.old == Self::DUMMY_LEVEL_TIME
            && self.process.level_time.current != Self::DUMMY_LEVEL_TIME
        {
            // The timer glitch may cause the level time to be 0.0 here.
            asr::timer::set_variable_float("Last IL Time", self.process.level_time.current);
        }

        // Update the level time
        if self.process.level_time.old == Self::DUMMY_LEVEL_TIME
            && self.process.level_time.current != Self::DUMMY_LEVEL_TIME
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
        // On the title screen
        if self.process.ui_state.current == 11 {
            return true;
        }

        // On the main menu
        if self.settings.reset_on_main_menu && self.process.ui_state.current == 15 {
            return true;
        }

        false
    }

    fn split(&self) -> bool {
        // Boss completion splits
        if self.process.ui_state.current == 0
            && self.process.not_in_cutscene.changed_from_to(&1, &0)
            && (self.process.world.current != 6 || self.settings.split_after_level)
            && self.process.level.current == 99
        {
            return true;
        }

        // Final cutscene splits
        if self.process.fetus.changed_to(&0x8000_0000)
            && (!(self.process.fetus_type.current == 0 && self.settings.dark_ending)
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
                && self.process.ui_state.current == 0
                && (self.level_time != Self::DUMMY_LEVEL_TIME || self.process.playing.old == 0)
            {
                return true;
            }

            if self.process.ui_state.current == 0
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

        let split_before_boss = match self.process.world.current {
            1 => self.settings.split_before_boss_1,
            2 => self.settings.split_before_boss_2,
            3 => self.settings.split_before_boss_3,
            4 => self.settings.split_before_boss_4,
            5 => self.settings.split_before_boss_5,
            6 => self.settings.split_before_boss_6,
            _ => false,
        };

        // Boss entrance split
        if split_before_boss
            && self.process.ui_state.current == 7
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
            && self.process.ui_state.changed_from_to(&0, &22)
            && (1..=5).contains(&self.process.world.current)
        {
            return true;
        }

        false
    }

    fn start(&self) -> bool {
        if !self.settings.iw_mode && self.process.ui_state.current == 13 {
            return true;
        }

        if self.settings.iw_mode
            && ((self.process.characters.current != 1
                && self.process.ui_state.changed_from_to(&4, &5))
                || ((self.process.characters.current == 1
                    || ([6, 7].contains(&self.process.world.current)))
                    && self.process.ui_state.changed_from_to(&1, &7)))
            && (self.process.level.current == 0 || !self.settings.iw_mode_split_on_first_level)
        {
            return true;
        }

        false
    }
}
