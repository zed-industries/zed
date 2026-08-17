use rgb::*;

// Run using: cargo run --features=serde --example serde

fn main() {
    let color = Rgb { r:255_u8, g:0, b:100 };
    println!("{}", serde_json::to_string(&color).unwrap());

    let color: Rgb<u8> = serde_json::from_str("{\"r\":10,\"g\":20,\"b\":30}").unwrap();
    println!("{}", color);
}
