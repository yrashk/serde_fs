//! # Simple "config" example
//!
//! This example will create a default config if none is found (in `test_config` directory),
//! or read existing one and will print it out as a JSON
//!
extern crate serde_fs;
#[macro_use] extern crate serde_derive;
extern crate serde_bytes;
extern crate serde_json;

#[derive(Serialize, Deserialize, Default)]
struct Config {
    name: String,
    values: Vec<u8>,
    #[serde(with = "serde_bytes")]
    file: Vec<u8>,
}

fn main() {
    use std::env;
    let config_dir = env::current_dir().unwrap().join("test_config");

    let maybe_cfg: Option<Config> = serde_fs::from_fs(&config_dir).unwrap();

    if maybe_cfg.is_none() {
        eprintln!("Config not found, creating one. You can edit it and re-run the example");
        serde_fs::to_fs(&config_dir, &Config::default()).unwrap();
    }

    let cfg: Config = serde_fs::from_fs(&config_dir).unwrap();
    println!("{}", serde_json::to_string_pretty(&cfg).unwrap());
}