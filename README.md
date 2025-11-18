# my_rust_project

Minimal Cargo workspace with a single binary crate `app`.

Run locally (use WSL2 on Windows 11):

If you're on macOS or Linux, use the commands below as normal. On Windows 11 we recommend using WSL2 to build and run the project to get a Linux-like environment that matches CI and contributors.

Option 1 — inside WSL2 (recommended):

```bash
# open your WSL2 shell (e.g. Ubuntu) and then:
cd /path/to/Rust_MC
cargo run -p app
```

Option 2 — from Windows to WSL (one-liner):

```pwsh
# run this from PowerShell to execute the command inside your default WSL distro
wsl -- cd /mnt/c/Users/<you>/github/Rust_MC && cargo run -p app
```

Run tests (inside WSL2):

```bash
cd /path/to/Rust_MC
cargo test -p app
```
