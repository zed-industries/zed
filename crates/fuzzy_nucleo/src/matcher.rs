use std::sync::Mutex;

static MATCHERS: Mutex<Vec<nucleo::Matcher>> = Mutex::new(Vec::new());

pub const LENGTH_PENALTY: f64 = 0.01;

fn pool_cap() -> usize {
    std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(8)
        .max(1)
}

pub fn get_matcher(config: nucleo::Config) -> nucleo::Matcher {
    let mut matchers = MATCHERS.lock().unwrap_or_else(|e| e.into_inner());
    match matchers.pop() {
        Some(mut matcher) => {
            matcher.config = config;
            matcher
        }
        None => nucleo::Matcher::new(config),
    }
}

pub fn return_matcher(matcher: nucleo::Matcher) {
    let mut pool = MATCHERS.lock().unwrap_or_else(|e| e.into_inner());
    if pool.len() < pool_cap() {
        pool.push(matcher);
    }
}

pub fn get_matchers(n: usize, config: nucleo::Config) -> Vec<nucleo::Matcher> {
    let mut matchers: Vec<_> = {
        let mut pool = MATCHERS.lock().unwrap_or_else(|e| e.into_inner());
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

pub fn return_matchers(matchers: Vec<nucleo::Matcher>) {
    let cap = pool_cap();
    let mut pool = MATCHERS.lock().unwrap_or_else(|e| e.into_inner());
    let space = cap.saturating_sub(pool.len());
    pool.extend(matchers.into_iter().take(space));
}
