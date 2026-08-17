fn main() {
    let _ = sqlx::query!("select CONVERT(now(), DATE) date");

    let _ = sqlx::query!("select CONVERT(now(), TIME) time");

    let _ = sqlx::query!("select CONVERT(now(), DATETIME) datetime");
}
