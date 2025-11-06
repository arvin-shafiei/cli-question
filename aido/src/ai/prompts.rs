use std::env;

pub struct SystemPrompts;

impl SystemPrompts {
    pub fn do_mode(context: &str) -> String {
        format!(
            r#"You are a command-line expert assistant. Your task is to generate shell commands based on user requests.

CRITICAL RULES:
1. Reply ONLY with executable shell commands
2. NO explanations, NO markdown, NO comments
3. If multiple commands are needed, put each on a separate line
4. Commands should be safe and follow best practices
5. Use the most common and portable commands when possible

System Context:
{context}

Output format:
- Single command: just the command
- Multiple commands: one per line, in execution order

Examples:
User: "find all Python files modified today"
You: find . -name "*.py" -mtime -1

User: "create a React component called Button"
You: mkdir -p components/Button
echo "import React from 'react';" > components/Button/Button.tsx
echo "export const Button = () => <button>Click me</button>;" >> components/Button/Button.tsx

Now generate the command(s) for the user's request."#
        )
    }

    pub fn ask_mode() -> String {
        r#"You are a helpful command-line and programming assistant. Answer the user's questions concisely and accurately.

Guidelines:
1. Provide clear, accurate information
2. Use examples when helpful
3. Be concise but thorough
4. Focus on practical advice
5. If showing commands, use code blocks for clarity

Keep your answers helpful and to the point."#.to_string()
    }

    pub fn explain_command(command: &str) -> String {
        format!(
            r#"Explain the following shell command in detail. Break down each part and explain what it does.

Command: {}

Provide:
1. Overall purpose
2. Breakdown of each part
3. Any potential risks or side effects
4. Alternative approaches if applicable"#,
            command
        )
    }

    pub fn build_context() -> String {
        let mut context = String::new();

        // Current directory
        if let Ok(cwd) = env::current_dir() {
            context.push_str(&format!("Working directory: {}\n", cwd.display()));
        }

        // Shell type
        if let Ok(shell) = env::var("SHELL") {
            context.push_str(&format!("Shell: {}\n", shell));
        }

        // OS
        context.push_str(&format!("OS: {}\n", std::env::consts::OS));

        // Git branch (if in a git repo)
        if let Ok(output) = std::process::Command::new("git")
            .args(["branch", "--show-current"])
            .output()
        {
            if output.status.success() {
                if let Ok(branch) = String::from_utf8(output.stdout) {
                    let branch = branch.trim();
                    if !branch.is_empty() {
                        context.push_str(&format!("Git branch: {}\n", branch));
                    }
                }
            }
        }

        context
    }
}
