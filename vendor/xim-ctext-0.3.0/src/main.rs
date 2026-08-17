#[allow(clippy::uninlined_format_args)]
fn dump(s: &str) {
    let b = xim_ctext::utf8_to_compound_text(s);

    println!("{}", s);

    for b in b {
        print!("{:02}/{:02}({:2X}), ", b / 16, b % 16, b);
    }

    println!();
}

fn main() {
    dump("ab");
    dump("abc");
    dump("가");
    dump("가나");
    dump("가나다");
    dump("あ");
    dump("あな");
    dump("가나あな");
    dump("あな가나");
}
