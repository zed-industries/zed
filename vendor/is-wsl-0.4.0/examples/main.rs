extern crate is_wsl;
fn main() {
    if is_wsl::is_wsl() {
        println!("Currently in WSL")
    } else {
        println!("Currently NOT in WSL")
    }
}
