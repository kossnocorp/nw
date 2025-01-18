use std::env;
use std::path::Path;
use std::process::{exit, Command};

fn main() {
    let mut args: Vec<String> = env::args().skip(1).collect();

    if Path::new("Justfile").exists() {
        return run("just", args);
    }

    if Path::new("Makefile").exists() {
        return run("make", args);
    }

    if Path::new("pnpm-lock.yaml").exists() {
        args.insert(0, "run".to_owned());
        return run("pnpm", args);
    }

    if Path::new("yarn.lock").exists() {
        args.insert(0, "run".to_owned());
        return run("yarn", args);
    }

    if Path::new("package.json").exists() {
        args.insert(0, "run".to_owned());
        return run("npm", args);
    }

    if Path::new("Cargo.toml").exists() {
        return run("cargo", args);
    }

    eprintln!("No recognized file found to identify the command to run");
    exit(1);
}

fn run(command: &str, args: Vec<String>) {
    let status = Command::new(command)
        .args(args)
        .spawn()
        .expect(&format!("Failed to start {command}"))
        .wait()
        .expect(&format!("Failed to wait on {command}"));

    exit(status.code().unwrap_or_default());
}
