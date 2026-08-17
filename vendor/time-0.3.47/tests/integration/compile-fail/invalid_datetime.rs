use time::macros::datetime;

fn main() {
    let _ = datetime!(2021-000 0:00);
    let _ = datetime!(2021-001 24:00);
    let _ = datetime!(2021-001 0:00 0);
    let _ = datetime!(2021-001 0:00 UTC x);
}
