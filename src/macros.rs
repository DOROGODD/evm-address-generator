#[macro_export]
macro_rules! input {
    ($text: expr) => {
        Input::with_theme(&ColorfulTheme::default())
            .with_prompt($text)
            .interact()
            .unwrap()
    };
    (Option($type: ty), $text: expr) => {
        Some(
            Input::<String>::with_theme(&ColorfulTheme::default())
                .with_prompt($text)
                .default("".to_string())
                .interact()
                .unwrap(),
        )
        .filter(|s| !s.is_empty())
        .and_then(|s| s.parse::<$type>().ok())
    };

    (Option($type: ty), $text: expr, $validator: ident) => {
        Some(
            Input::<String>::with_theme(&ColorfulTheme::default())
                .with_prompt($text)
                .default("".to_string())
                .validate_with(|input: &String| -> Result<(), &str> { $validator(input) })
                .interact()
                .unwrap(),
        )
        .filter(|s| !s.is_empty())
        .and_then(|s| s.parse::<$type>().ok())
    };
}
