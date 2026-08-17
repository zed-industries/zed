use time::macros::utc_datetime;

fn main() {
    let _ = utc_datetime!(2021-000 0:00);
    let _ = utc_datetime!(2021-001 24:00);
    let _ = utc_datetime!(2021-001 0:00 0);
    let _ = utc_datetime!(2021-001 0:00 UTC);
    let _ = utc_datetime!(2021-001 0:00 UTC x);
}
