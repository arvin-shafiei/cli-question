use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref CODE_BLOCK_REGEX: Regex =
        Regex::new(r"```(?:bash|sh|shell)?\s*\n([\s\S]*?)\n```").unwrap();
    static ref INLINE_CODE_REGEX: Regex = Regex::new(r"`([^`]+)`").unwrap();
}

pub struct CommandExtractor;

impl CommandExtractor {
    pub fn new() -> Self {
        Self
    }

    /// Extract commands from AI response
    pub fn extract(&self, text: &str) -> Vec<String> {
        let mut commands = Vec::new();

        // First, try to extract from code blocks
        if let Some(cmds) = self.extract_from_code_blocks(text) {
            if !cmds.is_empty() {
                return cmds;
            }
        }

        // If no code blocks found, try inline code
        if let Some(cmds) = self.extract_from_inline_code(text) {
            if !cmds.is_empty() {
                return cmds;
            }
        }

        // If still nothing, treat each non-empty line as a potential command
        for line in text.lines() {
            let line = line.trim();
            if !line.is_empty() && !line.starts_with('#') && self.looks_like_command(line) {
                commands.push(line.to_string());
            }
        }

        commands
    }

    fn extract_from_code_blocks(&self, text: &str) -> Option<Vec<String>> {
        let mut commands = Vec::new();

        for cap in CODE_BLOCK_REGEX.captures_iter(text) {
            if let Some(code) = cap.get(1) {
                let code = code.as_str();
                for line in code.lines() {
                    let line = line.trim();
                    if !line.is_empty() && !line.starts_with('#') {
                        commands.push(line.to_string());
                    }
                }
            }
        }

        if commands.is_empty() {
            None
        } else {
            Some(commands)
        }
    }

    fn extract_from_inline_code(&self, text: &str) -> Option<Vec<String>> {
        let mut commands = Vec::new();

        for cap in INLINE_CODE_REGEX.captures_iter(text) {
            if let Some(code) = cap.get(1) {
                let code = code.as_str().trim();
                if self.looks_like_command(code) {
                    commands.push(code.to_string());
                }
            }
        }

        if commands.is_empty() {
            None
        } else {
            Some(commands)
        }
    }

    fn looks_like_command(&self, text: &str) -> bool {
        // Common command patterns
        let common_commands = [
            "ls", "cd", "pwd", "mkdir", "rm", "cp", "mv", "cat", "echo", "grep", "find", "sed",
            "awk", "sort", "head", "tail", "git", "npm", "cargo", "python", "node", "docker",
            "curl", "wget", "ssh", "scp", "rsync", "tar", "zip", "unzip", "chmod", "chown", "ps",
            "kill", "top", "df", "du", "touch", "vim", "nano", "less", "more",
        ];

        let first_word = text.split_whitespace().next().unwrap_or("");

        // Check if it starts with a common command
        common_commands.iter().any(|&cmd| first_word == cmd || first_word.starts_with(cmd))
            // Or contains shell operators
            || text.contains("|")
            || text.contains("&&")
            || text.contains("||")
            || text.contains(">")
            || text.contains("<")
            // Or looks like a path or flag
            || first_word.starts_with("./")
            || first_word.starts_with("../")
            || first_word.starts_with('-')
    }
}

/// Convenience function to extract commands
pub fn extract_commands(text: &str) -> Vec<String> {
    let extractor = CommandExtractor::new();
    extractor.extract(text)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_from_code_block() {
        let text = r#"
Here's how to do it:

```bash
ls -la
pwd
```
        "#;

        let commands = extract_commands(text);
        assert_eq!(commands, vec!["ls -la", "pwd"]);
    }

    #[test]
    fn test_extract_from_inline() {
        let text = "You can use `ls -la` to list files";
        let commands = extract_commands(text);
        assert_eq!(commands, vec!["ls -la"]);
    }

    #[test]
    fn test_extract_plain_commands() {
        let text = "find . -name '*.rs'\ngrep -r 'TODO' .";
        let commands = extract_commands(text);
        assert_eq!(commands, vec!["find . -name '*.rs'", "grep -r 'TODO' ."]);
    }
}
