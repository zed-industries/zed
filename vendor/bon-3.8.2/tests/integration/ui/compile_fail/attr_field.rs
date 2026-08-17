use bon::Builder;

#[derive(Builder)]
struct WrongName {
    #[builder(field)]
    __x1: i32,
}

mod privacy {
    #[derive(bon::Builder)]
    pub struct MustBePrivate {
        #[builder(field)]
        x1: i32,
    }
}

fn main() {
    let builder = privacy::MustBePrivate::builder();

    // Should be inaccessible in this scope
    let _ = builder.x1;
}
