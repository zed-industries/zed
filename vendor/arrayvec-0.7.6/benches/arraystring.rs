
extern crate arrayvec;
#[macro_use] extern crate bencher;

use arrayvec::ArrayString;

use bencher::Bencher;

fn try_push_c(b: &mut Bencher) {
    let mut v = ArrayString::<512>::new();
    b.iter(|| {
        v.clear();
        while v.try_push('c').is_ok() {
        }
        v.len()
    });
    b.bytes = v.capacity() as u64;
}

fn try_push_alpha(b: &mut Bencher) {
    let mut v = ArrayString::<512>::new();
    b.iter(|| {
        v.clear();
        while v.try_push('α').is_ok() {
        }
        v.len()
    });
    b.bytes = v.capacity() as u64;
}

// Yes, pushing a string char-by-char is slow. Use .push_str.
fn try_push_string(b: &mut Bencher) {
    let mut v = ArrayString::<512>::new();
    let input = "abcαβγ“”";
    b.iter(|| {
        v.clear();
        for ch in input.chars().cycle() {
            if !v.try_push(ch).is_ok() {
                break;
            }
        }
        v.len()
    });
    b.bytes = v.capacity() as u64;
}

fn push_c(b: &mut Bencher) {
    let mut v = ArrayString::<512>::new();
    b.iter(|| {
        v.clear();
        while !v.is_full() {
            v.push('c');
        }
        v.len()
    });
    b.bytes = v.capacity() as u64;
}

fn push_alpha(b: &mut Bencher) {
    let mut v = ArrayString::<512>::new();
    b.iter(|| {
        v.clear();
        while !v.is_full() {
            v.push('α');
        }
        v.len()
    });
    b.bytes = v.capacity() as u64;
}

fn push_string(b: &mut Bencher) {
    let mut v = ArrayString::<512>::new();
    let input = "abcαβγ“”";
    b.iter(|| {
        v.clear();
        for ch in input.chars().cycle() {
            if !v.is_full() {
                v.push(ch);
            } else {
                break;
            }
        }
        v.len()
    });
    b.bytes = v.capacity() as u64;
}

benchmark_group!(benches, try_push_c, try_push_alpha, try_push_string, push_c,
                 push_alpha, push_string);
benchmark_main!(benches);
