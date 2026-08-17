#[cfg(test)]
mod conversion {

    use notify_rust::Urgency;

    #[test]
    fn urgency_from_int() {
        assert_eq!(Urgency::from(0), Urgency::Low);
        assert_eq!(Urgency::from(1), Urgency::Normal);
        assert_eq!(Urgency::from(2), Urgency::Critical);
        assert_eq!(Urgency::from(900), Urgency::Critical);
        assert_eq!(Urgency::from(u64::MAX), Urgency::Critical);
    }

    #[test]
    fn urgency_from_option_int() {
        assert_eq!(Urgency::from(Some(0)), Urgency::Low);
        assert_eq!(Urgency::from(Some(1)), Urgency::Normal);
        assert_eq!(Urgency::from(None), Urgency::Normal);
        assert_eq!(Urgency::from(Some(2)), Urgency::Critical);
    }

    #[test]
    fn str_into_urgency() {
        let u0: Urgency = "low".try_into().unwrap();
        assert_eq!(u0, Urgency::Low);
    }

    #[test]
    fn urgency_from_str() {
        assert_eq!(Urgency::try_from("low").ok(), Some(Urgency::Low));
        assert_eq!(Urgency::try_from("medium").ok(), Some(Urgency::Normal));
        assert_eq!(Urgency::try_from("Normal").ok(), Some(Urgency::Normal));
        assert_eq!(Urgency::try_from("NoRmaL").ok(), Some(Urgency::Normal));
        assert_eq!(Urgency::try_from("High").ok(), Some(Urgency::Critical));
        assert_eq!(Urgency::try_from("Hi").ok(), Some(Urgency::Critical));
        assert_eq!(Urgency::try_from("Critical").ok(), Some(Urgency::Critical));
    }
}
