use time::macros::offset;

fn main() {
    let _ = offset!(+26);
    let _ = offset!(+0:60);
    let _ = offset!(+0:00:60);
    let _ = offset!(0);
    let _ = offset!();
    let _ = offset!(+0a);
    let _ = offset!(+0:0a);
    let _ = offset!(+0:00:0a);
}
