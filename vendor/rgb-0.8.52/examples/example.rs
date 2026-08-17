#[cfg(feature = "as-bytes")]
fn main() {
    use rgb::ComponentSlice;
    use rgb::prelude::*;
    use rgb::Rgb;

    let px = Rgb {
        r: 255_u8,
        g: 0,
        b: 100,
    };
    assert_eq!(rgb::bytemuck::cast_slice::<_, u8>(&[px])[0], 255);

    let bigpx = Rgb::<u16> {
        r: 65535_u16,
        g: 0,
        b: 0,
    };
    assert_eq!(bigpx.as_slice()[0], 65535);

    let px = Rgb::<u8>::new(255, 0, 255);
    let inverted: Rgb<u8> = px.map(|ch| 255 - ch);

    println!("{inverted}"); // rgb(0,255,0)
}

#[cfg(not(feature = "as-bytes"))]
fn main() {}
