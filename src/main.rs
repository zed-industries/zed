use std::io::{self, Write};

fn draw_box(width: usize, height: usize) {
    // Top border
    println!("┌{}┐", "─".repeat(width));
    
    // Middle rows
    for _ in 0..height {
        println!("│{}│", " ".repeat(width));
    }
    
    // Bottom border
    println!("└{}┘", "─".repeat(width));
}

fn main() {
    // Draw a box
    println!("Drawing a box:");
    draw_box(20, 5);
    
    // Original print examples
    println!("\nOther print examples:");
    println!("Hello from Rust!");
    print!("This is on the same line");
    println!(" and this continues on the same line");
    
    // Print with formatting
    let name = "Mac";
    println!("Hello, {}!", name);
    
    // Print with different number formats
    let number = 42;
    println!("Decimal: {}", number);
    println!("Hex: {:x}", number);
    println!("Binary: {:b}", number);
    
    // Print with alignment
    println!("{:<10} | {:>10}", "Left", "Right");
    println!("{:-<10} | {:->10}", "", "");
    
    // Print with precision for floats
    let pi = 3.14159;
    println!("Pi with 2 decimal places: {:.2}", pi);
    
    // Print with debug formatting
    let point = (10, 20);
    println!("Debug format: {:?}", point);
    
    // Print with pretty debug formatting
    let complex = vec![1, 2, 3, 4, 5];
    println!("Pretty debug format:\n{:#?}", complex);
    
    // Print to stderr
    eprintln!("This is an error message");
    
    // Manual flush example
    print!("This will be flushed immediately...");
    io::stdout().flush().unwrap();
    println!(" and this comes after the flush");
} 