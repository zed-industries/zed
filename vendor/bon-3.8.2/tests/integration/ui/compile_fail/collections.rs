use std::collections::{BTreeMap, BTreeSet};

fn main() {
    let _repeated_keys_in_map: BTreeMap<String, String> = bon::map! {
        "Hello": "Blackjack",
        "Hello": "Littlepip",
    };

    let _set: BTreeSet<String> = bon::set!["mintals", "guns", "mintals", "roses"];
}
