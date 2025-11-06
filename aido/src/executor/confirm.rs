use crate::executor::validator::{RiskLevel, ValidationResult};
use colored::Colorize;
use dialoguer::{theme::ColorfulTheme, Confirm, Select};

#[derive(Debug, Clone, PartialEq)]
pub enum ConfirmAction {
    Execute,
    Cancel,
    Edit,
    Explain,
}

pub fn confirm_execution(
    command: &str,
    validation: &ValidationResult,
) -> anyhow::Result<ConfirmAction> {
    println!("\n{}", "═".repeat(60).blue());
    println!("{}", "  AI Generated Command".bold());
    println!("{}", "═".repeat(60).blue());

    // Show the command
    println!("\n{} {}\n", "▶".cyan().bold(), command.bright_white());

    // Show warning if present
    if let Some(warning) = &validation.warning {
        let warning_prefix = match validation.risk_level {
            RiskLevel::Critical => "⛔ CRITICAL WARNING".red().bold(),
            RiskLevel::High => "⚠️  WARNING".yellow().bold(),
            RiskLevel::Medium => "ℹ️  NOTICE".yellow(),
            RiskLevel::Low => "ℹ️  INFO".blue(),
        };

        println!("{}: {}", warning_prefix, warning);
        println!();
    }

    // For critical commands, require typing DELETE
    if validation.risk_level == RiskLevel::Critical {
        println!("{}", "This command is EXTREMELY DANGEROUS!".red().bold());
        println!(
            "Type {} to confirm, or anything else to cancel:",
            "DELETE".red().bold()
        );

        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;

        if input.trim() != "DELETE" {
            return Ok(ConfirmAction::Cancel);
        }

        return Ok(ConfirmAction::Execute);
    }

    // For other commands, show options menu
    println!("{}", "═".repeat(60).blue());

    let options = vec!["Execute", "Cancel", "Explain what this does"];

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("What would you like to do?")
        .items(&options)
        .default(0)
        .interact()?;

    match selection {
        0 => Ok(ConfirmAction::Execute),
        1 => Ok(ConfirmAction::Cancel),
        2 => Ok(ConfirmAction::Explain),
        _ => Ok(ConfirmAction::Cancel),
    }
}

pub fn simple_confirm(message: &str) -> anyhow::Result<bool> {
    let confirmation = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt(message)
        .default(false)
        .interact()?;

    Ok(confirmation)
}
