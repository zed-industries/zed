pub mod color {
    use gpui::color::Color;

    pub fn background(elevation: f32) -> Color {
        todo!()
    }
}

pub mod text {
    use gpui::elements::node::length::{rems, Rems};

    pub fn xs() -> Rems {
        rems(0.75)
    }

    pub fn sm() -> Rems {
        rems(0.875)
    }

    pub fn base() -> Rems {
        rems(1.0)
    }

    pub fn lg() -> Rems {
        rems(1.125)
    }

    pub fn xl() -> Rems {
        rems(1.25)
    }

    pub fn xxl() -> Rems {
        rems(1.5)
    }

    pub fn xxxl() -> Rems {
        rems(1.875)
    }

    pub fn x4l() -> Rems {
        rems(2.25)
    }

    pub fn x5l() -> Rems {
        rems(3.0)
    }

    pub fn x6l() -> Rems {
        rems(4.0)
    }
}

pub mod padding {
    use gpui::elements::node::length::{rems, Rems};

    pub fn p1() -> Rems {
        rems(0.25)
    }

    pub fn p2() -> Rems {
        rems(0.5)
    }

    pub fn p3() -> Rems {
        rems(0.75)
    }

    pub fn p4() -> Rems {
        rems(1.0)
    }

    pub fn p5() -> Rems {
        rems(1.25)
    }

    pub fn p6() -> Rems {
        rems(1.5)
    }

    pub fn p8() -> Rems {
        rems(2.0)
    }

    pub fn p10() -> Rems {
        rems(2.5)
    }

    pub fn p12() -> Rems {
        rems(3.0)
    }

    pub fn p16() -> Rems {
        rems(4.0)
    }

    pub fn p20() -> Rems {
        rems(5.0)
    }

    pub fn p24() -> Rems {
        rems(6.0)
    }

    pub fn p32() -> Rems {
        rems(8.0)
    }
}

pub mod margin {
    use gpui::elements::node::length::{rems, Rems};

    pub fn m1() -> Rems {
        rems(0.25)
    }

    pub fn m2() -> Rems {
        rems(0.5)
    }

    pub fn m3() -> Rems {
        rems(0.75)
    }

    pub fn m4() -> Rems {
        rems(1.0)
    }

    pub fn m5() -> Rems {
        rems(1.25)
    }

    pub fn m6() -> Rems {
        rems(1.5)
    }

    pub fn m8() -> Rems {
        rems(2.0)
    }

    pub fn m10() -> Rems {
        rems(2.5)
    }

    pub fn m12() -> Rems {
        rems(3.0)
    }

    pub fn m16() -> Rems {
        rems(4.0)
    }

    pub fn m20() -> Rems {
        rems(5.0)
    }

    pub fn m24() -> Rems {
        rems(6.0)
    }

    pub fn m32() -> Rems {
        rems(8.0)
    }
}
