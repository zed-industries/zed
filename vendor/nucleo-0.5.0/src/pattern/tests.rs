use nucleo_matcher::pattern::{CaseMatching, Normalization};

use crate::pattern::{MultiPattern, Status};

#[test]
fn append() {
    let mut pat = MultiPattern::new(1);
    pat.reparse(0, "!", CaseMatching::Smart, Normalization::Smart, true);
    assert_eq!(pat.status(), Status::Update);
    pat.reparse(0, "!f", CaseMatching::Smart, Normalization::Smart, true);
    assert_eq!(pat.status(), Status::Update);
    pat.reparse(0, "!fo", CaseMatching::Smart, Normalization::Smart, true);
    assert_eq!(pat.status(), Status::Rescore);
}
