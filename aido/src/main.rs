mod ai;
mod config;
mod executor;
mod modes;
mod ui;

use crate::config::{binding_label, binding_to_bash, binding_to_fish, binding_to_zsh};
use anyhow::Result;
use clap::{Parser, Subcommand};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Parser)]
#[command(name = "aido")]
#[command(version = "0.1.0")]
#[command(about = "AI-powered command generation and execution for your terminal", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    /// Enable verbose logging
    #[arg(short, long, global = true)]
    verbose: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate and execute shell commands from natural language (DO mode)
    Do {
        /// The task to perform in natural language
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        prompt: Vec<String>,

        /// Skip confirmation and execute immediately
        #[arg(short = 'y', long)]
        yes: bool,

        /// Show what would be executed without running it
        #[arg(short = 'n', long)]
        dry_run: bool,
    },

    /// Ask questions and get answers (ASK mode)
    Ask {
        /// The question to ask
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        question: Vec<String>,
    },

    /// Initialize configuration
    Init,

    /// Configuration management
    Config {
        #[command(subcommand)]
        action: ConfigAction,
    },

    /// Show command history
    History {
        /// Number of recent commands to show
        #[arg(short, long, default_value = "10")]
        limit: usize,
    },

    /// Re-run a command from history
    Rerun {
        /// Command number from history
        index: usize,
    },

    /// Check configuration and dependencies
    Doctor,

    /// Generate shell integration code for keybindings
    SetupShell,

    /// Manage persistent unsafe execution mode
    Unsafe {
        #[command(subcommand)]
        action: UnsafeAction,
    },
}

#[derive(Subcommand)]
enum ConfigAction {
    /// Show current configuration
    Show,

    /// Edit configuration file
    Edit,

    /// Set a configuration value
    Set {
        /// Configuration key (e.g., ai.api_key)
        key: String,

        /// Configuration value
        value: String,
    },
}

#[derive(Subcommand)]
enum UnsafeAction {
    /// Enable always-run mode (no validation or confirmation)
    Enable,

    /// Disable always-run mode
    Disable,

