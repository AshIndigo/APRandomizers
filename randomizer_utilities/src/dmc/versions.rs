use crate::dmc::versions::Game::Unknown;
use std::env::current_exe;
use std::fmt::{Display, Formatter};
use std::fs;
use std::io::ErrorKind;
use std::sync::LazyLock;
use xxhash_rust::const_xxh3::xxh3_64;

// Records of various DMCHDC hashes and mods (Not complete, need GOG)
#[derive(Debug, Clone, Copy, strum_macros::Display)]
pub enum Game {
    // HD Collection
    DMCLauncher,
    DMC1,
    DMC2,
    DMC3,

    Unknown,
}
#[derive(Debug, Clone, Copy, strum_macros::Display)]
pub enum Mod {
    // Mods (Probably not going to add every DDMK/Crimson Version, only from the time of writing and onwards)
    Eva,
    Lucia,
    Mary,
    Crimson,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct VersionInformation {
    hash: u64,
    pub valid_for_use: bool,
    pub description: &'static str,
    pub game_type: Game,
    pub mod_type: Option<Mod>,
}

impl Display for VersionInformation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

static EMPTY: LazyLock<Vec<VersionInformation>> = LazyLock::new(Vec::new);

static DMC1_INFO: LazyLock<Vec<VersionInformation>> = LazyLock::new(|| {
    vec![
        VersionInformation {
            hash: 16596094990179088469,
            valid_for_use: true,
            description: "DDMK Patched DMC1",
            game_type: Game::DMC1,
            mod_type: None,
        },
        VersionInformation {
            hash: 10860670779859874529,
            valid_for_use: true,
            description: "Version #1 DMC1",
            game_type: Game::DMC1,
            mod_type: None,
        },
        VersionInformation {
            hash: 342337984247752146,
            valid_for_use: true,
            description: "Version #2 DMC1",
            game_type: Game::DMC1,
            mod_type: None,
        },
        VersionInformation {
            hash: 6932768196842012018,
            valid_for_use: false,
            description: "Latest DMC1",
            game_type: Game::DMC1,
            mod_type: None,
        },
    ]
});

static DMC2_INFO: LazyLock<Vec<VersionInformation>> = LazyLock::new(|| {
    vec![
        VersionInformation {
            hash: 8566769405802122008,
            valid_for_use: true,
            description: "DDMK Patched DMC2",
            game_type: Game::DMC2,
            mod_type: None,
        },
        VersionInformation {
            hash: 4868173191699540308,
            valid_for_use: true,
            description: "Version #1 DMC2",
            game_type: Game::DMC2,
            mod_type: None,
        },
        VersionInformation {
            hash: 5981905978386037807,
            valid_for_use: true,
            description: "Version #2 DMC2",
            game_type: Game::DMC2,
            mod_type: None,
        },
        VersionInformation {
            hash: 7733538334450880217,
            valid_for_use: false,
            description: "Latest DMC2",
            game_type: Game::DMC2,
            mod_type: None,
        },
    ]
});

static DMC3_INFO: LazyLock<Vec<VersionInformation>> = LazyLock::new(|| {
    vec![
        VersionInformation {
            hash: 9031715114876197692,
            valid_for_use: true,
            description: "DDMK Patched DMC3",
            game_type: Game::DMC3,
            mod_type: None,
        },
        VersionInformation {
            hash: 7198991379004446668,
            valid_for_use: true,
            description: "Crimson Patched DMC3",
            game_type: Game::DMC3,
            mod_type: None,
        },
        VersionInformation {
            hash: 14598701335922013533,
            valid_for_use: true,
            description: "Version #1 DMC3",
            game_type: Game::DMC3,
            mod_type: None,
        },
        VersionInformation {
            hash: 6772293939166567304,
            valid_for_use: true,
            description: "Version #2 DMC3",
            game_type: Game::DMC3,
            mod_type: None,
        },
        VersionInformation {
            hash: 11219846177156872589,
            valid_for_use: false,
            description: "Latest DMC3",
            game_type: Game::DMC3,
            mod_type: None,
        },
    ]
});

static DMC_LAUNCHER_INFO: LazyLock<Vec<VersionInformation>> = LazyLock::new(|| {
    vec![
        VersionInformation {
            hash: 3012265650586028916,
            valid_for_use: true,
            description: "Crimson/DDMK Patched DMC Launcher",
            game_type: Game::DMCLauncher,
            mod_type: None,
        },
        VersionInformation {
            hash: 9711695139658080865,
            valid_for_use: true,
            description: "Version #1 DMC Launcher",
            game_type: Game::DMCLauncher,
            mod_type: None,
        },
        VersionInformation {
            hash: 14560228364278330367,
            valid_for_use: true,
            description: "Version #2 DMC Launcher",
            game_type: Game::DMCLauncher,
            mod_type: None,
        },
        VersionInformation {
            hash: 8868518716288212586,
            valid_for_use: true,
            description: "Latest DMC Launcher",
            game_type: Game::DMCLauncher,
            mod_type: None,
        },
    ]
});

static EVA_INFO: LazyLock<Vec<VersionInformation>> = LazyLock::new(|| {
    vec![VersionInformation {
        hash: 2536699235936189826,
        valid_for_use: true,
        description: "2.7.3 DDMK - Eva",
        game_type: Game::DMC1,
        mod_type: Some(Mod::Eva),
    }]
});

static LUCIA_INFO: LazyLock<Vec<VersionInformation>> = LazyLock::new(|| {
    vec![VersionInformation {
        hash: 16520636509798662806,
        valid_for_use: true,
        description: "2.7.3 DDMK - Lucia",
        game_type: Game::DMC2,
        mod_type: Some(Mod::Lucia),
    }]
});

static MARY_INFO: LazyLock<Vec<VersionInformation>> = LazyLock::new(|| {
    vec![VersionInformation {
        hash: 7087074874482460961,
        valid_for_use: true,
        description: "2.7.3 DDMK - Mary",
        game_type: Game::DMC3,
        mod_type: Some(Mod::Mary),
    }]
});

static CRIMSON_INFO: LazyLock<Vec<VersionInformation>> = LazyLock::new(|| {
    vec![VersionInformation {
        hash: 6027093939875741571,
        valid_for_use: true,
        description: "0.4 Crimson",
        game_type: Game::DMC3,
        mod_type: Some(Mod::Crimson),
    }]
});

impl Game {
    const DMC_LAUNCHER: &str = "dmcLauncher.exe";
    const DMC1_EXE: &str = "dmc1.exe";
    const DMC2_EXE: &str = "dmc2.exe";
    const DMC3_EXE: &str = "dmc3.exe";
    const UNKNOWN_EXE: &str = "Unknown";
    pub fn get_information(&self) -> &Vec<VersionInformation> {
        match self {
            Game::DMCLauncher => &DMC_LAUNCHER_INFO,
            Game::DMC1 => &DMC1_INFO,
            Game::DMC2 => &DMC2_INFO,
            Game::DMC3 => &DMC3_INFO,
            Unknown => &EMPTY,
        }
    }

