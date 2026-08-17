pub trait F {
    const B: u8;
}

impl F for u16 {
    const B: u8 = 3;
}
