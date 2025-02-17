use std::process::Command;

fn main() {
    println!("cargo::rustc-check-cfg=cfg(coverage_nightly)");
    let current_hooks = Command::new("git")
        .args(["config", "--local", "core.hooksPath"])
        .output()
        .unwrap();
    let current_hooks = String::from_utf8(current_hooks.stdout).unwrap();
    if current_hooks.trim() != "hooks" {
        let hooks_ok = Command::new("git")
            .args(["config", "--local", "core.hooksPath", "hooks"])
            .status()
            .unwrap()
            .success();
        assert!(hooks_ok);
    }
}
