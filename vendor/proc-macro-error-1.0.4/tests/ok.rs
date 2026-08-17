extern crate test_crate;

use test_crate::*;

ok!(it_works);

#[test]
fn check_it_works() {
    it_works();
}
