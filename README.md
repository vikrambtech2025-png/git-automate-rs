# git-automate-rs

A small Rust CLI that automates the boring part of Git: **stage → commit → push**.

Commit messages are generated automatically with the [`names`](https://crates.io/crates/names) crate, so every commit gets a memorable name like `courageous-birthday` instead of `update`.

## Features

- Stages all changes with `git add -A`
- Generates a random human-readable commit message
- Detects the current branch and pushes to `origin` with upstream tracking
- Skips safely when there is nothing to commit
- Reports a clear error on detached `HEAD`

## Requirements

- Rust 1.85+ (edition 2024) and Cargo — install from [rustup.rs](https://rustup.rs)
- Git on your `PATH`
- Run the binary **from inside a Git repository**

## Build

```powershell
git clone https://github.com/vikrambtech2025-png/git-automate-rs.git
cd git-automate-rs
cargo build --release
```

The binary is written to `target/release/git-automate-rs.exe` on Windows.

## Usage

```powershell
cd path\to\your\git\repo
cargo run --manifest-path path\to\git-automate-rs\Cargo.toml
```

Or after building, run the binary from the repository you want to commit:

```powershell
& path\to\git-automate-rs.exe
```

Example output:

```text
[master f3e3a7f] decisive-dogs
 1 file changed, 1 insertion(+)
 * [new branch]      master -> master
Successfully added, committed and pushed changes!
Commit message: decisive-dogs
```

## Behaviour

| Situation | Result |
|---|---|
| Clean working tree | Prints `Nothing to commit - working tree clean.` and exits 0 |
| Changes, no `origin` remote | Commits locally, prints `No 'origin' remote configured - skipped push.` and exits 0 |
| Changes, `origin` configured | Commits and runs `git push -u origin <branch>` |
| Detached `HEAD` | Prints an error and exits 1 before pushing |

> **Note:** `git add -A` stages **everything** that is not ignored. Keep sensitive files out of the repository with `.gitignore`.

## Development

```powershell
cargo fmt --all -- --check
cargo check --locked
cargo clippy --all-targets -- -D warnings
cargo test --locked
```

These same commands run in GitHub Actions on every push and pull request.

## License

MIT — see [LICENSE](LICENSE).
