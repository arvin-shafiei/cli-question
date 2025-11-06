use anyhow::Result;
use dialoguer::{theme::ColorfulTheme, Input};

pub fn prompt_user(prompt: &str) -> Result<String> {
    let input: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt)
        .interact_text()?;

    Ok(input)
}
