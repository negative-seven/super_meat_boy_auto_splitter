use asr::{deep_pointer::DeepPointer, watcher::Pair, Error, PointerSize, Process};

pub(crate) struct SmbProcess {
    process: Process,
    pointer_paths: PointerPaths,

    pub playing: Pair<u8>,
    pub level_time: Pair<f32>,
    pub world: Pair<u8>,
    pub not_in_cutscene: Pair<u8>,
    pub in_special_level: Pair<u8>,
    pub level_beaten: Pair<u8>,
    pub exit: Pair<u8>,
    pub fetus_type: Pair<u8>,
    pub death_count: Pair<i32>,
    pub characters: Pair<i32>,
    pub level: Pair<u8>,
    pub ui_state: Pair<u8>,
    pub level_transition: Pair<u8>,
    pub fetus: Pair<u32>,
    pub level_type: Pair<i32>,
}

impl SmbProcess {
    pub(crate) fn try_attach() -> Option<Self> {
        let process = Process::attach("SuperMeatBoy.exe")?;

        let pointer_paths = match process.get_module_size("SuperMeatBoy.exe") {
            Ok(0x34_2000) => PointerPaths::og_version(&process),
            Ok(0x33_c000) => PointerPaths::v125(&process),
            _ => return None,
        }
        .unwrap();

        Some(Self {
            process,
            pointer_paths,
            playing: Pair::default(),
            level_time: Pair::default(),
            world: Pair::default(),
            not_in_cutscene: Pair::default(),
            in_special_level: Pair::default(),
            level_beaten: Pair::default(),
            exit: Pair::default(),
            fetus_type: Pair::default(),
            death_count: Pair::default(),
            characters: Pair::default(),
            level: Pair::default(),
            ui_state: Pair::default(),
            level_transition: Pair::default(),
            fetus: Pair::default(),
            level_type: Pair::default(),
        })
    }

    pub(crate) fn is_running(&self) -> bool {
        self.process.is_open()
    }

    pub(crate) fn update_values(&mut self) {
        fn update<T>(process: &Process, field: &mut Pair<T>, pointer_path: DeepPointer<8>)
        where
            T: Copy + bytemuck::Pod,
        {
            if let Ok(value) = pointer_path.deref(process) {
                field.old = field.current;
                field.current = value;
            }
        }

        macro_rules! update {
            ($field:ident) => {
                update(&self.process, &mut self.$field, self.pointer_paths.$field);
            };
        }

        update!(playing);
        update!(level_time);
        update!(world);
        update!(not_in_cutscene);
        update!(in_special_level);
        update!(level_beaten);
        update!(exit);
        update!(fetus_type);
        update!(death_count);
        update!(characters);
        update!(level);
        update!(ui_state);
        update!(level_transition);
        update!(fetus);
        update!(level_type);
    }
}

struct PointerPaths {
    playing: DeepPointer<8>,
    level_time: DeepPointer<8>,
    world: DeepPointer<8>,
    not_in_cutscene: DeepPointer<8>,
    in_special_level: DeepPointer<8>,
    level_beaten: DeepPointer<8>,
    exit: DeepPointer<8>,
    fetus_type: DeepPointer<8>,
    death_count: DeepPointer<8>,
    characters: DeepPointer<8>,
    level: DeepPointer<8>,
    ui_state: DeepPointer<8>,
    level_transition: DeepPointer<8>,
    fetus: DeepPointer<8>,
    level_type: DeepPointer<8>,
}