    pub fn get_file_name(&self) -> &str {
        match self {
            Game::DMCLauncher => Self::DMC_LAUNCHER,
            Game::DMC1 => Self::DMC1_EXE,
            Game::DMC2 => Self::DMC2_EXE,
            Game::DMC3 => Self::DMC3_EXE,
            Unknown => Self::UNKNOWN_EXE,
        }
    }

    pub fn get_current_game() -> Game {
        match current_exe()
            .unwrap()
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
        {
            Self::DMC_LAUNCHER => Game::DMCLauncher,
            Self::DMC1_EXE => Game::DMC1,
            Self::DMC2_EXE => Game::DMC2,
            Self::DMC3_EXE => Game::DMC3,
            &_ => Unknown,
        }
    }

    pub fn get_mods_for_game(&self) -> Vec<Mod> {
        match self {
            Game::DMCLauncher => vec![],
            Game::DMC1 => vec![Mod::Eva],
            Game::DMC2 => vec![Mod::Lucia],
            Game::DMC3 => vec![Mod::Crimson, Mod::Mary],
            Unknown => Vec::new(),
        }
    }

    pub fn get_current_version(&self) -> Result<VersionInformation, std::io::Error> {
        let hash = xxh3_64(&fs::read(self.get_file_name())?);
        let res = *self
            .get_information()
            .iter()
            .find(|ver| ver.hash == hash)
            .unwrap_or(&VersionInformation {
                hash,
                valid_for_use: false,
                description: "Uncatalogued Version",
                game_type: *self,
                mod_type: None,
            });
        Ok(res)
    }

    pub fn identify_mods(&self) -> Vec<VersionInformation> {
        let mut mods = vec![];
        for game_mod in self.get_mods_for_game() {
            match game_mod.get_mod_version() {
                Ok(ver) => {
                    mods.push(ver);
                }
                Err(err) => match err.kind() {
                    ErrorKind::NotFound => {}
                    _ => {
                        log::error!("Unable to identify {} version: {}", game_mod, err);
                    }
                },
            }
        }
        mods
    }
}

impl Mod {
    fn get_information(&self) -> &Vec<VersionInformation> {
        match self {
            Mod::Eva => &EVA_INFO,
            Mod::Lucia => &LUCIA_INFO,
            Mod::Mary => &MARY_INFO,
            Mod::Crimson => &CRIMSON_INFO,
        }
    }

    pub fn get_file_name(&self) -> &str {
        match self {
            Mod::Eva => "Eva.dll",
            Mod::Lucia => "Lucia.dll",
            Mod::Mary => "Mary.dll",
            Mod::Crimson => "Crimson.dll",
        }
    }

    fn get_mod_version(self) -> Result<VersionInformation, std::io::Error> {
        let hash = xxh3_64(&fs::read(self.get_file_name())?);
        let res = *self
            .get_information()
            .iter()
            .find(|ver| ver.hash == hash)
            .unwrap_or(&VersionInformation {
                hash,
                valid_for_use: false,
                description: "Uncatalogued Version",
                game_type: self.get_game_for_mod(),
                mod_type: Some(self),
            });
        Ok(res)
    }

    fn get_game_for_mod(&self) -> Game {
        match self {
            Mod::Eva => Game::DMC1,
            Mod::Lucia => Game::DMC2,
            Mod::Mary | Mod::Crimson => Game::DMC3,
        }
    }
}

pub fn is_file_valid(file_path: &str, expected_hash: u64) -> Result<(), std::io::Error> {
    let data = fs::read(file_path)?;
    if xxh3_64(&data) == expected_hash {
        Ok(())
    } else {
        Err(std::io::Error::new(
            ErrorKind::InvalidData,
            "File has invalid hash",
        ))
    }
}
