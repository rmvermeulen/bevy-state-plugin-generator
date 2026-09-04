use bevy_state_plugin_generator::prelude::*;

fn main() {
    let plugin_config = PluginConfig {
        naming_scheme: NamingScheme::Short,
        ..Default::default()
    };

    update_template("src/auto_generated_states.rs", plugin_config)
        .expect("Failed to update template");

    for naming_scheme in [NamingScheme::None, NamingScheme::Short, NamingScheme::Full] {
        let output_path = format!("src/generated_states_{}.rs", naming_scheme.tag());
        generate_plugin("src/states.txt", output_path, naming_scheme.into())
            .expect("Failed to generate plugin: {output_path}");
    }
}
