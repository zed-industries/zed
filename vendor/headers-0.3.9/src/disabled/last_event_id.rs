use std::fmt;

use util::HeaderValueString;

/// `Last-Event-ID` header, defined in
/// [RFC3864](https://html.spec.whatwg.org/multipage/references.html#refsRFC3864)
///
/// The `Last-Event-ID` header contains information about
/// the last event in an http interaction so that it's easier to
/// track of event state. This is helpful when working
/// with [Server-Sent-Events](http://www.html5rocks.com/en/tutorials/eventsource/basics/). If the connection were to be dropped, for example, it'd
/// be useful to let the server know what the last event you
/// received was.
///
/// The spec is a String with the id of the last event, it can be
/// an empty string which acts a sort of "reset".
// NOTE: This module is disabled since there is no const LAST_EVENT_ID to be
// used for the `impl Header`. It should be possible to enable this module
// when `HeaderName::from_static` can become a `const fn`.
#[derive(Clone, Debug, PartialEq, Header)]
pub struct LastEventId(HeaderValueString);


impl fmt::Display for LastEventId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

#[cfg(test)]
mod tests {

    /*
    // Initial state
    test_header!(test1, vec![b""]);
    // Own testcase
    test_header!(test2, vec![b"1"], Some(LastEventId("1".to_owned())));
    */
}

