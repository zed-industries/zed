use dialoguer::{theme::ColorfulTheme, Sort};

fn main() {
    let list = &[
        "Ice Cream",
        "Vanilla Cupcake",
        "Chocolate Muffin",
        "A Pile of sweet, sweet mustard",
    ];
    let sorted = Sort::with_theme(&ColorfulTheme::default())
        .with_prompt("Order your foods by preference")
        .items(&list[..])
        .interact()
        .unwrap();

    println!("Your favorite item:");
    println!("  {}", list[sorted[0]]);
    println!("Your least favorite item:");
    println!("  {}", list[sorted[sorted.len() - 1]]);

    let sorted = Sort::with_theme(&ColorfulTheme::default())
        .with_prompt("Order your foods by preference")
        .items(&list[..])
        .max_length(2)
        .interact()
        .unwrap();

    println!("Your favorite item:");
    println!("  {}", list[sorted[0]]);
    println!("Your least favorite item:");
    println!("  {}", list[sorted[sorted.len() - 1]]);
}
