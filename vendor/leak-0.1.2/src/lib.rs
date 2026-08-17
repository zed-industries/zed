use std::mem;

/**
 * Leak a piece of data by never calling its destructor
 *
 * Useful for things that are going to be used for the life of the program, but aren't technically
 * static (because they are created in response to arguments, environment, or other
 * configuration/data read at program start).
 *
 * This is a modified version of the proposed rfc: https://github.com/rust-lang/rfcs/pull/1233
 *
 *
 * Notable changes:
 *  - for user convenience, leak() is a non-static method
 *  - Return `&T` instead of `&mut T`
 *
 * While it would be ideal to return a `&'a mut T`, we apparently can't do that due to limitations
 * in rust's borrow checker causing soundness issues. Details are in the RFC liked above.
 */
pub trait Leak<T : ?Sized> {
    fn leak<'a>(self) -> &'a T where T: 'a;
}

impl<T : ?Sized> Leak<T> for Box<T> {
    fn leak<'a>(self) -> &'a T where T: 'a {
        let r = Self::into_raw(self);
        unsafe { &mut *r }
    }
}

/*
 * while String and Vec<T> could have impls in terms of Box, we specialize them because their
 * conversions to Box (into_boxed_slice and into_boxed_str) result in resizing underlying storage
 */

impl Leak<str> for String {
    fn leak<'a>(mut self) -> &'a str where Self: 'a {
        let r: *mut str = &mut self[..];
        mem::forget(self);
        unsafe { &mut *r }
    }
}

impl<T> Leak<[T]> for Vec<T> {
    fn leak<'a>(mut self) -> &'a [T] where [T]: 'a {
        let r: *mut [T] = &mut self[..];
        mem::forget(self);
        unsafe { &mut *r }
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn leak_str() {
        use super::Leak;
        use std::borrow::ToOwned;

        let v = "hi";
        let leaked : &str = {
            let o = v.to_owned();
            o.leak()
        };
        assert_eq!(leaked, v);

        let leaked : &'static str = {
            let o = v.to_owned();
            o.leak()
        };
        assert_eq!(leaked, v);
    }

    #[test]
    fn leak_empty_str() {
        use super::Leak;
        use std::borrow::ToOwned;

        let v = "";
        let leaked : &'static str = {
            let o = v.to_owned();
            o.leak()
        };
        assert_eq!(leaked, v);
    }

    #[test]
    fn leak_vec() {
        use super::Leak;

        let v = vec![3, 5];
        let leaked : &'static [u8] = {
            let o = v.clone();
            o.leak()
        };
        assert_eq!(leaked, &*v);
    }

    #[test]
    fn leak_empty_vec() {
        use super::Leak;

        let v = vec![];
        let leaked : &'static [u8] = {
            let o = v.clone();
            o.leak()
        };
        assert_eq!(leaked, &*v);
    }

    #[test]
    fn leak_box() {
        use super::Leak;

        let v : Box<[&str]> = vec!["hi", "there"].into_boxed_slice();
        let leaked : &'static [&str] = {
            let o = v.clone();
            o.leak()
        };
        assert_eq!(leaked, &*v);
    }

    #[test]
    fn leak_nested() {
        use super::Leak;

        let v : Box<Vec<&str>> = Box::new(vec!["hi", "there"]);
        let leaked : &'static [&str] = {
            let o = v.clone();
            o.leak()
        };
        assert_eq!(leaked, &**v);
    }
}
