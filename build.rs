fn main() {
    println!("cargo::rustc-check-cfg=cfg(coverage_nightly)");

    #[cfg(feature = "dogfood")]
    bevy_state_plugin_generator::on_build_generate_plugin(
        "examples/states.txt",
        "examples/generated_states.rs",
        Default::default(),
    )
    .expect("Failed to generate plugin");
}
