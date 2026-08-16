#!/usr/bin/env -S cargo +nightly -Zscript --quiet

---
[package]
edition = "2024"
[dependencies]
---

use std::env;
use std::fs;
use std::io::Write;
use std::path::Path;

const TEMPLATE: &str = r#"#!/usr/bin/env -S cargo +nightly -Zscript --quiet

---
[package]
edition = "2024"
[dependencies]
---

fn main() {
    println!("hello from cargo script");
}
"#;

fn usage() -> ! {
    eprintln!(
        "usage: new_cargo_script.rs <path> [--force]\n\nCreates a new cargo script with the standard boilerplate."
    );
    std::process::exit(2);
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let Some(path) = env::args().skip(1).next() else {
        usage()
    };
    let path = Path::new(&path);

    if path.exists() {
        eprintln!("error: file exists: {}", path.display());
        std::process::exit(1);
    }

    if let Some(parent) = path.parent() {
        if !parent.as_os_str().is_empty() {
            fs::create_dir_all(parent)?;
        }
    }

    let mut file = fs::File::create(path)?;
    file.write_all(TEMPLATE.as_bytes())?;
    drop(file);

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(path)?.permissions();
        perms.set_mode(0o755);
        fs::set_permissions(path, perms)?;
    }

    Ok(())
}