impl PointerPaths {
    pub(crate) fn og_version(process: &Process) -> Result<Self, Error> {
        let main_module_address = process.get_module_address("SuperMeatBoy.exe")?;
        Ok(Self {
            playing: DeepPointer::new(main_module_address, PointerSize::Bit32, &[0x1b_6638]),
            level_time: DeepPointer::new(main_module_address, PointerSize::Bit32, &[0x1b_6a88]),
            world: DeepPointer::new(main_module_address, PointerSize::Bit32, &[0x1b_7cbc]),
            not_in_cutscene: DeepPointer::new(
                main_module_address,
                PointerSize::Bit32,
                &[0x2d_4c6c, 0x3a0],
            ),
            in_special_level: DeepPointer::new(
                main_module_address,
                PointerSize::Bit32,
                &[0x2d_4c6c, 0x3a4],
            ),
            level_beaten: DeepPointer::new(main_module_address, PointerSize::Bit32, &[0x2d_54a0]),
            exit: DeepPointer::new(main_module_address, PointerSize::Bit32, &[0x2d_54bc, 0x14]),
            fetus_type: DeepPointer::new(
                main_module_address,
                PointerSize::Bit32,
                &[0x2d_54bc, 0x2d2],
            ),
            death_count: DeepPointer::new(
                main_module_address,
                PointerSize::Bit32,
                &[0x2d_55ac, 0x1c8c],
            ),
            characters: DeepPointer::new(
                main_module_address,
                PointerSize::Bit32,
                &[0x2d_55ac, 0x1d24],
            ),
            level: DeepPointer::new(main_module_address, PointerSize::Bit32, &[0x2d_5ea0, 0x8d0]),
            ui_state: DeepPointer::new(
                main_module_address,
                PointerSize::Bit32,
                &[0x2d_5ea0, 0x8d4],
            ),
            level_transition: DeepPointer::new(
                main_module_address,
                PointerSize::Bit32,
                &[0x2d_5ea8],
            ),
            fetus: DeepPointer::new(main_module_address, PointerSize::Bit32, &[0x2d_64bc, 0x10c]),
            level_type: DeepPointer::new(
                main_module_address,
                PointerSize::Bit32,
                &[0x2d_54bc, 0x1ac],
            ),
        })
    }

    pub(crate) fn v125(process: &Process) -> Result<Self, Error> {
        let main_module_address = process.get_module_address("SuperMeatBoy.exe")?;
        Ok(Self {
            playing: DeepPointer::new(main_module_address, PointerSize::Bit32, &[0x30_a1c8]),
            level_time: DeepPointer::new(main_module_address, PointerSize::Bit32, &[0x2f_6abc]),
            world: DeepPointer::new(main_module_address, PointerSize::Bit32, &[0x2f_79ac]),
            not_in_cutscene: DeepPointer::new(
                main_module_address,
                PointerSize::Bit32,
                &[0x30_999c, 0x3a8],
            ),
            in_special_level: DeepPointer::new(
                main_module_address,
                PointerSize::Bit32,
                &[0x30_999c, 0x3a4],
            ),
            level_beaten: DeepPointer::new(main_module_address, PointerSize::Bit32, &[0x30_a1e0]),
            exit: DeepPointer::new(main_module_address, PointerSize::Bit32, &[0x30_a1a0, 0x14]),
            fetus_type: DeepPointer::new(
                main_module_address,
                PointerSize::Bit32,
                &[0x30_a1a0, 0x352],
            ),
            death_count: DeepPointer::new(
                main_module_address,
                PointerSize::Bit32,
                &[0x30_a380, 0x38ac],
            ),
            characters: DeepPointer::new(
                main_module_address,
                PointerSize::Bit32,
                &[0x30_a380, 0x3950],
            ),
            level: DeepPointer::new(main_module_address, PointerSize::Bit32, &[0x30_ac90, 0x8dc]),
            ui_state: DeepPointer::new(
                main_module_address,
                PointerSize::Bit32,
                &[0x30_ac90, 0x8e0],
            ),
            level_transition: DeepPointer::new(
                main_module_address,
                PointerSize::Bit32,
                &[0x30_ad00],
            ),
            fetus: DeepPointer::new(main_module_address, PointerSize::Bit32, &[0x30_b3e4, 0x10c]),
            level_type: DeepPointer::new(
                main_module_address,
                PointerSize::Bit32,
                &[0x30_a1a0, 0x3c68],
            ),
        })
    }
}
