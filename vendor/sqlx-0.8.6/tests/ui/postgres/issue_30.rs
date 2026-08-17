fn main() {
    let query = sqlx::query!("select 1 as \"'1\"");
}
