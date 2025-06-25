#![no_std]

mod auto_splitter;
mod smb_process;

use crate::{auto_splitter::AutoSplitter, smb_process::SmbProcess};
use asr::{future::next_tick, settings::Gui, Error};

asr::async_main!(stable);
asr::panic_handler!();

#[derive(Gui)]
struct Settings {
    /// Reset on the main menu
    #[default = false]
    reset_on_main_menu: bool,

    /// Split after every level
    #[default = false]
    split_after_level: bool,

    /// IW mode
    #[default = false]
    iw_mode: bool,

    /// IW mode - only start on the first level of a world
    #[default = true]
    iw_mode_split_on_first_level: bool,

    /// Dark ending mode
    #[default = true]
    dark_ending: bool,

    /// Split when entering boss 1
    #[default = false]
    split_before_boss_1: bool,

    /// Split when entering boss 2
    #[default = false]
    split_before_boss_2: bool,

    /// Split when entering boss 3
    #[default = false]
    split_before_boss_3: bool,

    /// Split when entering boss 4
    #[default = false]
    split_before_boss_4: bool,

    /// Split when entering boss 5
    #[default = false]
    split_before_boss_5: bool,

    /// Split when entering boss 6
    #[default = false]
    split_before_boss_6: bool,

    /// Death count display
    #[default = false]
    death_counter: bool,

    /// Freeze the death counter when the run ends
    #[default = false]
    freeze_death_counter_on_finish: bool,

    /// Last IL time display
    #[default]
    level_time: bool,
}

impl Settings {
    pub(crate) fn split_before_boss(&self, boss_index: u8) -> bool {
        match boss_index {
            1 => self.split_before_boss_1,
            2 => self.split_before_boss_2,
            3 => self.split_before_boss_3,
            4 => self.split_before_boss_4,
            5 => self.split_before_boss_5,
            6 => self.split_before_boss_6,
            _ => false,
        }
    }
}

async fn main() {
    inner_main().await.unwrap();
}

async fn inner_main() -> Result<(), Error> {
    let mut settings = Settings::register();

    loop {
        let process = loop {
            if let Some(process) = SmbProcess::try_attach() {
                break process;
            }
            next_tick().await;
        };

        let mut auto_splitter = AutoSplitter::new(process, &mut settings);
        while auto_splitter.is_process_running() {
            auto_splitter.run_tick();
            next_tick().await;
        }
    }
}
