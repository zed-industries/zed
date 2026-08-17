#![allow(non_upper_case_globals)]

use std::env;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;

use getopts::{Matches, Options, ParsingStyle};
use term::{color, Attr};

fn parse_args() -> Matches {
    let args: Vec<String> = env::args().collect();
    let mut opts = Options::new();
    opts.optflag("c", "", "colorize output (for ANSI terminals)")
        .optflag("q", "", "test quietly (output only errors)")
        .optflag(
            "t",
            "",
            "print contents of tEXt/zTXt/iTXt chunks (can be used with -q)",
        )
        .optflag("v", "", "test verbosely (print most chunk data)")
        .parsing_style(ParsingStyle::StopAtFirstFree);
    if args.len() > 1 {
        match opts.parse(&args[1..]) {
            Ok(matches) => return matches,
            Err(err) => println!("{}", err),
        }
    }
    println!("{}", opts.usage("Usage: pngcheck [-cpt] [file ...]"));
    std::process::exit(0);
}

#[derive(Clone, Copy)]
struct Config {
    quiet: bool,
    verbose: bool,
    color: bool,
    text: bool,
}

fn display_interlaced(i: bool) -> &'static str {
    if i {
        "interlaced"
    } else {
        "non-interlaced"
    }
}

fn display_image_type(bits: u8, color: png::ColorType) -> String {
    use png::ColorType::*;
    format!(
        "{}-bit {}",
        bits,
        match color {
            Grayscale => "grayscale",
            Rgb => "RGB",
            Indexed => "palette",
            GrayscaleAlpha => "grayscale+alpha",
            Rgba => "RGB+alpha",
        }
    )
}
// channels after expansion of tRNS
fn final_channels(c: png::ColorType, trns: bool) -> u8 {
    use png::ColorType::*;
    match c {
        Grayscale => 1 + u8::from(trns),
        Rgb => 3,
        Indexed => 3 + u8::from(trns),
        GrayscaleAlpha => 2,
        Rgba => 4,
    }
}
fn check_image<P: AsRef<Path>>(c: Config, fname: P) -> io::Result<()> {
    // TODO improve performance by reusing allocations from decoder
    use png::Decoded::*;
    let mut t = term::stdout()
        .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "could not open terminal"))?;
    let data = &mut vec![0; 10 * 1024][..];
    let mut reader = io::BufReader::new(File::open(&fname)?);
    let fname = fname.as_ref().to_string_lossy();
    let n = reader.read(data)?;
    let mut buf = &data[..n];
    let mut pos = 0;
    let mut decoder = png::StreamingDecoder::new();
    // Image data
    let mut width = 0;
    let mut height = 0;
    let mut color = png::ColorType::Grayscale;
    let mut bits = 0;
    let mut trns = false;
    let mut interlaced = false;
    let mut compressed_size = 0;
    let mut n_chunks = 0;
    let mut have_idat = false;
    macro_rules! c_ratio(
        // TODO add palette entries to compressed_size
        () => ({
            compressed_size as f32/(
                height as u64 *
                (width as u64 * final_channels(color, trns) as u64 * bits as u64 + 7)>>3
            ) as f32
        });
    );
    let display_error = |err| -> Result<_, io::Error> {
        let mut t = term::stdout()
            .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "could not open terminal"))?;
        if c.verbose {
            if c.color {
                print!(": ");
                t.fg(color::RED)?;
                writeln!(t, "{}", err)?;
                t.attr(Attr::Bold)?;
                write!(t, "ERRORS DETECTED")?;
                t.reset()?;
            } else {
                println!(": {}", err);
                print!("ERRORS DETECTED")
            }
            println!(" in {}", fname);
        } else {
            if !c.quiet {
                if c.color {
                    t.fg(color::RED)?;
                    t.attr(Attr::Bold)?;
                    write!(t, "ERROR")?;
                    t.reset()?;
                    write!(t, ": ")?;
                    t.fg(color::YELLOW)?;
                    writeln!(t, "{}", fname)?;
                    t.reset()?;
                } else {
                    println!("ERROR: {}", fname)
                }
            }
            print!("{}: ", fname);
            if c.color {
                t.fg(color::RED)?;
                writeln!(t, "{}", err)?;
                t.reset()?;
            } else {
                println!("{}", err);
            }
        }
        Ok(())
    };

    if c.verbose {
        print!("File: ");
        if c.color {
            t.attr(Attr::Bold)?;
            write!(t, "{}", fname)?;
            t.reset()?;
        } else {
            print!("{}", fname);
        }
        print!(" ({}) bytes", data.len())
    }
    loop {
        if buf.is_empty() {
            // circumvent borrow checker
            assert!(!data.is_empty());
            let n = reader.read(data)?;

            // EOF
            if n == 0 {
                println!("ERROR: premature end of file {}", fname);
                break;
            }
            buf = &data[..n];
        }
        match decoder.update(buf, &mut Vec::new()) {
            Ok((_, ImageEnd)) => {
                if !have_idat {
                    // This isn't beautiful. But it works.
                    display_error(png::DecodingError::IoError(io::Error::new(
                        io::ErrorKind::InvalidData,
                        "IDAT chunk missing",
                    )))?;
                    break;
                }
                if !c.verbose && !c.quiet {
                    if c.color {
                        t.fg(color::GREEN)?;
                        t.attr(Attr::Bold)?;
                        write!(t, "OK")?;
                        t.reset()?;
                        write!(t, ": ")?;
                        t.fg(color::YELLOW)?;
                        write!(t, "{}", fname)?;
                        t.reset()?;
                    } else {
                        print!("OK: {}", fname)
                    }
                    println!(
                        " ({}x{}, {}{}, {}, {:.1}%)",
                        width,
                        height,
                        display_image_type(bits, color),
                        (if trns { "+trns" } else { "" }),
                        display_interlaced(interlaced),
                        100.0 * (1.0 - c_ratio!())
                    )
                } else if !c.quiet {
                    println!();
                    if c.color {
                        t.fg(color::GREEN)?;
                        t.attr(Attr::Bold)?;
                        write!(t, "No errors detected ")?;
                        t.reset()?;
                    } else {
                        print!("No errors detected ");
                    }
                    println!(
                        "in {} ({} chunks, {:.1}% compression)",
                        fname,
                        n_chunks,
                        100.0 * (1.0 - c_ratio!()),
                    )
                }
                break;
            }
            Ok((n, res)) => {
                buf = &buf[n..];
                pos += n;
                match res {
                    Header(w, h, b, c, i) => {
                        width = w;
                        height = h;
                        bits = b as u8;
                        color = c;
                        interlaced = i;
                    }
                    ChunkBegin(len, type_str) => {
                        use png::chunk;
                        n_chunks += 1;
                        if c.verbose {
                            let chunk = type_str;
                            println!();
                            print!("  chunk ");
                            if c.color {
                                t.fg(color::YELLOW)?;
                                write!(t, "{:?}", chunk)?;
                                t.reset()?;
                            } else {
                                print!("{:?}", chunk)
                            }
                            print!(
                                " at offset {:#07x}, length {}",
                                pos - 4, // subtract chunk name length
                                len
                            )
                        }
                        match type_str {
                            chunk::IDAT => {
                                have_idat = true;
                                compressed_size += len
                            }
                            chunk::tRNS => {
                                trns = true;
                            }
                            _ => (),
                        }
                    }
                    ImageData => {
                        //println!("got {} bytes of image data", data.len())
                    }
                    ChunkComplete(_, type_str) if c.verbose => {
                        use png::chunk::*;
                        if type_str == IHDR {
                            println!();
                            print!(
                                "    {} x {} image, {}{}, {}",
                                width,
                                height,
                                display_image_type(bits, color),
                                (if trns { "+trns" } else { "" }),
                                display_interlaced(interlaced),
                            );
                        }
                    }
                    AnimationControl(actl) => {
                        println!();
                        print!("    {} frames, {} plays", actl.num_frames, actl.num_plays,);
                    }
                    FrameControl(fctl) => {
                        println!();
                        println!(
                            "    sequence #{}, {} x {} pixels @ ({}, {})",
                            fctl.sequence_number,
                            fctl.width,
                            fctl.height,
                            fctl.x_offset,
                            fctl.y_offset,
                            /*fctl.delay_num,
                            fctl.delay_den,
                            fctl.dispose_op,
                            fctl.blend_op,*/
                        );
                        print!(
                            "    {}/{} s delay, dispose: {}, blend: {}",
                            fctl.delay_num,
                            if fctl.delay_den == 0 {
                                100
                            } else {
                                fctl.delay_den
                            },
                            fctl.dispose_op,
                            fctl.blend_op,
                        );
                    }
                    _ => (),
                }
                //println!("{} {:?}", n, res)
            }
            Err(err) => {
                let _ = display_error(err);
                break;
            }
        }
    }
    if c.text {
        println!("Parsed tEXt chunks:");
        for text_chunk in &decoder.info().unwrap().uncompressed_latin1_text {
            println!("{:#?}", text_chunk);
        }

        println!("Parsed zTXt chunks:");
        for text_chunk in &decoder.info().unwrap().compressed_latin1_text {
            let mut cloned_text_chunk = text_chunk.clone();
            cloned_text_chunk.decompress_text()?;
            println!("{:#?}", cloned_text_chunk);
        }

        println!("Parsed iTXt chunks:");
        for text_chunk in &decoder.info().unwrap().utf8_text {
            let mut cloned_text_chunk = text_chunk.clone();
            cloned_text_chunk.decompress_text()?;
            println!("{:#?}", cloned_text_chunk);
        }
    }

    Ok(())
}

fn main() {
    let m = parse_args();

    let config = Config {
        quiet: m.opt_present("q"),
        verbose: m.opt_present("v"),
        color: m.opt_present("c"),
        text: m.opt_present("t"),
    };

    for file in m.free {
        let result = if file.contains('*') {
            glob::glob(&file)
                .map_err(|err| io::Error::new(io::ErrorKind::Other, err))
                .and_then(|mut glob| {
                    glob.try_for_each(|entry| {
                        entry
                            .map_err(|err| io::Error::new(io::ErrorKind::Other, err))
                            .and_then(|file| check_image(config, file))
                    })
                })
        } else {
            check_image(config, &file)
        };

        result.unwrap_or_else(|err| {
            println!("{}: {}", file, err);
            std::process::exit(1)
        });
    }
}
