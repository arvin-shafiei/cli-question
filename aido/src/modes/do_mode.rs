use anyhow::Result;
use colored::Colorize;

use crate::ai::{extract_commands, ClaudeClient};
use crate::config::load_config;
use crate::executor::{confirm_execution, ConfirmAction, SafetyValidator, ShellExecutor};
use crate::ui::display::{print_error, print_info};

pub async fn do_mode(prompt: &str, skip_confirmation: bool, dry_run: bool) -> Result<()> {
    print_info(&format!("Generating command for: {}", prompt.italic()));

    // Load config
    let config = load_config()?;
    let unsafe_mode = config.execution.unsafe_mode;

    if unsafe_mode {
        println!(
            "{}",
            "⚠ UNSAFE MODE: commands will run without validation or confirmation."
                .red()
                .bold()
        );
    }

    // Create AI client
    let client = ClaudeClient::from_config(&config)?;

    // Generate command
    let response = client.generate_command(prompt).await?;

    // Extract commands from response
    let commands = extract_commands(&response);

    if commands.is_empty() {
        print_error("Could not extract any commands from AI response");
        println!("\nAI Response:");
        println!("{}", response);
        return Ok(());
    }

    // Create validator and executor
    let validator = SafetyValidator::new(config.execution.dangerous_commands.clone());
    let executor = ShellExecutor::new(dry_run || config.execution.dry_run);

    // Process commands
    for command in &commands {
        let validation = validator.validate(command);

        // Check if command is safe
        if !validation.is_safe && !unsafe_mode {
            print_error(&format!(
                "Command blocked due to safety concerns: {}",
                command
            ));
            if let Some(warning) = &validation.warning {
                println!("{}", warning.red());
            }
            continue;
        } else if !validation.is_safe && unsafe_mode {
            if let Some(warning) = &validation.warning {
                println!("\n{} {}", "⚠".red().bold(), warning.red());
            } else {
                println!(
                    "\n{} {}",
                    "⚠".red().bold(),
                    "Unsafe mode bypassing critical safety block.".red()
                );
            }
        }

        // Determine if we need confirmation
        let needs_confirmation =
            !unsafe_mode && (validation.requires_confirmation || config.execution.always_confirm);

        // Skip confirmation if -y flag is set
        let should_execute = if unsafe_mode || skip_confirmation || !needs_confirmation {
            let marker = if unsafe_mode {
                "⚠".red().bold().to_string()
            } else {
                "▶".cyan().bold().to_string()
            };
            println!("\n{} Auto-executing: {}", marker, command);
            true
        } else {
            // Ask for confirmation
            match confirm_execution(command, &validation)? {
                ConfirmAction::Execute => true,
                ConfirmAction::Cancel => {
                    print_info("Command cancelled");
                    false
                }
                ConfirmAction::Explain => {
                    // Explain the command
                    println!("\n{}", "Asking AI to explain the command...".dimmed());
                    match client.explain_command(command).await {
                        Ok(explanation) => {
                            println!("\n{}", "═".repeat(60).blue());
                            println!("{}", "Explanation".bold());
                            println!("{}", "═".repeat(60).blue());
                            println!("\n{}", explanation);
                            println!();
                        }
                        Err(e) => {
                            print_error(&format!("Failed to get explanation: {}", e));
                        }
                    }

                    // Ask again if they want to execute after explanation
                    match confirm_execution(command, &validation)? {
                        ConfirmAction::Execute => true,
                        _ => {
                            print_info("Command cancelled");
                            false
                        }
                    }
                }
                ConfirmAction::Edit => {
                    // TODO: Implement edit functionality
                    print_info("Edit functionality not yet implemented");
                    false
                }
            }
        };

        // Execute if approved
        if should_execute {
            executor.execute(command)?;
        }
    }

    Ok(())
}
