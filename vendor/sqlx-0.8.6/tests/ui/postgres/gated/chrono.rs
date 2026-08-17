fn main() {
    let _ = sqlx::query!("select now()::date");

    let _ = sqlx::query!("select now()::time");

    let _ = sqlx::query!("select now()::timestamp");

    let _ = sqlx::query!("select now()::timestamptz");

    let _ = sqlx::query!("select $1::date", ());

    let _ = sqlx::query!("select $1::time", ());

    let _ = sqlx::query!("select $1::timestamp", ());

    let _ = sqlx::query!("select $1::timestamptz", ());
}
