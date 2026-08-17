use optfield::optfield;

#[optfield(Opt1, attrs)]
#[optfield(Opt2, attrs)]
#[optfield(Opt3, attrs)]
#[derive(Default)]
struct Original {
    field: String,
}

#[test]
fn multiple_optfield_same_struct() {
    let original = Original::default();
    let opt1 = Opt1::default();
    let opt2 = Opt2::default();
    let opt3 = Opt3::default();

    assert_eq!(original.field, "");
    assert_eq!(opt1.field, None);
    assert_eq!(opt2.field, None);
    assert_eq!(opt3.field, None);
}
