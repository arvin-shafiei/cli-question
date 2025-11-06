use crate::ai::prompts::SystemPrompts;
use crate::config::AidoConfig;
use anyhow::{Context, Result};
use std::process::{Command, Stdio};

pub struct ClaudeClient {
    model: String,
    claude_command: String,
}

impl ClaudeClient {
    pub fn from_config(config: &AidoConfig) -> Result<Self> {
        // Check if claude CLI is available
        let claude_cmd = config.ai.claude_command.clone();

        let check = Command::new(&claude_cmd)
            .arg("--version")
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status();

        if check.is_err() || !check.unwrap().success() {
            anyhow::bail!(
                "Claude CLI not found. Please install Claude Code CLI first.\n\
                Visit: https://docs.claude.com/claude-code"
            );
        }

        Ok(Self {
            model: config.ai.model.clone(),
            claude_command: claude_cmd,
        })
    }

    fn call_claude(&self, prompt: &str) -> Result<String> {
        let output = Command::new(&self.claude_command)
            .arg("-p")
            .arg(prompt)
            .arg("--model")
            .arg(&self.model)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .context("Failed to execute claude command")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("Claude command failed: {}", stderr);
        }

        let stdout =
            String::from_utf8(output.stdout).context("Claude output is not valid UTF-8")?;

        Ok(stdout.trim().to_string())
    }

    pub async fn generate_command(&self, prompt: &str) -> Result<String> {
        let context = SystemPrompts::build_context();
        let full_prompt = format!(
            "{}\n\nUser request: {}",
            SystemPrompts::do_mode(&context),
            prompt
        );

        let response = self.call_claude(&full_prompt)?;
        Ok(response)
    }

    pub async fn answer_question(&self, question: &str) -> Result<String> {
        let full_prompt = format!("{}\n\nQuestion: {}", SystemPrompts::ask_mode(), question);

        let response = self.call_claude(&full_prompt)?;
        Ok(response)
    }

    pub async fn explain_command(&self, command: &str) -> Result<String> {
        let prompt = SystemPrompts::explain_command(command);
        let response = self.call_claude(&prompt)?;
        Ok(response)
    }
}
