use chrono::{Local, DateTime};  
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
    daily_note::create_daily_note(current_local, &config, config.max_recursion);
    daily_note::symlink_daily_note(current_local, &config);
}
