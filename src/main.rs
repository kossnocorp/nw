use std::env;
use std::path::Path;
use std::process::{exit, Command};

fn main() {
    let args = env::args().skip(1).collect();

    if Path::new("Justfile").exists() {
        return run("just", &args);
    }

    if Path::new("Makefile").exists() {
        return run("make", &args);
    }

    if Path::new("pnpm-lock.yaml").exists() {
        return run("pnpm run", &args);
    }

    if Path::new("yarn.lock").exists() {
        return run("yarn run", &args);
    }

    if Path::new("package.json").exists() {
        return run("npm run", &args);
    }

    if Path::new("Cargo.toml").exists() {
        return run("cargo", &args);
    }

    eprintln!("No recognized file found to identify the command to run");
    exit(1);
}

fn run(command: &str, args: &Vec<String>) {
    let status = Command::new(command)
        .args(args)
        .spawn()
        .expect(&format!("Failed to start {command}"))
        .wait()
        .expect(&format!("Failed to wait on {command}"));

    exit(status.code().unwrap_or_default());
}
