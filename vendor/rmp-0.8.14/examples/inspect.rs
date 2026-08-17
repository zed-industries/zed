use std::fmt;
use std::io::{self, Read};
use rmp::{decode::*, Marker};

fn main() {
    let path = std::env::args_os().nth(1).expect("Specify path to a file with msgpack content");
    let data = std::fs::read(&path).expect(&path.to_string_lossy());

    dump(&mut Indent { i: 0, start: true }, &mut data.as_slice()).unwrap();
}

fn dump(indent: &mut Indent, rd: &mut &[u8]) -> Result<(), Box<dyn std::error::Error>> {
    match read_marker(rd).map_err(ValueReadError::from)? {
        Marker::FixPos(n) => print!("U0({n})"),
        Marker::FixNeg(n) => print!("I0({n})"),
        Marker::Null => print!("Null"),
        Marker::True => print!("True"),
        Marker::False => print!("False"),
        Marker::U8 => print!("U8({})", rd.read_data_u8()?),
        Marker::U16 => print!("U16({})", rd.read_data_u16()?),
        Marker::U32 => print!("U32({})", rd.read_data_u32()?),
        Marker::U64 => print!("U64({})", rd.read_data_u64()?),
        Marker::I8 => print!("I8({})", rd.read_data_i8()?),
        Marker::I16 => print!("I16({})", rd.read_data_i16()?),
        Marker::I32 => print!("I32({})", rd.read_data_i32()?),
        Marker::I64 => print!("I64({})", rd.read_data_i64()?),
        Marker::F32 => print!("F32({})", rd.read_data_f32()?),
        Marker::F64 => print!("F64({})", rd.read_data_f64()?),
        Marker::FixStr(len) => print!("Str0(\"{}\")", read_str_data(len.into(), rd)?),
        Marker::Str8 => print!("Str8(\"{}\")", read_str_data(rd.read_data_u8()?.into(), rd)?),
        Marker::Str16 => print!("Str16(\"{}\")", read_str_data(rd.read_data_u16()?.into(), rd)?),
        Marker::Str32 => print!("Str32(\"{}\")", read_str_data(rd.read_data_u32()?.into(), rd)?),
        Marker::Bin8 => print!("Bin8({})", HexDump(&read_bin_data(rd.read_data_u8()?.into(), rd)?)),
        Marker::Bin16 => print!("Bin16({})", HexDump(&read_bin_data(rd.read_data_u16()?.into(), rd)?)),
        Marker::Bin32 => print!("Bin32({})", HexDump(&read_bin_data(rd.read_data_u32()?.into(), rd)?)),
        Marker::FixArray(len) => dump_array(indent, 0, len.into(), rd)?,
        Marker::Array16 => dump_array(indent, 16, rd.read_data_u16()?.into(), rd)?,
        Marker::Array32 => dump_array(indent, 32, rd.read_data_u32()?.into(), rd)?,
        Marker::FixMap(len) => dump_map(indent, 0, len.into(), rd)?,
        Marker::Map16 => dump_map(indent, 16, rd.read_data_u16()?.into(), rd)?,
        Marker::Map32 => dump_map(indent, 32, rd.read_data_u32()?.into(), rd)?,
        Marker::FixExt1 => todo!(),
        Marker::FixExt2 => todo!(),
        Marker::FixExt4 => todo!(),
        Marker::FixExt8 => todo!(),
        Marker::FixExt16 => todo!(),
        Marker::Ext8 => todo!(),
        Marker::Ext16 => todo!(),
        Marker::Ext32 => todo!(),
        Marker::Reserved => todo!(),
    }
    Ok(())
}

fn dump_map(indent: &mut Indent, ty: u8, len: u32, rd: &mut &[u8]) -> Result<(), Box<dyn std::error::Error>> {
    indent.print(format_args!("Map{ty}{{"));
    let multiline = len > 1;
    if multiline { indent.ln(); } else { print!(" ") }
    indent.ind();
    for i in 0..len {
        indent.print(""); dump(indent, rd)?; print!(": ");  dump(indent, rd)?;
        if multiline {  print!(","); indent.ln(); } else if i+1 != len { print!(", ") }
    }
    indent.out();
    indent.print(format_args!("}}"));
    Ok(())
}

fn dump_array(indent: &mut Indent, ty: u8, len: u32, rd: &mut &[u8]) -> Result<(), Box<dyn std::error::Error>> {
    indent.print(format_args!("Array{ty}["));
    let multiline = len > 1;
    if multiline { indent.ln(); } else { print!(" ") }
    indent.ind();
    for i in 0..len {
        indent.print(""); dump(indent, rd)?;
        if multiline {  print!(","); indent.ln(); } else if i+1 != len { print!(", ") }
    }
    indent.out();
    indent.print("]");
    Ok(())
}

fn read_str_data<R: Read>(len: u32, rd: &mut R) -> Result<String, io::Error> {
    Ok(String::from_utf8_lossy(&read_bin_data(len, rd)?).into_owned())
}

fn read_bin_data<R: Read>(len: u32, rd: &mut R) -> Result<Vec<u8>, io::Error> {
    let mut buf = Vec::with_capacity(len.min(1<<16) as usize);
    let bytes_read = rd.take(len as u64).read_to_end(&mut buf)?;
    if bytes_read != len as usize {
        return Err(io::ErrorKind::UnexpectedEof.into());
    }
    Ok(buf)
}

struct Indent { i: u16, start: bool }
impl Indent {
    fn print(&mut self, args: impl fmt::Display) {
        print!("{:w$}{args}", "", w = if self.start { (self.i as usize) * 2 } else { 0 });
        self.start = false;
    }

    pub fn ind(&mut self) {
        self.i += 1;
    }

    pub fn ln(&mut self) {
        println!();
        self.start = true;
    }

    pub fn out(&mut self) {
        self.i -= 1;
    }
}

struct HexDump<'a>(&'a [u8]);
impl fmt::Display for HexDump<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let truncate = self.0.len() > 50;
        if truncate {
            f.write_fmt(format_args!("{}B ", self.0.len()))?;
        }

        for &b in &self.0[0.. (if truncate { 50 } else { self.0.len() })] {
            f.write_fmt(format_args!("{b:02x}"))?;
        }

        if truncate {
            f.write_str("…")?;
        }
        Ok(())
    }
}

