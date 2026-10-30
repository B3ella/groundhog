# Groundhog

Groundhog is a lightweight daily note templating system written in Rust. It generates a markdown note for the current day, optionally carries over content from the previous day, and keeps a symlink pointing to the most recent daily note.

It is inspired by obsidian daily-note plug-in and can easily be used with obsidian

## Features

- Generates a daily note named in the format `YYYY-MM-DD.md`
- Configurable template file
- Copies sections from the prior day with a template directive
- Inserts date-aware content using weekday-based substitutions
- Maintains a symlink to the current daily note for quick access

## Installation

### From source

```bash
git clone https://github.com/B3ella/groundhog.git
cd groundhog
cargo build --release
```

The built binary will be available at:

```bash
./target/release/groundhog
```

### With Nix

```bash
nix build .#default
```

This builds the `groundhog` package using the included flake.

## Configuration

Groundhog reads config from:

```bash
~/.config/groundhog
```

If the file does not exist, it falls back to these defaults:

```ini
template_path=~/Notes/templates/dnt.md
archive_path=~/Notes/daily-note/
symlink_path=~/Notes/daily-note.md
max_recursion=5
```

template_path is the path to the template file

archive_path is where the generate notes will be stored

symlink_path is the shortcut for the current daily note

for features like !copy_last_day groundhog will look for past daily notes, max recursion defines how many days groundhog will look back to before assuming a past note doesnt exist

If you use your computer everyday a small value is okay. If you use your computer less frequently (once a week for example) a bigger value may be needed so groundhog can find your last note 

You can override the config path with:

```bash
groundhog --config-path /path/to/config
```
## Template Syntax

Groundhog processes markdown templates and supports a few special tokens.

### Copy previous section

```md
# Work
!copy_last_day
```

This copies the matching section from yesterday’s note, skipping completed tasks. 
Completed task are lines that begin with:

```md
- [x]
```

### Weekday-specific content

```md
!by_weekday[Deep work;Admin;Planning;Meetings;Follow-ups;Review;Reset]
!by_weekday[Sunday;Monday;Tuesday;Wednesday;Thursday;Friday;Saturday]
```

This selects a value based on the current day of the week.

## Example Template

```md
# Daily Summary

# tasks
!copy_last_day

# Today

- Focus:
- Wins:
- Blockers:

!by_weekday[Deep work;Admin;Planning;Meetings;Follow-ups;Review;Reset]
```

When run, Groundhog will:

1. Look for the current day’s file in the archive directory
2. If it does not exist, recurse backward to find a previous note
3. Read the configured template
4. Replace special directives
5. Write the generated note
6. Update the symlink to the new daily note

## Typical Workflow

```bash
groundhog
```

Then open the symlink:

```bash
vim ~/Notes/daily-note.md
```

This will open the most recently generated daily note.

## Notes

- This project currently targets Unix-like systems.

## Project Structure

- `src/main.rs` — CLI entry point and daily note generation flow
- `src/config.rs` — configuration loading and default settings
- `src/interpreter.rs` — template token processing
- `data/default_config` — sample default config
- `test_data/` — sample template and expected output for testing


## Contributing

Contributions, bug reports, and feature ideas are welcome. If you are working on changes, the project’s Rust tests and sample template fixtures can help validate behavior.

## Author

B3ella