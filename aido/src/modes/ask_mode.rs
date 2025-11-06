use anyhow::Result;
use colored::Colorize;

use crate::ai::ClaudeClient;
use crate::config::load_config;
use crate::ui::display::print_info;

pub async fn ask_mode(question: &str) -> Result<()> {
    print_info(&format!("Asking: {}", question.italic()));

    // Load config
    let config = load_config()?;

    // Create AI client
    let client = ClaudeClient::from_config(&config)?;

    // Get answer
    let answer = client.answer_question(question).await?;

    // Display answer
    println!("\n{}", "═".repeat(60).blue());
    println!("{}", "Answer".bold());
    println!("{}", "═".repeat(60).blue());
    println!("\n{}", answer);
    println!();

    Ok(())
}
