use std::sync::Mutex;

static MATCHERS: Mutex<Vec<nucleo::Matcher>> = Mutex::new(Vec::new());

pub const LENGTH_PENALTY: f64 = 0.01;

pub fn get_matcher(config: nucleo::Config) -> nucleo::Matcher {
    let mut matchers = MATCHERS.lock().unwrap();
    match matchers.pop() {
        Some(mut matcher) => {
            matcher.config = config;
            matcher
        }
        None => nucleo::Matcher::new(config),
    }
}

pub fn return_matcher(matcher: nucleo::Matcher) {
    MATCHERS.lock().unwrap().push(matcher);
}

pub fn get_matchers(n: usize, config: nucleo::Config) -> Vec<nucleo::Matcher> {
    let mut matchers: Vec<_> = {
        let mut pool = MATCHERS.lock().unwrap();
        let available = pool.len().min(n);
        pool.drain(..available)
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
