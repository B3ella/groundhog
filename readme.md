# Groundhog
Groundhog was built to replace the Obsidian Daily Notes plugin.

Daily notes are great but obsidian is limited to just copying the same text over and over

Groundhog can generate different text depending on the day of the week, carries forward tasks and recurring content, and integrates particularly well with NixOS
## Example
```md
!by_weekday[# Sunday;# Monday;# Tuesday;# Wednesday;# Thursday;# Friday;# Saturday]
## Plans for today
!by_weekday [short run; long run; Therapy; Guitar lessons- 16:00; short run; long run;]
## Habit tracker
- [ ] Open notes
- [ ] Clean the cats litterboxed
- [ ] take my meds
- [ ] Play the guitar
## todays MVA
!copy_tasks()
## Tasks
!copy_tasks()
## Diary
```

the note generated will depend on the day of the week

On monday for example we would have

> # Monday
> ## Plans for today
> long run
> ## Habit tracker
> - [ ] Open notes
> - [ ] Clean the cats litterboxed
> - [ ] take my meds
> - [ ] Play the guitar
> ## todays Most Valuable Action
> - [x] Take my cat to the vet
> ## Tasks
> - [ ] Fix my tv
> - [ ] schedule an apointment
> ## Diary

And then on tuesday the note would look like this

> # Tuesday
> ## Plans for today
> Therapy
> ## Habit tracker
> - [ ] Open notes
> - [ ] Clean the cats litterboxed
> - [ ] take my meds
> - [ ] Play the guitar
> ## todays Most Valuable Action
> ## Tasks
> - [ ] Fix my tv
> - [ ] schedule an apointment
> ## Diary

The !copy_tasks token ignores any completed tasks

## Features

- Generates a daily note named in the format `YYYY-MM-DD.md`
- Configurable template file
- Copies sections from the prior day with a template directive
- Inserts date-aware content using weekday-based substitutions
- Maintains a symlink to the current daily note for quick access

## Installation
To install with cargo just run the command
```bash
cargo install --git https://github.com/B3ella/groundhog
```
### With nixos
To install groundhog add the following to your nix flake
```nix

{
  description = "Nixos config";
  inputs = {
    groundhog.url = "github:B3ella/groundhog";
  };

  outputs = { self, nixpkgs, groundhog, ...}@inputs:
  let
    system = "x86_64-linux";
    username = "bella";
  in {
    nixosConfigurations.nixos = nixpkgs.lib.nixosSystem {
      inherit system;

      specialArgs = {
        inherit system;
        inherit groundhog;
      };

      modules = [
        groundhog.nixosModules.default
        {
          services.groundhog = {
            enable = true;
            user = "bella";
          };
        }
      ];
    };
  };
}
```
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

## Template Syntax
By default groundhog looks for a template on ~/Notes/templates/dnt.md

to overwrite this value read the config section

Groundhog processes markdown templates and supports a few special tokens.

### Copy previous section

```md
# Work
!copy_last_day
```

This copies the matching section from yesterday’s note
### Copy Tasks

```md
# Work
!copy_tasks
```
works just as copy last day, but ignores completed tasks

Completed taks are lines that start with:


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
## Typical Workflow
If you installed via the nix service, groundhog will run automatically everyday at 00:00
If you used cargo or other installation method you can run groundhog with the following command
```bash
groundhog
```

if you are using obsidian just turn on obsidians daily note plugin and configure the new file location to daily-note or whatever the path from your vault to your archive_dir will be

[comment]: <> (to-do add print here)

Groundhog uses the same naming convention as obsidian for its daily notes. This means every daily note funtion from obsidian will work (Open next daily note, open todays daily note and open previous daily note)

If you are not using obsidian you probably want to open the symlink in your editor of choice. in my case
```bash
nvim ~/Notes/daily-note.md
```

wich I aliased to:
```bash
notes
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
