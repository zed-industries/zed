use dialoguer::{theme::ColorfulTheme, FuzzySelect};

fn main() {
    let selections = &[
        "Ice Cream",
        "Vanilla Cupcake",
        "Chocolate Muffin",
        "A Pile of sweet, sweet mustard",
        "Carrots",
        "Peas",
        "Pistacio",
        "Mustard",
        "Cream",
        "Banana",
        "Chocolate",
        "Flakes",
        "Corn",
        "Cake",
        "Tarte",
        "Cheddar",
        "Vanilla",
        "Hazelnut",
        "Flour",
        "Sugar",
        "Salt",
        "Potato",
        "French Fries",
        "Pizza",
        "Mousse au chocolat",
        "Brown sugar",
        "Blueberry",
        "Burger",
    ];

    let selection = FuzzySelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Pick your flavor")
        .default(0)
        .items(&selections[..])
        .interact()
        .unwrap();

    println!("Enjoy your {}!", selections[selection]);
}
