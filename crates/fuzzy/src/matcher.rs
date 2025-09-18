use std::{borrow::Cow, sync::Mutex};

pub static MATCHERS: Mutex<Vec<nucleo::Matcher>> = Mutex::new(Vec::new());
pub fn get_matcher(config: nucleo::Config) -> nucleo::Matcher {
    let mut matchers = MATCHERS.lock().unwrap();
    let mut matcher = matchers
        .pop()
        .unwrap_or_else(|| nucleo::Matcher::new(config.clone()));
    matcher.config = config;
    matcher
}

pub fn return_matcher(matcher: nucleo::Matcher) {
    MATCHERS.lock().unwrap().push(matcher);
}

pub fn get_matchers(n: usize, config: nucleo::Config) -> Vec<nucleo::Matcher> {
    let mut matchers: Vec<_> = {
        let mut matchers = MATCHERS.lock().unwrap();
        let num_matchers = matchers.len();
        matchers
            .drain(0..std::cmp::min(n, num_matchers))
            .map(|mut matcher| {
                matcher.config = config.clone();
                matcher
            })
            .collect()
    };
    matchers.resize_with(n, || nucleo::Matcher::new(config.clone()));
    matchers
}

pub fn return_matchers(mut matchers: Vec<nucleo::Matcher>) {
    MATCHERS.lock().unwrap().append(&mut matchers);
}
