use kv_log_macro::info;

fn main() {
    femme::start(log::LevelFilter::Info).unwrap();
    info!("hello");
    info!("hello",);
    info!("hello {}", "cats");
    info!("hello {}", "cats",);
    info!("hello {}", "cats", {
        cat_1: "chashu",
        cat_2: "nori",
        cat_count: 2,
    });
}
