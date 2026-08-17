use bon::Builder;

struct IntoUnit;

impl From<IntoUnit> for () {
    fn from(_: IntoUnit) -> Self {
        ()
    }
}

pub fn test_type_pattern_matching() {
    #[derive(Builder)]
    #[builder(on((), into))]
    struct TypePatternMatching {
        #[builder(start_fn)]
        _a: (),

        #[builder(start_fn)]
        _b: Option<()>,

        #[builder(finish_fn)]
        _c: (),

        #[builder(finish_fn)]
        _d: Option<()>,
    }

    TypePatternMatching::builder(IntoUnit, IntoUnit).build(IntoUnit, IntoUnit);
}

fn main() {}
