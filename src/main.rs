use chrono::{DateTime, Local};
use clap::Parser;
mod config;
mod daily_note;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = "~/.config/groundhog".to_string())]
    config_path: String,
}

fn main() {
    let args = Args::parse();

    let config = config::get_config(&args.config_path);

    let current_local: DateTime<Local> = Local::now();
    daily_note::create_daily_note(current_local, &config);
    daily_note::symlink_daily_note(current_local, &config);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_path_defaults_to_the_documented_location() {
        let args = Args::try_parse_from(["groundhog"]).unwrap();
        assert_eq!(args.config_path, "~/.config/groundhog");
    }

    #[test]
    fn config_path_can_be_overridden_with_long_option() {
        let args =
            Args::try_parse_from(["groundhog", "--config-path", "/tmp/custom-config"]).unwrap();
        assert_eq!(args.config_path, "/tmp/custom-config");
    }

    #[test]
    fn config_path_can_be_overridden_with_short_option() {
        let args = Args::try_parse_from(["groundhog", "-c", "/tmp/short-config"]).unwrap();
        assert_eq!(args.config_path, "/tmp/short-config");
    }
}
