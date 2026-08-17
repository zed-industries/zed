use bon::{bon, builder, Builder};

#[derive(Builder)]
#[builder(start_fn())]
struct EmptyStartFn {}

#[derive(Builder)]
#[builder(finish_fn())]
struct EmptyFinisFn {}

#[derive(Builder)]
#[builder(start_fn)]
struct BareStartFnAttrOnStruct {}

#[builder(start_fn)]
fn bare_start_fn_on_free_function() {}

#[builder(start_fn())]
fn empty_paren_start_fn_on_free_function() {}

#[builder(start_fn(vis = ""))]
fn missing_name_for_start_fn_on_free_function1() {}

#[builder(start_fn(doc {}))]
fn missing_name_for_start_fn_on_free_function2() {}

struct AssocCtx;
struct AssocCtx2;
struct AssocCtx3;
struct AssocCtx4;

#[bon]
impl AssocCtx {
    #[builder(start_fn)]
    fn new() {}
}

#[bon]
impl AssocCtx2 {
    #[builder(start_fn())]
    fn new() {}
}

#[bon]
impl AssocCtx3 {
    #[builder(start_fn(vis = ""))]
    fn new() {}
}

#[bon]
impl AssocCtx4 {
    #[builder(start_fn(doc {}))]
    fn new() {}
}

#[bon]
impl AssocCtx {
    #[builder(start_fn)]
    fn bare_start_fn_on_method() {}
}

#[bon]
impl AssocCtx {
    #[builder(start_fn(vis = ""))]
    fn missing_name_for_start_fn_on_method1() {}
}

#[bon]
impl AssocCtx {
    #[builder(start_fn(doc {}))]
    fn missing_name_for_start_fn_on_method2() {}
}

fn main() {}
