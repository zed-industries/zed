pub fn unexpected<E>(action: String, err: E) -> String
where
    E: std::error::Error + std::fmt::Display,
{
    format!(
        "Unexpected error {}: {}\nPlease open an issue detailing how this happened!",
        action, err
    )
}
