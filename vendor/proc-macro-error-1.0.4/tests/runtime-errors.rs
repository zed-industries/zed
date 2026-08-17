use proc_macro_error::*;

#[test]
#[should_panic = "proc-macro-error API cannot be used outside of"]
fn missing_attr_emit() {
    emit_call_site_error!("You won't see me");
}

#[test]
#[should_panic = "proc-macro-error API cannot be used outside of"]
fn missing_attr_abort() {
    abort_call_site!("You won't see me");
}
