use tiny_skia::*;

#[test]
fn decode_grayscale() {
    let pixmap = Pixmap::load_png("tests/images/pngs/grayscale.png").unwrap();
    assert_eq!(pixmap.pixel(10, 10).unwrap(), ColorU8::from_rgba(255, 255, 255, 255).premultiply());
    assert_eq!(pixmap.pixel(50, 50).unwrap(), ColorU8::from_rgba(0, 0, 0, 255).premultiply());
}

#[test]
fn decode_grayscale_alpha() {
    let pixmap = Pixmap::load_png("tests/images/pngs/grayscale-alpha.png").unwrap();
    assert_eq!(pixmap.pixel(10, 10).unwrap(), ColorU8::from_rgba(0, 0, 0, 0).premultiply());
    assert_eq!(pixmap.pixel(50, 50).unwrap(), ColorU8::from_rgba(0, 0, 0, 255).premultiply());
}

#[test]
fn decode_rgb() {
    let pixmap = Pixmap::load_png("tests/images/pngs/rgb.png").unwrap();
    assert_eq!(pixmap.pixel(10, 10).unwrap(), ColorU8::from_rgba(255, 255, 255, 255).premultiply());
    assert_eq!(pixmap.pixel(50, 50).unwrap(), ColorU8::from_rgba(36, 191, 49, 255).premultiply());
}

#[test]
fn decode_rgba() {
    let pixmap = Pixmap::load_png("tests/images/pngs/rgba.png").unwrap();
    assert_eq!(pixmap.pixel(10, 10).unwrap(), ColorU8::from_rgba(0, 0, 0, 0).premultiply());
    assert_eq!(pixmap.pixel(25, 25).unwrap(), ColorU8::from_rgba(161, 227, 165, 108).premultiply());
    assert_eq!(pixmap.pixel(50, 50).unwrap(), ColorU8::from_rgba(33, 190, 47, 252).premultiply());
}

// TODO: test encoding, somehow
