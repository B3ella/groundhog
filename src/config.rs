use std::collections::HashMap;
use std::env;
use std::fs;
use std::fs::File;
use std::io::prelude::*;

pub struct Config {
    pub template_path: String,
    pub symlink_path: String,
    pub archive_path: String,
}

pub fn get_config(config_path: &str) -> Config {
    let config_path = expand_tilde(config_path);
    let config_path_exists =
        fs::exists(&config_path).expect("The file system is throwing an error?");

    let default_config = get_default_config();
    if !config_path_exists {
        println!(
            "no file found at {}, using default config instead",
            config_path
        );
        return default_config;
    }

    let config_map = read_config(&config_path);

    let template_path = expand_tilde(
        config_map
            .get("template_path")
            .unwrap_or(&default_config.template_path),
    );
    let symlink_path = expand_tilde(
        config_map
            .get("symlink_path")
            .unwrap_or(&default_config.symlink_path),
    );
    let mut archive_path = expand_tilde(
        config_map
            .get("archive_path")
            .unwrap_or(&default_config.archive_path),
    );
    if !archive_path.ends_with("/") {
        archive_path = archive_path + "/";
    }

    Config {
        template_path,
        symlink_path,
        archive_path,
    }
}

fn read_config(config_path: &str) -> HashMap<String, String> {
    let mut config = HashMap::new();
    let config_file = read_file(config_path);

    for line in config_file.lines() {
        let (key, value) = line
            .split_once('=')
            .expect("Invalid config line (missing '=')");

        config.insert(key.trim().to_string(), expand_tilde(value.trim()));
    }
    config
}

fn get_default_config() -> Config {
    Config {
        template_path: expand_tilde("~/Notes/templates/dnt.md"),
        symlink_path: expand_tilde("~/Notes/daily-note.md"),
        archive_path: expand_tilde("~/Notes/daily-note/"),
    }
}

fn read_file(path: &str) -> String {
    let file = File::open(path);
    let mut contents = String::new();
    let _ = file
        .expect("Failed to read file {}")
        .read_to_string(&mut contents);
    contents
}

fn expand_tilde(string: &str) -> String {
    let home = env::var("HOME").expect("Stirng contains ~ but variable HOME is not set");
    string.replace("~", &home)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn temp_path(label: &str) -> String {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        env::temp_dir()
            .join(format!(
                "groundhog-config-{label}-{}-{nonce}",
                std::process::id()
            ))
            .to_string_lossy()
            .into_owned()
    }

    #[test]
    fn missing_config_uses_defaults() {
        let config = get_config(&temp_path("missing"));
        let home = env::var("HOME").unwrap();

        assert_eq!(
            config.template_path,
            format!("{home}/Notes/templates/dnt.md")
        );
        assert_eq!(config.symlink_path, format!("{home}/Notes/daily-note.md"));
        assert_eq!(config.archive_path, format!("{home}/Notes/daily-note/"));
    }

    #[test]
    fn reads_all_config_values_and_trims_whitespace() {
        let path = temp_path("complete");
        fs::write(
            &path,
            " template_path = /tmp/template.md \nsymlink_path=/tmp/today.md\n archive_path = /tmp/archive/ \n",
        )
        .unwrap();

        let config = get_config(&path);
        assert_eq!(config.template_path, "/tmp/template.md");
        assert_eq!(config.symlink_path, "/tmp/today.md");
        assert_eq!(config.archive_path, "/tmp/archive/");
        fs::remove_file(path).unwrap();
    }

    #[test]
    fn omitted_values_fall_back_to_defaults() {
        let path = temp_path("partial");
        fs::write(&path, "template_path=/tmp/custom-template.md\n").unwrap();

        let config = get_config(&path);
        let home = env::var("HOME").unwrap();
        assert_eq!(config.template_path, "/tmp/custom-template.md");
        assert_eq!(config.symlink_path, format!("{home}/Notes/daily-note.md"));
        assert_eq!(config.archive_path, format!("{home}/Notes/daily-note/"));
        fs::remove_file(path).unwrap();
    }

    #[test]
    fn works_with_or_without_trailling_separators() {
        let path1 = temp_path("partial");
        fs::write(&path1, "archive_path=~/Notes/daily-note/").unwrap();
        let config1 = get_config(&path1);

        let path2 = temp_path("partial");
        fs::write(&path2, "archive_path=~/Notes/daily-note").unwrap();
        let config2 = get_config(&path2);

        assert_eq!(config1.archive_path, config2.archive_path);
    }
}
