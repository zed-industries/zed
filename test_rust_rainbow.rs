fn main() {
    let vec = vec![1, 2, 3];
    let map = HashMap::new();

    if true {
        println!("Hello {}", "world");
        let nested = vec![(1, 2), (3, 4)];
    }

    match vec.len() {
        0 => println!("empty"),
        1..=5 => {
            for item in vec {
                println!("{}", item);
            }
        }
        _ => println!("large"),
    }
}