use serde_json::{self, Value};
use std::{collections::HashSet, fs};

const CONFIG_PATH: &str = "data/config.json";
const REQUEST_PATH: &str = "data/remove_requests.json";

const CONFIG_DEFAULT: &str = r#"{
		"CHANNEL": "User",
		"OAUTH_TOKEN": "None",
		"WINDOW_SIZE": [1280, 720],
		"BG_COL": [40, 40, 40],
		"FPS": 24,
		"SHOW_MESSAGES": true,
		"CAPTURE": ["broadcaster", "admins", "moderators", "viewers", "vips"],
		"FILTERED_WORDS": [],
		"COLOUR_PALETTE": [
			[217, 215, 241], [255, 253, 222], [231, 251, 190], [255, 203, 203], [211, 228, 205], [173, 194, 169], 
			[153, 167, 153], [198, 213, 126], [213, 126, 126], [162, 205, 205], [255, 225, 175] 
		]
	}"#;

const REQUEST_DEFAULT: &str = r#"{
	"requests": []
}"#;

#[derive(Clone)]
pub struct Config {
    pub settings: Value,
    pub requests: HashSet<String>,
    pub display_size: (u32, u32),
}

impl Config {
    pub fn new() -> Config {
        let mut to_save = false;

        let settings: Value = match fs::read_to_string(CONFIG_PATH) {
            Ok(val) => serde_json::from_str(&val).unwrap(),
            Err(_) => {
                to_save = true;
                serde_json::from_str(CONFIG_DEFAULT).unwrap()
            }
        };

        let requests: Value = match fs::read_to_string(REQUEST_PATH) {
            Ok(val) => serde_json::from_str(&val).unwrap(),
            Err(_) => {
                to_save = true;
                serde_json::from_str(REQUEST_DEFAULT).unwrap()
            }
        };

        let requests = match requests.as_array() {
            Some(val) => val.iter().map(Value::to_string).collect(),
            None => HashSet::new(),
        };

        let display_size =
            serde_json::from_slice::<(u32, u32)>(settings["WINDOW_SIZE"].to_string().as_bytes())
                .unwrap();

        let cfg = Config {
            settings,
            requests,
            display_size,
        };

        if to_save {
            cfg.save();
        }

        cfg
    }

    pub fn save(&self) {
        let config_str = self.settings.to_string();
        let req_str: String = format!(
            "{{ \"requests\": [ {} ] }}",
            self.requests
                .iter()
                .flat_map(|s| s.chars())
                .collect::<String>()
        );

        fs::write(CONFIG_PATH, config_str).unwrap();
        fs::write(REQUEST_PATH, req_str).unwrap();
    }
}
