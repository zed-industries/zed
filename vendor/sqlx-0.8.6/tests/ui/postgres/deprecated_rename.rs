#[derive(sqlx::Type)]
#[sqlx(rename = "foo")]
enum Foo {
    One,
    Two,
    Three,
}

fn main() {
    compile_error!("trybuild test needs to fail for stderr checking");
}
