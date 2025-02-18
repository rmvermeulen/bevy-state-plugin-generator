use bevy_state_plugin_generator::*;
fn main() {
    on_build_generate_plugin(
        "src/states.txt",
        "src/generated_states.rs",
        PluginConfig::default(),
    )
    .expect("Failed to generate plugin");
}
