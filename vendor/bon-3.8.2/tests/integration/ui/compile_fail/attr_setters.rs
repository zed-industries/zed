use bon::Builder;

#[derive(Builder)]
struct UnusedNameConfig {
    #[builder(setters(
        name = littlepip,
        some_fn = blackjack,
        option_fn = roseluck,
    ))]
    value: Option<u32>,
}

#[derive(Builder)]
struct UnusedNameConfigVerbose {
    #[builder(setters(
        name = littlepip,
        some_fn(name = blackjack),
        option_fn(name = roseluck),
    ))]
    value: Option<u32>,
}

#[derive(Builder)]
struct UnusedVisConfig {
    #[builder(setters(vis = "pub(crate)", some_fn(vis = ""), option_fn(vis = ""),))]
    value: Option<u32>,
}

#[derive(Builder)]
struct UnusedDocsConfig {
    #[builder(setters(
        doc {
            /// Unused
        },
        some_fn(doc {
            /// some_fn docs
        }),
        option_fn(doc {
            /// option_fn docs
        }),
    ))]
    value: Option<u32>,
}

#[derive(Builder)]
struct SomeFnSetterRequiredMember {
    #[builder(setters(some_fn = foo))]
    member: i32,
}

#[derive(Builder)]
struct OptionFnSetterOnRequiredMember {
    #[builder(setters(option_fn = bar))]
    member: i32,
}

#[derive(Builder)]
struct SomeFnSetterWithrequired {
    #[builder(required, setters(some_fn = foo))]
    member: Option<i32>,
}

#[derive(Builder)]
struct OptionFnSetterWithrequired {
    #[builder(required, setters(option_fn = bar))]
    member: Option<i32>,
}

#[derive(Builder)]
struct EmptySettersConfig {
    #[builder(setters())]
    member: i32,
}

#[derive(Builder)]
struct RepeatedSettersDocContent {
    #[builder(setters(
        doc {
            /// Doc 1
        },
        doc {
            /// Doc 2
        }
    ))]
    member: i32,
}

#[derive(Builder)]
struct RepeatedSettersDocConfig {
    #[builder(setters(
        doc {
            /// Doc 1
        },
        doc(default(skip)),
        doc(default(skip)),
    ))]
    member: i32,
}

#[derive(Builder)]
struct InvalidSettersDocSyntax {
    #[builder(setters(doc["Doc 2"]))]
    member: i32,
}

#[derive(Builder)]
struct SettersDocDefaultOnNonDefault {
    #[builder(setters(doc(default(skip))))]
    member: i32,
}

fn main() {}
