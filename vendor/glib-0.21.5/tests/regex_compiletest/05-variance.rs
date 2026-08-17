use glib::MatchInfo;

// pass
fn covariance_check<'short>(input: MatchInfo<'static>) -> MatchInfo<'short> {
    input
}

// fail
fn contravariance_check<'short>(input: MatchInfo<'short>) -> MatchInfo<'static> {
    input
}

fn main() {}
