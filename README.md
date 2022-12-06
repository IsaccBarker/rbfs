# Rust Build-time File System

[![Crates.io](https://img.shields.io/crates/v/rbfs)](https://crates.io/crates/rbfs)
[![Docs.rs](https://img.shields.io/docsrs/rbfs)](https://docs.rs/rbfs)

RBFS is a dead simple interface for creating a virtual filesystem in memory. Once populated,
probably in your `build.rs`, RBFS generates Rust source code which is linked into your main Rust
program. During runtime, you can access any files you shoved into your RBFS code.

```
----------------                    --------------------
|   build.rs   | --- generates ---> |   resources.rs   |
----------------                    --------------------

Figure 1: Program can now access the files packed into RBFS in the build script by including the
resources.rs file.
```

It's very useful in, for example, games, where you might want to package image data, configuration
files, audio, etc...

## Usage
Include it in your `Cargo.toml` manifest. The following is a basic example.

```rust
// Create the filesystem.
let fs = FileSystem::new();

// Create a path to a virtual location in the filesystem, representing images..
let mut image_directory = fs.get_base_path();
image_directory.push("assets");
image_directory.push("imgs");

// Refering to an icon.png, which will be included via include_bytes! at preprocess time.
let icon = fs.add_file(PathBuf::from("icon.png"), &image_directory);

// Generate the Rust code.
let code = fs.code();

let resources_rs = File::create(concat!(env!("CARGO_MANIFEST_DIR"), "/src/resources.rs")).unwrap();
writeln!(resources_rs, code).unwrap();
```

## Authors
Made with 🫀 by Milo Banks.

