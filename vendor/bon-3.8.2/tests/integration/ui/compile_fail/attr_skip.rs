use bon::{builder, Builder};

#[derive(Builder)]
struct ConflictingAttrs {
    #[builder(skip, into)]
    x: u32,
}

#[derive(Builder)]
struct ConflictingAttrs2 {
    #[builder(skip, name = bar)]
    x: u32,
}

#[derive(Builder)]
struct ConflictingAttrs3 {
    #[builder(skip, default = 42)]
    z: u32,
}

#[builder]
fn skip_on_fn_is_unsupporetd1(#[builder(skip)] _x: u32) {}
#[builder]
fn skip_on_fn_is_unsupporetd2(#[builder(skip = "skip".to_owned())] _y: String) {}
#[builder]
fn skip_on_fn_is_unsupporetd3(#[builder(skip = vec![42])] _z: Vec<u32>) {}

fn main() {
    #[derive(Builder)]
    struct SkipGeneratesNoSetter {
        #[builder(skip)]
        x: u32,

        #[builder(skip = 4)]
        y: u32,
    }

    SkipGeneratesNoSetter::builder().x(42).build();
    SkipGeneratesNoSetter::builder().y(42).build();
}
