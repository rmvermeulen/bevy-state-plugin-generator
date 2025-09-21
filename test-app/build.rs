use bevy_state_plugin_generator::*;
fn main() {
    let plugin_config = PluginConfig::default();
    update_template("src/auto_generated_states.rs", plugin_config)
        .expect("Failed to update template");
    generate_plugin("src/states.txt", "src/generated_states.rs", plugin_config)
        .expect("Failed to generate plugin");
}