    /// Show whether always-run mode is enabled
    Status,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // Initialize logging
    let log_level = if cli.verbose {
        tracing::Level::DEBUG
    } else {
        tracing::Level::INFO
    };

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| format!("aido={}", log_level).into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    match cli.command {
        Some(Commands::Do {
            prompt,
            yes,
            dry_run,
        }) => {
            let prompt_str = prompt.join(" ");
            modes::do_mode(&prompt_str, yes, dry_run).await?;
        }
        Some(Commands::Ask { question }) => {
            let question_str = question.join(" ");
            modes::ask_mode(&question_str).await?;
        }
        Some(Commands::Init) => {
            config::init_config()?;
        }
        Some(Commands::Config { action }) => {
            match action {
                ConfigAction::Show => {
                    let cfg = config::load_config()?;
                    println!("{}", serde_json::to_string_pretty(&cfg)?);
                }
                ConfigAction::Edit => {
                    let config_path = dirs::config_dir()
                        .ok_or_else(|| anyhow::anyhow!("Could not find config directory"))?
                        .join("aido")
                        .join("config.toml");

                    let editor = std::env::var("EDITOR").unwrap_or_else(|_| "vim".to_string());
                    std::process::Command::new(editor)
                        .arg(&config_path)
                        .status()?;
                }
                ConfigAction::Set { key, value } => {
                    let cfg = config::load_config()?;
                    // TODO: Implement setting nested config values
                    println!("Setting {key} = {value}");
                    config::save_config(&cfg)?;
                }
            }
        }
        Some(Commands::History { limit }) => {
            println!("Showing last {limit} commands...");
            // TODO: Implement history
        }
        Some(Commands::Rerun { index }) => {
            println!("Re-running command #{index}...");
            // TODO: Implement rerun
        }
        Some(Commands::Doctor) => {
            println!("Running diagnostics...\n");

            // Check config
            match config::load_config() {
                Ok(cfg) => {
                    println!("✓ Configuration loaded successfully");

                    // Check if claude CLI is available
                    let claude_check = std::process::Command::new(&cfg.ai.claude_command)
                        .arg("--version")
                        .output();

                    match claude_check {
                        Ok(output) if output.status.success() => {
                            println!("✓ Claude CLI found: {}", cfg.ai.claude_command);
                        }
                        _ => {
                            println!("✗ Claude CLI not found!");
                            println!(
                                "  Install Claude Code from: https://docs.claude.com/claude-code"
                            );
                        }
                    }

                    println!("✓ Model: {}", cfg.ai.model);
                }
                Err(e) => {
                    println!("✗ Configuration error: {e}");
                }
            }

            // Check shell
            let shell = std::env::var("SHELL").unwrap_or_else(|_| "unknown".to_string());
            println!("✓ Shell: {shell}");

            println!("\nAll checks complete!");
        }
        Some(Commands::SetupShell) => {
            // Detect shell and output appropriate keybinding setup
            let cfg = config::load_config()?;
            let bindings = cfg.keybindings;

            let ask_label = binding_label(&bindings.ask_binding);
            let do_label = binding_label(&bindings.do_binding);

            let shell = std::env::var("SHELL").unwrap_or_else(|_| String::from(""));

            if shell.contains("zsh") {
                let script = format!(
                    r#"# AIDO shell integration for Zsh
# {ask_label}: ASK mode - ask questions
# {do_label}: DO mode - generate and execute commands

function _aido_widget_run() {{
    local mode="$1"
    local prompt="$2"
    local input=""

    # Tell zle we're going to do I/O
    zle -I
    printf '\n'

    if read -r "input?$prompt" < /dev/tty; then
        if [[ -n "$input" ]]; then
            printf '\n'
            command aido "$mode" "$input"
        fi
    fi

    # Reset the prompt
    zle reset-prompt
}}

function aido-do-widget() {{
    _aido_widget_run "do" "🤖 AIDO DO Mode - What do you want to do? "
}}

function aido-ask-widget() {{
    _aido_widget_run "ask" "💬 AIDO ASK Mode - What's your question? "
}}

zle -N aido-do-widget
zle -N aido-ask-widget

bindkey '{ask_bind}' aido-ask-widget     # {ask_label} for ASK mode
bindkey '{do_bind}' aido-do-widget       # {do_label} for DO mode
"#,
                    ask_label = ask_label,
                    do_label = do_label,
                    ask_bind = binding_to_zsh(&bindings.ask_binding),
                    do_bind = binding_to_zsh(&bindings.do_binding)
                );

                println!("{script}");
            } else if shell.contains("bash") {
                let script = format!(
                    r#"# AIDO shell integration for Bash
# {ask_label}: ASK mode - ask questions
# {do_label}: DO mode - generate and execute commands

function aido-do-widget() {{
    echo ""
    echo "AIDO DO Mode - What do you want to do?"
    read -r input
    if [[ -n "$input" ]]; then
        aido do "$input"
    fi
}}

function aido-ask-widget() {{
    echo ""
    echo "AIDO ASK Mode - What's your question?"
    read -r input
    if [[ -n "$input" ]]; then
        aido ask "$input"
    fi
}}

bind -x '"{ask_bind}": aido-ask-widget'     # {ask_label} for ASK mode
bind -x '"{do_bind}": aido-do-widget'       # {do_label} for DO mode
"#,
                    ask_label = ask_label,
                    do_label = do_label,
                    ask_bind = binding_to_bash(&bindings.ask_binding),
                    do_bind = binding_to_bash(&bindings.do_binding)
                );

                println!("{script}");
            } else if shell.contains("fish") {
                let script = format!(
                    r#"# AIDO shell integration for Fish
# {ask_label}: ASK mode - ask questions
# {do_label}: DO mode - generate and execute commands

function aido_do_widget
    echo ""
    echo "AIDO DO Mode - What do you want to do?"
    read -l input
    if test -n "$input"
        aido do "$input"
    end
    commandline -f repaint
end

function aido_ask_widget
    echo ""
    echo "AIDO ASK Mode - What's your question?"
    read -l input
    if test -n "$input"
        aido ask "$input"
    end
    commandline -f repaint
end

bind {ask_bind} aido_ask_widget       # {ask_label} for ASK mode
bind {do_bind} aido_do_widget         # {do_label} for DO mode
"#,
                    ask_label = ask_label,
                    do_label = do_label,
                    ask_bind = binding_to_fish(&bindings.ask_binding),
                    do_bind = binding_to_fish(&bindings.do_binding)
                );

                println!("{script}");
            } else {
                eprintln!("Unable to detect shell. Supported shells: bash, zsh, fish");
                std::process::exit(1);
            }
        }
        Some(Commands::Unsafe { action }) => match action {
            UnsafeAction::Enable => {
                let mut cfg = config::load_config()?;
                if cfg.execution.unsafe_mode {
                    println!("Unsafe mode is already enabled.");
                } else {
                    cfg.execution.unsafe_mode = true;
                    config::save_config(&cfg)?;
                    println!("✓ Unsafe mode enabled. Commands will run without validation or confirmation until you disable it.");
                }
            }
            UnsafeAction::Disable => {
                let mut cfg = config::load_config()?;
                if cfg.execution.unsafe_mode {
                    cfg.execution.unsafe_mode = false;
                    config::save_config(&cfg)?;
                    println!("✓ Unsafe mode disabled. Safety checks restored.");
                } else {
                    println!("Unsafe mode is already disabled.");
                }
            }
            UnsafeAction::Status => {
                let cfg = config::load_config()?;
                if cfg.execution.unsafe_mode {
                    println!("Unsafe mode is currently ENABLED.");
                } else {
                    println!("Unsafe mode is currently disabled.");
                }
            }
        },
        None => {
            ui::settings::open_settings()?;
        }
    }

    Ok(())
}
