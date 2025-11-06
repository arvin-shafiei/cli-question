use anyhow::Result;
use colored::Colorize;
use dialoguer::{theme::ColorfulTheme, Select};

use crate::config::{binding_label, load_config, save_config, KNOWN_BINDINGS};

pub fn open_settings() -> Result<()> {
    loop {
        let mut cfg = load_config()?;
        let unsafe_status = if cfg.execution.unsafe_mode {
            "ON".red().bold().to_string()
        } else {
            "off".green().bold().to_string()
        };

        let ask_label = binding_label(&cfg.keybindings.ask_binding);
        let do_label = binding_label(&cfg.keybindings.do_binding);

        let options = vec![
            format!("Toggle unsafe mode (currently: {unsafe_status})"),
            format!("Set ASK keybinding (currently: {ask_label})"),
            format!("Set DO keybinding (currently: {do_label})"),
            "Exit settings".to_string(),
        ];

        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("AIDO Settings")
            .items(&options)
            .default(0)
            .interact()?;

        match selection {
            0 => {
                cfg.execution.unsafe_mode = !cfg.execution.unsafe_mode;
                save_config(&cfg)?;
                let state = if cfg.execution.unsafe_mode {
                    "enabled".red().bold()
                } else {
                    "disabled".green().bold()
                };
                println!("Unsafe mode {state}.");
            }
            1 => {
                if let Some(new_binding) =
                    choose_binding("Select binding for ASK mode", &cfg.keybindings.ask_binding)?
                {
                    if new_binding != cfg.keybindings.ask_binding {
                        cfg.keybindings.ask_binding = new_binding;
                        save_config(&cfg)?;
                        println!(
                            "{}",
                            "ASK keybinding updated. Run `eval \"$(aido setup-shell)\"` or reload your shell to apply."
                                .dimmed()
                        );
                    }
                }
            }
            2 => {
                if let Some(new_binding) =
                    choose_binding("Select binding for DO mode", &cfg.keybindings.do_binding)?
                {
                    if new_binding != cfg.keybindings.do_binding {
                        cfg.keybindings.do_binding = new_binding;
                        save_config(&cfg)?;
                        println!(
                            "{}",
                            "DO keybinding updated. Run `eval \"$(aido setup-shell)\"` or reload your shell to apply."
                                .dimmed()
                        );
                    }
                }
            }
            _ => break,
        }
    }

    Ok(())
}

fn choose_binding(prompt: &str, current: &str) -> Result<Option<String>> {
    let theme = ColorfulTheme::default();

    let mut items: Vec<String> = KNOWN_BINDINGS
        .iter()
        .map(|(value, label)| {
            if *value == current {
                format!("{label} (current)")
            } else {
                label.to_string()
            }
        })
        .collect();

    let cancel_index = items.len();
    items.push("Cancel".to_string());

    let default_index = KNOWN_BINDINGS
        .iter()
        .position(|(value, _)| *value == current)
        .unwrap_or(0);

    let selection = Select::with_theme(&theme)
        .with_prompt(prompt)
        .items(&items)
        .default(default_index)
        .interact()?;

    if selection == cancel_index {
        Ok(None)
    } else {
        Ok(Some(KNOWN_BINDINGS[selection].0.to_string()))
    }
}
