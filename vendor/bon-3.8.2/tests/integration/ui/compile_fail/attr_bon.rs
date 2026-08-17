use bon::bon;

struct InvalidAttrsForBonMacro;

#[bon(attrs)]
impl InvalidAttrsForBonMacro {
    #[builder]
    fn sut() {}
}

struct BuilderAttrOnReceiver;

#[bon]
impl BuilderAttrOnReceiver {
    #[builder]
    fn sut(#[builder] &self) {}
}

struct NoBuilderMethods;

#[bon]
impl NoBuilderMethods {
    fn not_builder1() {}
    fn not_builder2(&self) {}

    const NOT_BUILDER: () = ();
}

struct BuilderLikeMethodAttribute;

#[bon]
impl BuilderLikeMethodAttribute {
    fn not_builder() {}

    #[::foo::builder]
    fn builder_like() {}
}

struct ActiveBuilderInsideImplWithSelf;

#[bon]
impl ActiveBuilderInsideImplWithSelf {
    #[bon::builder]
    fn active_bon_with_self(&self) {}
}

struct ActiveBuilderInsideImplClueless;

#[bon]
impl ActiveBuilderInsideImplClueless {
    // The `builder` attribute is "clueless" here because nothing tells it that
    // it is inside of an impl block, so it silently "succeeds" to generate
    // a builder method, but it produces a bunch of errors that items can't
    // be defined inside of an impl block.
    #[bon::builder]
    fn active_bon_clueless() {}
}

fn main() {}
