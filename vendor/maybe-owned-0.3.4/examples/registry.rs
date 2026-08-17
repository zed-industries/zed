extern crate maybe_owned;

use maybe_owned::MaybeOwned;
use std::collections::HashMap;
use std::time::SystemTime;

struct Data {
    text: String,
    // this should be some think like
    // chrono::Date, but then it's just an examples
    time: SystemTime,
}

impl Data {
    fn new<T>(text: T) -> Data
    where
        T: Into<String>,
    {
        Data {
            text: text.into(),
            time: SystemTime::now(),
        }
    }
}

#[derive(Default)]
struct Regestry<'a> {
    registry: HashMap<String, MaybeOwned<'a, Data>>,
}

impl<'a> Regestry<'a> {
    fn new() -> Regestry<'a> {
        Default::default()
    }

    fn register_data<K, D>(&mut self, key: K, data: D) -> Option<MaybeOwned<'a, Data>>
    where
        K: Into<String>,
        D: Into<MaybeOwned<'a, Data>>,
    {
        self.registry.insert(key.into(), data.into())
    }

    fn print_me(&self) {
        for (key, val) in self.registry.iter() {
            println!(
                "got: {:>6} => {:>11} {:<10} @ {:10.10?}",
                //we can just deref MaybeOwned
                key,
                val.text,
                if val.is_owned() {
                    "[owned]"
                } else {
                    "[borrowed]"
                },
                val.time
            )
        }
    }
}

fn main() {
    let shared_data = Data::new("--missing--");

    let mut reg = Regestry::new();
    reg.register_data("tom", Data::new("abc"));
    reg.register_data("lucy", &shared_data);
    reg.register_data("peter", &shared_data);
    reg.print_me();
}
