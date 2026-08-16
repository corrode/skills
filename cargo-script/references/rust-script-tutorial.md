# Rust Script Mini-Tutorial

## Paths and file IO

Use `Path` and `PathBuf` for filesystem paths and avoid manual string concatenation.

```rust
use std::fs;
use std::path::{Path, PathBuf};

let root = Path::new("/tmp");
let file = root.join("example.txt");

fs::create_dir_all(root)?;
fs::write(&file, "hello\n")?;
let contents = fs::read_to_string(&file)?;
println!("{contents}");
```

## Walk directories

Use `fs::read_dir` for shallow scans.

```rust
use std::fs;
use std::path::Path;

for entry in fs::read_dir(Path::new("."))? {
    let entry = entry?;
    println!("{}", entry.path().display());
}
```

Use `walkdir` for recursive traversal.

```rust
use walkdir::WalkDir;

for entry in WalkDir::new(".").into_iter().filter_map(Result::ok) {
    println!("{}", entry.path().display());
}
```

Add to the script frontmatter when needed:

```toml
[dependencies]
walkdir = "2"
```

## Invoke other programs

Use `std::process::Command` for subprocesses.

```rust
use std::process::Command;

let status = Command::new("git")
    .arg("status")
    .status()?;

if !status.success() {
    eprintln!("git status failed");
}
```

Capture output when you need stdout/stderr:

```rust
use std::process::Command;

let output = Command::new("rg").arg("TODO").output()?;
let stdout = String::from_utf8_lossy(&output.stdout);
let stderr = String::from_utf8_lossy(&output.stderr);
println!("{stdout}");
if !output.status.success() {
    eprintln!("{stderr}");
}
```

## Parallelize work

For small scripts, start with `std::thread` and join handles. Use `thread::scope` when you want threads to borrow data without `Arc`.

```rust
use std::thread;

let handles: Vec<_> = (0..4)
    .map(|i| thread::spawn(move || i * 2))
    .collect();

for h in handles {
    println!("{}", h.join().unwrap());
}
```

For data-parallel work, prefer `rayon` to avoid manual thread management.

```rust
use rayon::prelude::*;

let nums = vec![1, 2, 3, 4, 5];
let squares: Vec<_> = nums.par_iter().map(|n| n * n).collect();
println!("{squares:?}");
```

Add to the script frontmatter when needed:

```toml
[dependencies]
rayon = "1"
```

## Simple CLI interfaces

Use a quick `match` on `std::env::args()` for trivial CLIs.

```rust
use std::env;

let mut args = env::args().skip(1);
match args.next().as_deref() {
    Some("init") => println!("init"),
    Some("run") => println!("run"),
    _ => eprintln!("usage: script <init|run>"),
}
```

Use `clap` when you need flags, subcommands, defaults, or `--help` output.

```rust
use clap::{Parser, Subcommand};

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Init { path: String },
    Run,
}

let cli = Cli::parse();
```

Add to the script frontmatter when needed:

```toml
[dependencies]
clap = { version = "4", features = ["derive"] }
```

For interactive prompts (confirm, select, input), use `cliclack`.

```rust
use cliclack::{confirm, input};

let name: String = input("Project name")?.interact()?;
let ok = confirm("Continue?")?.interact()?;
println!("{name}, ok={ok}");
```

Add to the script frontmatter when needed:

```toml
[dependencies]
cliclack = "0.1"
```
