extern crate is_docker;

fn main() {
    if is_docker::is_docker() {
        println!("Currently in a Docker Container!")
    } else {
        println!("Currently NOT in Docker Container!")
    }
}