use time::macros::time;

fn main() {
    let _ = time!(24:00);
    let _ = time!(0:60);
    let _ = time!(0:00:60);
    let _ = time!(x);
    let _ = time!(0:00:00 x);
    let _ = time!("");
    let _ = time!(0:);
    let _ = time!(0,);
    let _ = time!(0:00:0a);
    let _ = time!(0:00 pm);
    let _ = time!(0);
    let _ = time!(0 pm);
    let _ = time!(1 am :);
}
