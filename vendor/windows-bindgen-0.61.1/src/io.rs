use std::io::BufRead;

#[track_caller]
pub fn read_file_lines(path: &str) -> Vec<String> {
    let Ok(file) = std::fs::File::open(path) else {
        panic!("failed to open file `{path}`")
    };

    let file = std::io::BufReader::new(file);
    let mut lines = vec![];

    for line in file.lines() {
        let Ok(line) = line else {
            panic!("failed to read file lines `{path}`");
        };

        lines.push(line);
    }

    lines
}

#[track_caller]
pub fn write_to_file<C: AsRef<[u8]>>(path: &str, contents: C) {
    if let Some(parent) = std::path::Path::new(path).parent() {
        if std::fs::create_dir_all(parent).is_err() {
            panic!("failed to create directory `{path}`");
        }
    }

    if std::fs::write(path, contents).is_err() {
        panic!("failed to write file `{path}`");
    }
}
