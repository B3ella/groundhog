use std::fs;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

pub struct Config {
    pub template_path: String,
    pub symlink_path: String,
    pub archive_path: String
}

pub fn get_config(config_path: &str) -> Config {
    let config_path_exists = fs::exists(&config_path)
        .expect("The file system is throwing an error?");

    let default_config = get_default_config();
    if !config_path_exists {
        println!(
            "no file found at {}, using default config instead",
            config_path
        );
        return default_config;
    }

    let config_map = read_config(config_path);

    let template_path = expand_tilde(config_map.get("template_path")
        .unwrap_or(&default_config.template_path));
    let symlink_path = expand_tilde(config_map.get("symlink_path")
        .unwrap_or(&default_config.symlink_path));
    let archive_path = expand_tilde(config_map.get("archive_path")
        .unwrap_or(&default_config.archive_path));

    let config = Config {template_path, symlink_path, archive_path};

    config
}

fn read_config(config_path: &str) -> HashMap<String, String> {
    let mut config = HashMap::new();
    let config_file = read_file(config_path);

    for line in config_file.lines() {
        let (key, value) = line
             .split_once('=')
             .expect("Invalid config line (missing '=')");

        config.insert(
            key.trim().to_string(),
            expand_tilde(value.trim())
        );
    };
    config
}

fn get_default_config() -> Config {
    let config = Config {
        template_path: expand_tilde("~/Notes/templates/dnt.md"),
        symlink_path: expand_tilde("~/Notes/daily-note.md"),
        archive_path: expand_tilde("~/Notes/daily-note/")
    };

    config
}

fn read_file(path: &str) -> String {
    let file = File::open(path);
    let mut contents = String::new();
    let _ = file.expect("Failed to read file {}").read_to_string(&mut contents);
    contents
} 

fn expand_tilde(string: &str) -> String {
    let home = env::var("HOME")
        .expect("Stirng contains ~ but variable HOME is not set");
    string.replace("~",&home)
}

pub fn get_config_path(args: &[String]) -> String {
    let mut config_path = "~/.config/groundhog".to_string();

    if args.len() > 1 {
        config_path = args[1].clone();
    }

    if config_path.contains("~") {
        config_path = expand_tilde(&config_path);
    }

    config_path
}


