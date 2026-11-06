# Contributing
Contributions, bug reports, and feature ideas are welcome. If you are working on changes, the project’s Rust tests and sample template fixtures can help validate behavior.
## First issues
If this is your first contribution you may want to look for issues with the "good first issue" label
## Running the code locally
```bash
git clone https://github.com/B3ella/groundhog.git
cd groundhog
cargo run
```
## Project Structure
- `src/main.rs` — CLI entry point
- `src/config.rs` — configuration loading and default settings
- `src/daily_note/interpreter.rs` — template token processing
- `src/daily_note/mod.rs` — daily note generation flow
- `.gitub/workflows/ci.yml` — pipeline definition
## Before opening a pull request
Before opening a pull request make sure your code passes
```bash
cargo build --verbose --locked
cargo test --verbose
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
```
## Building from source
```bash
git clone https://github.com/B3ella/groundhog.git
cd groundhog
cargo build --release
```
The built binary will be available at:
```bash
./target/release/groundhog
```
## building With Nix
```bash
nix build .#default
```
This builds the `groundhog` package using the included flake.