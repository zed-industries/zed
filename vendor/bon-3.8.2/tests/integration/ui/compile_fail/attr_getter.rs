use bon::Builder;

#[derive(Builder)]
struct StartFnCompat {
    #[builder(getter, start_fn)]
    x: u32,
}

#[derive(Builder)]
struct FinishFnCompat {
    #[builder(getter, finish_fn)]
    x: u32,
}

#[derive(Builder)]
struct SkipCompat {
    #[builder(getter, skip)]
    x: u32,
}

#[derive(Builder)]
struct NegativeTest {
    #[builder(getter)]
    x1: u32,

    #[builder(getter)]
    x2: Option<u32>,

    #[builder(getter, default)]
    x3: u32,
}

#[derive(Builder)]
struct NonCopy {
    #[builder(getter(copy))]
    x1: String,

    #[builder(getter(copy))]
    x2: Option<String>,

    #[builder(getter(copy), default)]
    x3: String,
}

#[derive(Default)]
struct NonClone;

#[derive(Builder)]
struct NonCloneTest {
    #[builder(getter(clone))]
    x1: NonClone,

    #[builder(getter(clone))]
    x2: Option<NonClone>,

    #[builder(getter(clone), default)]
    x3: NonClone,
}

#[derive(Builder)]
struct NoDeref {
    #[builder(getter(deref(u64)))]
    x1: String,

    #[builder(getter(deref(u64)))]
    x2: Option<String>,

    #[builder(getter(deref(u64)), default)]
    x3: String,
}

#[derive(Builder)]
struct CantInferDerefTarget {
    #[builder(getter(deref))]
    x1: u32,
}

#[derive(Builder)]
struct CopyCloneExclusion {
    #[builder(getter(copy, clone))]
    x1: u32,
}

#[derive(Builder)]
struct CopyDerefExclusion {
    #[builder(getter(copy, deref))]
    x1: u32,
}

#[derive(Builder)]
struct CloneDerefExclusion {
    #[builder(getter(clone, deref))]
    x1: u32,
}

fn main() {
    let builder = NegativeTest::builder();

    builder.get_x1();
    builder.get_x2();
    builder.get_x3();
}
