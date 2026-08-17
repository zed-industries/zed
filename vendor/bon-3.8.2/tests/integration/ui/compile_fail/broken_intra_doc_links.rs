use bon::{bon, builder, Builder};

#[builder]
fn broken_link_in_arg_docs(
    /// [Self] link
    _arg: u32,
) {
}

struct BrokenLinkInArgDocs;

#[bon]
impl BrokenLinkInArgDocs {
    #[builder]
    fn broken_link_in_arg_docs(
        /// [Self] link
        _arg: u32,
    ) {
    }
}

#[derive(Builder)]
struct BrokenLinkInFieldDocs {
    /// [`Self`] link
    field: u32,
}

#[derive(Builder)]
struct BrokenLinkInSettersDocs {
    #[builder(setters(doc {
        /// [`Self`] link
    }))]
    field: u32,
}

#[derive(Builder)]
struct BrokenLinkInSomeFnDocs {
    #[builder(setters(
        some_fn(doc {
            /// [`Self`] link
        })
    ))]
    field: Option<u32>,
}

#[derive(Builder)]
struct BrokenLinkInOptionFnDocs {
    #[builder(setters(
        option_fn(doc {
            /// [`Self`] link
        })
    ))]
    field: Option<u32>,
}

#[derive(Builder)]
#[builder(builder_type(doc {
    /// [Self] link
}))]
struct BrokenLinkInBuilderTypeDocs {}

#[derive(Builder)]
#[builder(finish_fn(doc {
    /// [Self] link
}))]
struct BrokenLinkInFinishFnDocs {}

#[derive(Builder)]
#[builder(state_mod(doc {
    /// [Self] link
}))]
struct BrokenLinkInStateModDocs {}

fn main() {}
