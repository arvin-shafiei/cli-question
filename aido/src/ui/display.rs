use colored::Colorize;

pub fn format_output(text: &str) {
    println!("{}", text);
}

pub fn print_header(title: &str) {
    println!("\n{}", "═".repeat(60).blue());
    println!("{}", title.bold());
    println!("{}", "═".repeat(60).blue());
}

pub fn print_error(message: &str) {
    eprintln!("{} {}", "✗".red().bold(), message.red());
}

pub fn print_success(message: &str) {
    println!("{} {}", "✓".green().bold(), message.green());
}

pub fn print_info(message: &str) {
    println!("{} {}", "ℹ".blue().bold(), message);
}
