use time::macros::format_description;

fn main() {
    let _ = format_description!();
    let _ = format_description!("[]");
    let _ = format_description!("[foo]");
    let _ = format_description!("[");
    let _ = format_description!("[hour foo]");
    let _ = format_description!("" x);
    let _ = format_description!(x);
    let _ = format_description!(0);
    let _ = format_description!({});

    let _ = format_description!("[ invalid ]");
    let _ = format_description!("[");
    let _ = format_description!("[ ");
    let _ = format_description!("[]");
    let _ = format_description!("[day sign:mandatory]");
    let _ = format_description!("[day sign:]");
    let _ = format_description!("[day :mandatory]");
    let _ = format_description!("[day sign:mandatory");
    let _ = format_description!("[day padding:invalid]");

    let _ = format_description!(version);
    let _ = format_description!(version "");
    let _ = format_description!(version =);
    let _ = format_description!(version = 0);
    let _ = format_description!(version = 1);
    let _ = format_description!(version = 3);
    let _ = format_description!(version = two);

    let _ = format_description!(version = 2, r"\a");
    let _ = format_description!(version = 2, r"\");

    let _ = format_description!(version = 2, "[year [month]]");
    let _ = format_description!(version = 2, "[optional[]]");
    let _ = format_description!(version = 2, "[first[]]");
    let _ = format_description!(version = 2, "[optional []");
    let _ = format_description!(version = 2, "[first []");
    let _ = format_description!(version = 2, "[optional [");
    let _ = format_description!(version = 2, "[optional [[year");
    let _ = format_description!(version = 2, "[optional ");

    let _ = format_description!("[ignore]");
    let _ = format_description!("[ignore count:0]");
}
