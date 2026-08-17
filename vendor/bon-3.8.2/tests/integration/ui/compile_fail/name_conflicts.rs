use bon::builder;

#[builder]
fn body(val: &u32) {
    let _: &'fn1 u32 = val;
}

#[builder]
fn attr_with(#[builder(with = |val: &'fn1 u32| val)] _val: &u32) {}

#[builder]
fn attr_default(
    #[builder(default = {
        let val: &'fn1 u32 = &32;
        val
    })]
    _val: &u32,
) {
}

fn main() {}
