use crate::gen::include::HEADER;
use crate::gen::out::Content;

pub(super) fn write(out: &mut Content, needed: bool, guard: &str) {
    let ifndef = format!("#ifndef {}", guard);
    let define = format!("#define {}", guard);
    let endif = format!("#endif // {}", guard);

    let mut offset = 0;
    loop {
        let begin = find_line(offset, &ifndef);
        let end = find_line(offset, &endif);
        if let (Some(begin), Some(end)) = (begin, end) {
            if !needed {
                return;
            }
            out.next_section();
            if offset == 0 {
                writeln!(out, "{}", ifndef);
                writeln!(out, "{}", define);
            }
            for line in HEADER[begin + ifndef.len()..end].trim().lines() {
                if line != define && !line.trim_start().starts_with("//") {
                    writeln!(out, "{}", line);
                }
            }
            offset = end + endif.len();
        } else if offset == 0 {
            panic!("not found in cxx.h header: {}", guard)
        } else {
            writeln!(out, "{}", endif);
            return;
        }
    }
}

fn find_line(mut offset: usize, line: &str) -> Option<usize> {
    loop {
        offset += HEADER[offset..].find(line)?;
        let rest = &HEADER[offset + line.len()..];
        if rest.starts_with('\n') || rest.starts_with('\r') {
            return Some(offset);
        }
        offset += line.len();
    }
}
