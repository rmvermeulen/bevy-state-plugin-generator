use bevy_state_plugin_generator::on_build_generate_plugin;

fn main() {
    println!("cargo::rustc-check-cfg=cfg(coverage_nightly)");

    on_build_generate_plugin(
        "examples/states.txt",
        "examples/generated_states.rs",
        Default::default(),
    )
    .expect("Failed to genrate plugin");
}
