use std::io::{Read, Write};

fn main() {
    let input = {
        let mut args = std::env::args().skip(1).collect::<Vec<_>>().join(" ");
        args.push('\n');
        args
    };
    let socket_path = std::env::args().nth(1).expect("missing socket path");
    let mut stream = windows_net::stream::UnixStream::connect(socket_path).unwrap();
    stream.write(input.as_bytes()).unwrap();

    stream.shutdown(std::net::Shutdown::Write).unwrap();

    let mut password = String::new();
    stream.read_to_string(&mut password).unwrap();
    println!("{}", password);
}
