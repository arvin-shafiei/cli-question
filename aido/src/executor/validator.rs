use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref DANGEROUS_PATTERNS: Vec<Regex> = vec![
        // rm -rf with dangerous paths
        Regex::new(r"rm\s+(-[rf]{1,2}\s+|--recursive\s+|--force\s+).*(/$|/\*|/home|/Users|~)").unwrap(),
        // dd to disk devices
        Regex::new(r"dd\s+.*of=/dev/[hs]d[a-z]").unwrap(),
        // Format filesystem
        Regex::new(r"mkfs\.").unwrap(),
        // Fork bomb
        Regex::new(r":\(\)\s*\{.*;\};").unwrap(),
        // Pipe to shell from web
        Regex::new(r"(curl|wget).*\|\s*(bash|sh|zsh|fish)").unwrap(),
        // Write directly to disk
        Regex::new(r">\s*/dev/[hs]d[a-z]").unwrap(),
        // chmod 777 recursively
        Regex::new(r"chmod\s+-R\s+777").unwrap(),
    ];

    static ref REQUIRES_CONFIRMATION_PATTERNS: Vec<Regex> = vec![
        Regex::new(r"^sudo\s+").unwrap(),
        Regex::new(r"^rm\s+").unwrap(),
        Regex::new(r"^mv\s+.*\s+/").unwrap(),
        Regex::new(r"git\s+push.*--force").unwrap(),
        Regex::new(r"git\s+reset.*--hard").unwrap(),
        Regex::new(r"^chmod\s+").unwrap(),
        Regex::new(r"^chown\s+").unwrap(),
        Regex::new(r"^kill\s+").unwrap(),
        Regex::new(r"^pkill\s+").unwrap(),
    ];
}

#[derive(Debug, Clone, PartialEq)]
pub enum RiskLevel {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone)]
pub struct ValidationResult {
    pub is_safe: bool,
    pub risk_level: RiskLevel,
    pub warning: Option<String>,
    pub requires_confirmation: bool,
}

pub struct SafetyValidator {
    dangerous_commands: Vec<String>,
}

impl SafetyValidator {
    pub fn new(dangerous_commands: Vec<String>) -> Self {
        Self { dangerous_commands }
    }

    pub fn validate(&self, command: &str) -> ValidationResult {
        // Check for critically dangerous patterns
        for pattern in DANGEROUS_PATTERNS.iter() {
            if pattern.is_match(command) {
                return ValidationResult {
                    is_safe: false,
                    risk_level: RiskLevel::Critical,
                    warning: Some(self.get_critical_warning(command)),
                    requires_confirmation: true,
                };
            }
        }

        // Check for commands that require confirmation
        let needs_confirmation = REQUIRES_CONFIRMATION_PATTERNS
            .iter()
            .any(|pattern| pattern.is_match(command));

        // Check for user-configured dangerous commands
        let first_word = command.split_whitespace().next().unwrap_or("");
        let is_in_dangerous_list = self.dangerous_commands.iter().any(|cmd| first_word == cmd);

        if needs_confirmation || is_in_dangerous_list {
            return ValidationResult {
                is_safe: true,
                risk_level: if first_word == "sudo" {
                    RiskLevel::High
                } else if first_word == "rm" {
                    RiskLevel::High
                } else {
                    RiskLevel::Medium
                },
                warning: Some(self.get_warning(command)),
                requires_confirmation: true,
            };
        }

        // Safe command
        ValidationResult {
            is_safe: true,
            risk_level: RiskLevel::Low,
            warning: None,
            requires_confirmation: false,
        }
    }

    fn get_critical_warning(&self, command: &str) -> String {
        if command.contains("rm") && command.contains("-rf") {
            "This command will PERMANENTLY DELETE files/directories recursively!".to_string()
        } else if command.contains("dd") {
            "This command writes directly to a disk device - can destroy data!".to_string()
        } else if command.contains("mkfs") {
            "This command will FORMAT a filesystem - all data will be lost!".to_string()
        } else if command.contains("curl") || command.contains("wget") {
            "This command downloads and executes code from the internet!".to_string()
        } else {
            "This is a potentially dangerous command!".to_string()
        }
    }

    fn get_warning(&self, command: &str) -> String {
        let first_word = command.split_whitespace().next().unwrap_or("");

        match first_word {
            "sudo" => "This command will run with superuser privileges".to_string(),
            "rm" => "This command will delete files".to_string(),
            "mv" => "This command will move/rename files".to_string(),
            "chmod" => "This command will change file permissions".to_string(),
            "chown" => "This command will change file ownership".to_string(),
            "kill" | "pkill" => "This command will terminate processes".to_string(),
            _ if command.contains("git push --force") => {
                "This will force push to git remote - may overwrite others' work".to_string()
            }
            _ if command.contains("git reset --hard") => {
                "This will discard all local changes permanently".to_string()
            }
            _ => format!("This command may modify system state: {}", first_word),
        }
    }
}

impl Default for SafetyValidator {
    fn default() -> Self {
        Self::new(vec![
            "rm".to_string(),
            "mv".to_string(),
            "dd".to_string(),
            "mkfs".to_string(),
        ])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dangerous_command() {
        let validator = SafetyValidator::default();

        let result = validator.validate("rm -rf /");
        assert!(!result.is_safe);
        assert_eq!(result.risk_level, RiskLevel::Critical);
    }

    #[test]
    fn test_requires_confirmation() {
        let validator = SafetyValidator::default();

        let result = validator.validate("sudo apt install foo");
        assert!(result.is_safe);
        assert!(result.requires_confirmation);
        assert_eq!(result.risk_level, RiskLevel::High);
    }

    #[test]
    fn test_safe_command() {
        let validator = SafetyValidator::default();

        let result = validator.validate("ls -la");
        assert!(result.is_safe);
        assert!(!result.requires_confirmation);
        assert_eq!(result.risk_level, RiskLevel::Low);
    }
}
