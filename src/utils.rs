use serde_json::Value;
use sfml::graphics::Color;
use crate::config::Config;


pub fn colour_from_json(json_value: &Value) -> Color {
	let raw_col: Vec<u8> = json_value.as_array().unwrap().iter().map(|s| s.as_f64().unwrap() as u8).collect();
    Color::rgb(raw_col[0], raw_col[1], raw_col[2])
}

pub fn colour_from_cfg(cfg: &Config, section: &str) -> Color {
	colour_from_json(&cfg.settings[section])
}