use anyhow::{Context, Result};
use colored::Colorize;
use std::process::{Command, Output, Stdio};

#[derive(Debug)]
pub struct ExecutionResult {
    pub stdout: String,
    pub stderr: String,
    pub exit_code: i32,
    pub success: bool,
}

pub struct ShellExecutor {
    shell: String,
    dry_run: bool,
}

impl ShellExecutor {
    pub fn new(dry_run: bool) -> Self {
        let shell = std::env::var("SHELL").unwrap_or_else(|_| "/bin/sh".to_string());

        Self { shell, dry_run }
    }

    pub fn execute(&self, command: &str) -> Result<ExecutionResult> {
        if self.dry_run {
            println!("{} {}", "[DRY RUN]".yellow().bold(), command);
            return Ok(ExecutionResult {
                stdout: String::new(),
                stderr: String::new(),
                exit_code: 0,
                success: true,
            });
        }

        println!("{} {}", "▶".cyan().bold(), command.bright_white());

        let output = Command::new(&self.shell)
            .arg("-c")
            .arg(command)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .with_context(|| format!("Failed to execute command: {}", command))?;

        let result = self.process_output(output);

        // Print output
        if !result.stdout.is_empty() {
            print!("{}", result.stdout);
        }

        if !result.stderr.is_empty() {
            eprint!("{}", result.stderr.red());
        }

        if result.success {
            println!("{}", "✓ Command completed successfully".green());
        } else {
            println!(
                "{} Exit code: {}",
                "✗ Command failed".red().bold(),
                result.exit_code
            );
        }

        Ok(result)
    }

    pub fn execute_sequence(&self, commands: Vec<String>) -> Result<Vec<ExecutionResult>> {
        let mut results = Vec::new();

        for (i, command) in commands.iter().enumerate() {
            if commands.len() > 1 {
                println!(
                    "\n{} Command {} of {}",
                    "━━━".blue().bold(),
                    i + 1,
                    commands.len()
                );
            }

            match self.execute(command) {
                Ok(result) => {
                    let success = result.success;
                    results.push(result);

                    if !success {
                        println!(
                            "\n{} Stopping execution due to failure",
                            "⚠".yellow().bold()
                        );
                        break;
                    }
                }
                Err(e) => {
                    println!("\n{} Error: {}", "✗".red().bold(), e);
                    return Err(e);
                }
            }
        }

        Ok(results)
    }

    fn process_output(&self, output: Output) -> ExecutionResult {
        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        let exit_code = output.status.code().unwrap_or(-1);
        let success = output.status.success();

        ExecutionResult {
            stdout,
            stderr,
            exit_code,
            success,
        }
    }
}

impl Default for ShellExecutor {
    fn default() -> Self {
        Self::new(false)
    }
}
