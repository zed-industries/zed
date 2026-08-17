#![cfg(feature = "CFDate")]

#[cfg(test)]
mod tests {
    use crate::CFTimeZone;

    #[test]
    fn cmp() {
        let system = CFTimeZone::system().unwrap();
        let default = CFTimeZone::default().unwrap();
        assert_eq!(system, default);
        assert_eq!(system.name().unwrap(), default.name().unwrap(),);
    }
}
