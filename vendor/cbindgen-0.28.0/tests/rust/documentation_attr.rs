#[doc="With doc attr, each attr contribute to one line of document"]
#[doc="like this one with a new line character at its end"]
#[doc="and this one as well. So they are in the same paragraph"]
#[doc=""]
#[doc="Line ends with one new line\nshould not break"]
#[doc=""]
#[doc="Line ends with two spaces and a new line  \nshould break to next line"]
#[doc=""]
#[doc="Line ends with two new lines\n\nShould break to next paragraph"]
#[no_mangle]
pub extern "C" fn root() {
}
