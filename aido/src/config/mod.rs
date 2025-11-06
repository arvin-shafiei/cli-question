pub mod settings;

pub use settings::{
    binding_label, binding_to_bash, binding_to_fish, binding_to_zsh, init_config, load_config,
    save_config, AidoConfig, KNOWN_BINDINGS,
};
