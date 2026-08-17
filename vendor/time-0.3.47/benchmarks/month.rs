use criterion::Bencher;
use time::Month::*;

setup_benchmark! {
    "Month",

    fn previous(ben: &mut Bencher<'_>) {
        ben.iter(|| January.previous());
        ben.iter(|| February.previous());
        ben.iter(|| March.previous());
        ben.iter(|| April.previous());
        ben.iter(|| May.previous());
        ben.iter(|| June.previous());
        ben.iter(|| July.previous());
        ben.iter(|| August.previous());
        ben.iter(|| September.previous());
        ben.iter(|| October.previous());
        ben.iter(|| November.previous());
        ben.iter(|| December.previous());
    }

    fn next(ben: &mut Bencher<'_>) {
        ben.iter(|| January.next());
        ben.iter(|| February.next());
        ben.iter(|| March.next());
        ben.iter(|| April.next());
        ben.iter(|| May.next());
        ben.iter(|| June.next());
        ben.iter(|| July.next());
        ben.iter(|| August.next());
        ben.iter(|| September.next());
        ben.iter(|| October.next());
        ben.iter(|| November.next());
        ben.iter(|| December.next());
    }

    fn length(ben: &mut Bencher<'_>) {
        // Common year
        ben.iter(|| January.length(2019));
        ben.iter(|| February.length(2019));
        ben.iter(|| March.length(2019));
        ben.iter(|| April.length(2019));
        ben.iter(|| May.length(2019));
        ben.iter(|| June.length(2019));
        ben.iter(|| July.length(2019));
        ben.iter(|| August.length(2019));
        ben.iter(|| September.length(2019));
        ben.iter(|| October.length(2019));
        ben.iter(|| November.length(2019));
        ben.iter(|| December.length(2019));

        // Leap year
        ben.iter(|| January.length(2020));
        ben.iter(|| February.length(2020));
        ben.iter(|| March.length(2020));
        ben.iter(|| April.length(2020));
        ben.iter(|| May.length(2020));
        ben.iter(|| June.length(2020));
        ben.iter(|| July.length(2020));
        ben.iter(|| August.length(2020));
        ben.iter(|| September.length(2020));
        ben.iter(|| October.length(2020));
        ben.iter(|| November.length(2020));
        ben.iter(|| December.length(2020));
    }
}
