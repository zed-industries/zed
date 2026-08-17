fn main() {
    // we're probably not going to get around to the geometric types anytime soon
    let _ = sqlx::query!("select null::circle");
    let _ = sqlx::query!("select $1::circle", panic!());
}
