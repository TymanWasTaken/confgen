mod console_utils;
mod parsing_utils;

use console_utils::console;
use parsing_utils::parsing;
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
    r#type: Option<String>, // Types: String (default), Number, Boolean
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
        Err(err) => console::err!(format!("Error parsing config file: {:?}", err))
    };

    console::info!("Starting interactive configuration setup.");
    console::info!(format!("Configuration will be saved in {}.", conf.path));

    let mut opts = HashMap::new();

    for cap in OPTION_REGEX.captures_iter(&*conf.template) {
        let opt_name = cap.get(1).unwrap().as_str();
        let opt_conf = match conf.options.iter().find(|opt| opt.id == opt_name.to_string()) {
            Some(opt) => opt,
            None => console::err!(format!("Option {} not found in configuration file.", opt_name))
        };
        let r#type = opt_conf.r#type.to_owned().unwrap_or("String".to_string());
        if !["String", "Number", "Boolean"].contains(&&*r#type) {
            console::err!(format!("Option {} has invalid type {}.", opt_name, r#type));
        }
        console::val!("Name", opt_conf.name);
        console::val!("Description", opt_conf.description);
        console::val!("Type", r#type);
        console::val!("Default", opt_conf.default.as_ref().unwrap_or(&"None".to_string()));
        let input = console::prompt!("Value", opt_conf.name, opt_conf.default.as_ref());
        let _ = match r#type.as_str() {
            "String" => opts.insert(opt_name, input),
            "Number" => {
                let parsed = parsing::number(input.as_str());
                match parsed {
                    Some(num) => opts.insert(opt_name, num.to_string()),
                    None => console::err!("Error parsing number type for option.")
                }
            },
            "Boolean" => {
                let parsed = parsing::boolean(input.as_str());
                match parsed {
                    Some(bool) => opts.insert(opt_name, bool.to_string()),
                    None => console::err!("Error parsing boolean type for option.")
                }
            }
            _ => console::err!(format!("Unknown type {} (impossible).", r#type))
        };
        println!();
    }

    let mut completed_config = conf.template.clone();
    for opt in opts {
        completed_config = completed_config.replace(format!("${{{{{}}}}}", opt.0).as_str(), &*opt.1);
    }

    let mut file = fs::File::create(conf.path).unwrap();
    file.write_all(completed_config.as_bytes()).unwrap();

    console::info!("Finished interactive configuration setup.");
}
