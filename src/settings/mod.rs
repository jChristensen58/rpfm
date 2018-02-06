// In this module should be everything related to the settings stuff.
extern crate serde_json;
extern crate failure;

use std::path::PathBuf;
use std::fs::File;
use std::io::Write;

use self::failure::Error;

/// This struct hold every setting of the program, and it's the one that we are going to serialize.
/// The default game is the position in the list of the game:
/// - 0 -> Warhammer 2.
/// - 1 -> Warhammer 1.
/// - 2 -> Attila.
/// - 3 -> Rome 2.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Settings {
    pub paths: Paths,
    pub default_game: i32,
}

/// This struct should hold any path we need to store in the settings.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Paths {
    pub my_mods_base_path: Option<PathBuf>,
    pub warhammer_2: Option<PathBuf>,
    pub warhammer_1: Option<PathBuf>,
    pub attila: Option<PathBuf>,
    pub rome_2: Option<PathBuf>,
}

/// Implementation of Settings.
impl Settings {

    /// This function creates a new settings file with default values and loads it into memory.
    /// Should be run if no settings file has been found at the start of the program.
    pub fn new() -> Settings {
        Settings {
            paths: Paths::new(),
            default_game: 0,
        }
    }

    /// This function takes a settings.json file and reads it into a "Settings" object.
    pub fn load() -> Result<Settings, Error> {
        let settings_file = File::open("settings.json")?;
        let settings = serde_json::from_reader(settings_file)?;
        Ok(settings)
    }

    /// This function takes the Settings object and saves it into a settings.json file.
    pub fn save(&self) -> Result<(), Error> {
        let settings_json = serde_json::to_string_pretty(self);
        match File::create(PathBuf::from("settings.json")) {
            Ok(mut file) => {
                match file.write_all(&settings_json.unwrap().as_bytes()) {
                    Ok(_) => Ok(()),
                    Err(_) => Err(format_err!("Error while trying to write the \"settings.json\" file.")),
                }
            },
            Err(_) => Err(format_err!("Error while trying prepare the \"settings.json\" file to be written."))
        }
    }
}

/// Implementation of Paths.
impl Paths {

    /// This function creates a set of empty paths. Just for the initial creation of the settings file.
    pub fn new() -> Paths {
        Paths {
            my_mods_base_path: None,
            warhammer_2: None,
            warhammer_1: None,
            attila: None,
            rome_2: None,
        }
    }
}