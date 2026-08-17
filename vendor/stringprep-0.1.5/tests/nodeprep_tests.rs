// Examples from http://josefsson.org/idn.php
extern crate stringprep;

use stringprep::nodeprep;

#[test]
fn test_nodeprep() {
    assert_eq!(
        "räksmörgås.josefsson.org",
        nodeprep("räksmörgås.josefßon.org").unwrap()
    );
}
