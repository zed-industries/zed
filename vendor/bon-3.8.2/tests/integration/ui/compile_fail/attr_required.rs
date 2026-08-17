use bon::Builder;

#[derive(Builder)]
struct InvalidOnRequiredMember {
    #[builder(required)]
    member: i32,
}

#[derive(Builder)]
struct InvalidOnStartFnMember {
    #[builder(start_fn, required)]
    member: Option<i32>,
}

#[derive(Builder)]
struct InvalidOnFnMember {
    #[builder(finish_fn, required)]
    member: Option<i32>,
}

#[derive(Builder)]
struct InvalidOnSkippedMember {
    #[builder(skip, required)]
    member: Option<i32>,
}

#[derive(Builder)]
struct Valid {
    #[builder(required)]
    member: Option<u32>,

    #[builder(required, with = Some)]
    some_member: Option<()>,
}

fn main() {
    // Make sure there is no `maybe_` setter generated
    let _ = Valid::builder().maybe_member(Some(42));
    let _ = Valid::builder().maybe_some_member(Some(()));

    // Another way to get transparency
    {
        type OpaqueOption<T> = Option<T>;

        #[derive(Builder)]
        struct Sut {
            arg1: OpaqueOption<u32>,
        }

        // Should not be allowed `OpaqueOption` is required
        let _ = Sut::builder().build();
    }
}
