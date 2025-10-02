use std::io;
use std::process::Command;

fn configure_hooks_path(path: &str) -> io::Result<bool> {
    Command::new("git")
        .args(["config", "--local", "core.hooksPath", path])
        .status()
        .map(|code| code.success())
}

fn main() {
    println!("cargo::rustc-check-cfg=cfg(coverage_nightly)");
    #[cfg(debug_assertions)]
    assert!(
        configure_hooks_path("hooks").unwrap(),
        "configure_hooks_path: unexpected exit code"
    );
}
