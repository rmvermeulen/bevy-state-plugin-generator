fn main() {
    println!("cargo::rustc-check-cfg=cfg(coverage_nightly)");
    #[cfg(feature = "dev")]
    {
        use std::process::Command;
        assert!(
            Command::new("git")
                .args(["config", "--local", "core.hooksPath", "hooks"])
                .status()
                .map(|code| code.success())
                .unwrap(),
            "configure_hooks_path: unexpected exit code"
        );
    }
}
