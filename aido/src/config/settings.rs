use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AidoConfig {
    pub ai: AiConfig,
    pub execution: ExecutionConfig,
    pub ui: UiConfig,
    #[serde(default)]
    pub keybindings: KeybindingsConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiConfig {
    pub claude_command: String,
    pub model: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionConfig {
    pub always_confirm: bool,
    pub auto_explain: bool,
    pub dangerous_commands: Vec<String>,
    pub dry_run: bool,
    #[serde(default)]
    pub unsafe_mode: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UiConfig {
    pub style: String,
    pub color_scheme: String,
    pub show_context: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeybindingsConfig {
    #[serde(default = "default_ask_binding")]
    pub ask_binding: String,
    #[serde(default = "default_do_binding")]
    pub do_binding: String,
}

pub const KNOWN_BINDINGS: &[(&str, &str)] = &[
    ("ctrl-o", "Ctrl+O"),
    ("ctrl-k", "Ctrl+K"),
    ("ctrl-alt-o", "Ctrl+Alt+O"),
    ("ctrl-alt-k", "Ctrl+Alt+K"),
];

pub fn binding_label(binding: &str) -> String {
    KNOWN_BINDINGS
        .iter()
        .find(|(key, _)| *key == binding)
        .map(|(_, label)| (*label).to_string())
        .unwrap_or_else(|| binding.to_string())
}

pub fn binding_to_zsh(binding: &str) -> String {
    match binding {
        "ctrl-o" => "^O".to_string(),
        "ctrl-k" => "^K".to_string(),
        "ctrl-alt-o" => "^[^O".to_string(),
        "ctrl-alt-k" => "^[^K".to_string(),
        other => other.to_string(),
    }
}

pub fn binding_to_bash(binding: &str) -> String {
    match binding {
        "ctrl-o" => "\\C-o".to_string(),
        "ctrl-k" => "\\C-k".to_string(),
        "ctrl-alt-o" => "\\e\\C-o".to_string(),
        "ctrl-alt-k" => "\\e\\C-k".to_string(),
        other => other.to_string(),
    }
}

pub fn binding_to_fish(binding: &str) -> String {
    match binding {
        "ctrl-o" => "\\co".to_string(),
        "ctrl-k" => "\\ck".to_string(),
        "ctrl-alt-o" => "\\e\\co".to_string(),
        "ctrl-alt-k" => "\\e\\ck".to_string(),
        other => other.to_string(),
    }
}

fn default_ask_binding() -> String {
    "ctrl-o".to_string()
}

fn default_do_binding() -> String {
    "ctrl-k".to_string()
}

impl Default for KeybindingsConfig {
    fn default() -> Self {
        Self {
            ask_binding: default_ask_binding(),
            do_binding: default_do_binding(),
        }
    }
}

impl Default for AidoConfig {
    fn default() -> Self {
        Self {
            ai: AiConfig {
                claude_command: String::from("claude"),
                model: String::from("claude-sonnet-4-20250514"),
            },
            execution: ExecutionConfig {
                always_confirm: true,
                auto_explain: false,
                dangerous_commands: vec![
                    "rm".to_string(),
                    "mv".to_string(),
                    "dd".to_string(),
                    "mkfs".to_string(),
                ],
                dry_run: false,
                unsafe_mode: false,
            },
            ui: UiConfig {
                style: "terminal".to_string(),
                color_scheme: "auto".to_string(),
                show_context: true,
            },
            keybindings: KeybindingsConfig::default(),
        }
    }
}

fn config_path() -> Result<PathBuf> {
    let config_dir = dirs::config_dir()
        .ok_or_else(|| anyhow::anyhow!("Could not find config directory"))?
        .join("aido");

    Ok(config_dir.join("config.toml"))
}

pub fn load_config() -> Result<AidoConfig> {
    let path = config_path()?;

    if !path.exists() {
        return Ok(AidoConfig::default());
    }

    let contents = fs::read_to_string(&path)
        .with_context(|| format!("Failed to read config file: {}", path.display()))?;

    let config: AidoConfig =
        toml::from_str(&contents).with_context(|| "Failed to parse config file")?;

    Ok(config)
}

pub fn save_config(config: &AidoConfig) -> Result<()> {
    let path = config_path()?;

    // Create config directory if it doesn't exist
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("Failed to create config directory: {}", parent.display()))?;
    }

    let contents = toml::to_string_pretty(config).with_context(|| "Failed to serialize config")?;

    fs::write(&path, contents)
        .with_context(|| format!("Failed to write config file: {}", path.display()))?;

    Ok(())
}

pub fn init_config() -> Result<()> {
    let path = config_path()?;

    if path.exists() {
        println!("Configuration already exists at: {}", path.display());
        print!("Do you want to overwrite it? [y/N]: ");
        use std::io::{self, Write};
        io::stdout().flush()?;

        let mut response = String::new();
        io::stdin().read_line(&mut response)?;

        if !response.trim().eq_ignore_ascii_case("y") {
            println!("Keeping existing configuration.");
            return Ok(());
        }
    }

    let config = AidoConfig::default();

    println!("\n=== AIDO Configuration Setup ===\n");
    println!("AIDO uses the Claude CLI (claude command) that you already have installed.");
    println!("No API key configuration needed!\n");

    save_config(&config)?;

    println!("✓ Configuration saved to: {}", path.display());
    println!("\nYou can:");
    println!("  - Edit config: aido config edit");
    println!("  - View config: aido config show");
    println!("  - Test setup: aido doctor");
    println!("\nYou're all set! Try:");
    println!("  aido do \"list all files\"");
    println!("  aido ask \"what does rsync do?\"");

    Ok(())
}
