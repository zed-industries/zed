use crate::mapping::Mapping;
use crate::value::{Number, Value};
use std::fmt::{self, Debug, Display};

impl Debug for Value {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Null => formatter.write_str("Null"),
            Value::Bool(boolean) => write!(formatter, "Bool({})", boolean),
            Value::Number(number) => write!(formatter, "Number({})", number),
            Value::String(string) => write!(formatter, "String({:?})", string),
            Value::Sequence(sequence) => {
                formatter.write_str("Sequence ")?;
                formatter.debug_list().entries(sequence).finish()
            }
            Value::Mapping(mapping) => Debug::fmt(mapping, formatter),
            Value::Tagged(tagged) => Debug::fmt(tagged, formatter),
        }
    }
}

struct DisplayNumber<'a>(&'a Number);

impl<'a> Debug for DisplayNumber<'a> {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        Display::fmt(self.0, formatter)
    }
}

impl Debug for Number {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "Number({})", self)
    }
}

impl Debug for Mapping {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("Mapping ")?;
        let mut debug = formatter.debug_map();
        for (k, v) in self {
            let tmp;
            debug.entry(
                match k {
                    Value::Bool(boolean) => boolean,
                    Value::Number(number) => {
                        tmp = DisplayNumber(number);
                        &tmp
                    }
                    Value::String(string) => string,
                    _ => k,
                },
                v,
            );
        }
        debug.finish()
    }
}
