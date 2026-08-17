use crate::{Config, PrintTermcolor, Printer};
use wasmparser::{
    BinaryReader, Frame, OperatorsReader, Payload, Result, ValType, ValidPayload,
    ValidatorResources, WasmFeatures,
};

pub struct Validator {
    validator: wasmparser::Validator,
    last_function: Option<FuncValidator>,
}

impl Validator {
    pub fn new() -> Option<Validator> {
        Some(Validator {
            validator: wasmparser::Validator::new_with_features(WasmFeatures::all()),
            last_function: None,
        })
    }

    pub fn payload(&mut self, payload: &Payload<'_>) -> Result<()> {
        match self.validator.payload(payload)? {
            ValidPayload::Ok | ValidPayload::End(_) | ValidPayload::Parser(_) => Ok(()),
            ValidPayload::Func(validator, _) => {
                assert!(self.last_function.is_none());
                self.last_function = Some(FuncValidator {
                    push_count: 0,
                    validator: validator.into_validator(Default::default()),
                });
                Ok(())
            }
        }
    }

    pub fn next_func(&mut self) -> Option<FuncValidator> {
        Some(self.last_function.take().unwrap())
    }
}

pub struct FuncValidator {
    validator: wasmparser::FuncValidator<ValidatorResources>,
    push_count: u32,
}

impl FuncValidator {
    pub fn read_locals(&mut self, mut reader: BinaryReader<'_>) -> Result<()> {
        self.validator.read_locals(&mut reader)
    }

    pub fn visit_operator(&mut self, reader: &OperatorsReader<'_>, is_end: bool) -> Result<()> {
        let pos = reader.original_position();
        reader
            .clone()
            .visit_operator(&mut self.validator.visitor(pos))??;

        if !is_end {
            let op = reader.clone().read()?;
            let arity = op.operator_arity(&self.validator.visitor(pos));
            if let Some((_pop_count, push_count)) = arity {
                self.push_count = push_count;
            }
        }
        Ok(())
    }

    pub fn visualize_operand_stack(&self, use_color: bool) -> crate::Result<String> {
        let mut buf_color = PrintTermcolor(termcolor::Ansi::new(Vec::new()));
        let mut buf_nocolor = PrintTermcolor(termcolor::NoColor::new(Vec::new()));

        let stack_printer = Printer {
            result: if use_color {
                &mut buf_color
            } else {
                &mut buf_nocolor
            },
            config: &Config::new(),
            nesting: 0,
            line: 0,
            group_lines: Vec::new(),
            code_section_hints: Vec::new(),
        };

        if let Some(&Frame { height, .. }) = self.validator.get_control_frame(0) {
            stack_printer.result.start_comment()?;
            stack_printer.result.write_str("[")?;
            let max_height = self.validator.operand_stack_height() as usize;
            for depth in (height..max_height).rev() {
                if depth + 1 < max_height {
                    stack_printer.result.write_str(" ")?;
                }
                if depth + 1 == height + self.push_count as usize {
                    stack_printer.result.start_type()?;
                }
                match self.validator.get_operand_type(depth) {
                    Some(Some(ty)) => {
                        stack_printer.result.write_str(&ty_to_str(ty))?;
                    }
                    Some(None) => {
                        stack_printer.result.write_str("(unknown)")?;
                    }
                    None => {
                        stack_printer.result.write_str("(invalid)")?;
                    }
                }
            }
            stack_printer.result.start_comment()?;
            stack_printer.result.write_str("]")?;
            stack_printer.result.reset_color()?;
        }

        let ret = String::from_utf8(if use_color {
            buf_color.0.into_inner()
        } else {
            buf_nocolor.0.into_inner()
        })?;
        return Ok(ret);

        fn ty_to_str(ty: ValType) -> String {
            match ty {
                ValType::I32 => String::from("i32"),
                ValType::I64 => String::from("i64"),
                ValType::F32 => String::from("f32"),
                ValType::F64 => String::from("f64"),
                ValType::V128 => String::from("v128"),
                ValType::Ref(r) => format!("{r}"),
            }
        }
    }
}
