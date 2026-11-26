# Rust Learning
A simple repository for learning Rust through small programs and exercises.

## Prerequisites
Install Rust with [rustup](https://rustup.rs/). This also installs `rustc` and Cargo.

Check that the tools are available:

```bash
rustc --version
cargo --version
```
## Create Project
Start initialized Cargo to Create project from selected folder with configuration file.
```bash
cargo init
```
`cargo init --bin` command to create executeable project aplication.

`cargo init --lib` command to create library project for use in other project.

OR Create new project folder iclude with configuration file.
```bash
cargo new <project name>
```

## Run Project
After creating or initializing the project, cargo will create file `src/main.rs`. 

You can the run project with cargo command:
```bash
cargo run
```

or manually compile the main scripts in `src` folder using `rustc` command and run the result script.
```bash
cd src
rustc main.rs
./main # or .\main on Windows
