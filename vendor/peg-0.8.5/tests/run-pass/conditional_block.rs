extern crate peg;

peg::parser!( grammar parse() for str {

    pub rule dec_byte() -> u8
        = match_str:$(['0'..='9']*<,3>) {?
            let val: u64 = match_str.parse().unwrap();

            // only let this rule match if the value is in range 0..255
            if val <= 255 {
                Ok(val as u8)
            } else {
                // the message explains what the rule expected and is used in the parse error
                Err("decimal byte")
            }
        }

    rule tag() -> &'input str
        = $(['a'..='z']+)

    pub rule xml()
        = "<" open:tag() ">" xml()* "</" close:tag() ">" {?
            if open == close {
                Ok(())
            } else {
                // TODO this has to be a `&'static str`, so we can't use a dynamic string
                Err("matching close tag")
            }
        }

    pub rule return_early() -> i32 = vs:$([_]+) {?
        let v = vs.parse::<i32>().map_err(|_| "number")?;
        if v > 100 {
            return Err("smaller number");
        }
        Ok(v)
    }
});

fn main() {
    assert_eq!(parse::dec_byte("0"), Ok(0));
    assert_eq!(parse::dec_byte("255"), Ok(255));
    assert_eq!(parse::dec_byte("1"), Ok(1));
    assert!(parse::dec_byte("256").is_err());
    assert!(parse::dec_byte("1234").is_err());

    assert!(parse::xml("<a></a>").is_ok());
    assert!(parse::xml("<a><b></b><c></c></a>").is_ok());
    assert!(parse::xml("<a><b><c></b></c></a>").is_err());
    assert!(parse::xml("<a><b></c><c></b></a>").is_err());

    assert!(parse::return_early("a").unwrap_err().expected.tokens().any(|e| e == "number"));
    assert!(parse::return_early("123").unwrap_err().expected.tokens().any(|e| e == "smaller number"));
    assert_eq!(parse::return_early("99").unwrap(), 99);
}
