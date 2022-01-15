pub mod console_utils;

use console_utils::console;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::Path;
use serde::{Deserialize, Serialize};
use regex::Regex;
use ansi_term::Color;
use std::io;
use std::io::Write;
use lazy_static::lazy_static;

#[derive(Serialize, Deserialize)]
struct ConfigOption {
    name: String,
    id: String,
    default: Option<String>,
    description: String
}

#[derive(Serialize, Deserialize)]
struct ProjectConfig {
    template: String,
    path: String,
    options: Vec<ConfigOption>
}

lazy_static! {
    static ref OPTION_REGEX: Regex = Regex::new(r"\$\{\{(.+)}}").unwrap();
}

fn main() {
    let config_path = Path::new(
        env::current_dir().unwrap().to_str().unwrap()
    ).join(".confgen.yaml");
    let conf: ProjectConfig = match serde_yaml::from_str(
        &*fs::read_to_string(config_path).unwrap()
    ) {
        Ok(json) => json,
        Err(err) => panic!("Error parsing config file: {:?}", err)
    };

    println!(
        "{} {}",
        Color::Cyan.bold().paint("::"),
        Color::Green.bold().paint("Starting interactive configuration setup.")
    );
    println!(
        "{} {}",
        Color::Purple.bold().paint("::"),
        Color::Green.bold().paint(
            format!("Configuration will be saved in {}.", conf.path)
        )
    );

    let mut opts = HashMap::new();

    for cap in OPTION_REGEX.captures_iter(&*conf.template) {
        let opt_name = cap.get(1).unwrap().as_str();
        let opt_conf = match conf.options.iter().find(|opt| opt.id == opt_name.to_string()) {
            Some(opt) => opt,
            None => console::err!(format!("Option {} not found in configuration file.", opt_name))
        };
        println!(
            "{} {}: {}",
            Color::Yellow.bold().paint("::"),
            Color::Blue.bold().paint("Name"),
            Color::Cyan.paint(format!("{}", opt_conf.name))
        );
        println!(
            "{} {}: {}",
            Color::Yellow.bold().paint("::"),
            Color::Blue.bold().paint("Description"),
            Color::Cyan.paint(format!("{}", opt_conf.description))
        );
        println!(
            "{} {}: {}",
            Color::Yellow.bold().paint("::"),
            Color::Blue.bold().paint("Default"),
            Color::Cyan.paint(format!("{}", opt_conf.default.as_ref().unwrap_or(&"None".to_string())))
        );
        let input = console::prompt!("Value", opt_conf.name, opt_conf.default.as_ref());
        opts.insert(opt_name, String::from(input));
        println!();
    }

    let mut completed_config = conf.template.clone();
    for opt in opts {
        completed_config = completed_config.replace(format!("${{{{{}}}}}", opt.0).as_str(), &*opt.1);
    }

    let mut file = fs::File::create(conf.path).unwrap();
    file.write_all(completed_config.as_bytes()).unwrap();

    println!(
        "{} {}",
        Color::Cyan.bold().paint("::"),
        Color::Green.bold().paint("Finished interactive configuration setup.")
    );
}
