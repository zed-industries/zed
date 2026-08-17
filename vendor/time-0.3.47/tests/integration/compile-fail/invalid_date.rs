use time::macros::date;

fn main() {
    let _ = date!(+1_000_000-01-01);
    let _ = date!(10_000-01-01);
    let _ = date!(2021-W 60-1);
    let _ = date!(2021-W60-1);
    let _ = date!(2021-W 01-0);
    let _ = date!(2021-W01-0);
    let _ = date!(2021-W 01-8);
    let _ = date!(2021-W01-8);
    let _ = date!(2021-00-01);
    let _ = date!(2021-13-01);
    let _ = date!(2021-01-00);
    let _ = date!(2021-01-32);
    let _ = date!(2021-000);
    let _ = date!(2021-366);
    let _ = date!(0a);
    let _ = date!(2021:);
    let _ = date!(2021-W 0a);
    let _ = date!(2021-W0a);
    let _ = date!(2021-W 01:);
    let _ = date!(2021-W01:);
    let _ = date!(2021-W 01-0a);
    let _ = date!(2021-W01-0a);
    let _ = date!(2021-0a);
    let _ = date!(2021-01-0a);
}
