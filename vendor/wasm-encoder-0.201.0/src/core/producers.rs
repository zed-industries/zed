use std::borrow::Cow;

use crate::{CustomSection, Encode, Section, SectionId};

/// An encoder for the [producers custom
/// section](https://github.com/WebAssembly/tool-conventions/blob/main/ProducersSection.md).
///
/// This section is a non-standard convention that is supported by many toolchains.
///
/// # Example
///
/// ```
/// use wasm_encoder::{ProducersSection, ProducersField, Module};
///
/// // Create a new producers section.
/// let mut field = ProducersField::new();
/// field.value("clang", "14.0.4");
/// field.value("rustc", "1.66.1 (90743e729 2023-01-10)");
/// let mut producers = ProducersSection::new();
/// producers.field("processed-by", &field);
///
/// // Add the producers section to a new Wasm module and get the encoded bytes.
/// let mut module = Module::new();
/// module.section(&producers);
/// let wasm_bytes = module.finish();
/// ```
#[derive(Clone, Debug)]
pub struct ProducersSection {
    bytes: Vec<u8>,
    num_fields: u32,
}

impl ProducersSection {
    /// Construct an empty encoder for the producers custom section.
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a field to the section. The spec recommends names for this section
    /// are "language", "processed-by", and "sdk".  Each field in section must
    /// have a unique name.
    pub fn field(&mut self, name: &str, values: &ProducersField) -> &mut Self {
        name.encode(&mut self.bytes);
        values.encode(&mut self.bytes);
        self.num_fields += 1;
        self
    }
}

impl Default for ProducersSection {
    fn default() -> Self {
        Self {
            bytes: Vec::new(),
            num_fields: 0,
        }
    }
}

impl Encode for ProducersSection {
    fn encode(&self, sink: &mut Vec<u8>) {
        let mut data = Vec::new();
        self.num_fields.encode(&mut data);
        data.extend(&self.bytes);

        CustomSection {
            name: "producers".into(),
            data: Cow::Borrowed(&data),
        }
        .encode(sink);
    }
}

impl Section for ProducersSection {
    fn id(&self) -> u8 {
        SectionId::Custom.into()
    }
}

/// The value of a field in the producers custom section
#[derive(Clone, Debug)]
pub struct ProducersField {
    bytes: Vec<u8>,
    num_values: u32,
}

impl ProducersField {
    /// Construct an empty encoder for a producers field value
    pub fn new() -> Self {
        ProducersField::default()
    }

    /// Add a value to the field encoder. Each value in a field must have a
    /// unique name. If there is no sensible value for `version`, use the
    /// empty string.
    pub fn value(&mut self, name: &str, version: &str) -> &mut Self {
        name.encode(&mut self.bytes);
        version.encode(&mut self.bytes);
        self.num_values += 1;
        self
    }
}

impl Default for ProducersField {
    fn default() -> Self {
        Self {
            bytes: Vec::new(),
            num_values: 0,
        }
    }
}

impl Encode for ProducersField {
    fn encode(&self, sink: &mut Vec<u8>) {
        self.num_values.encode(sink);
        sink.extend(&self.bytes);
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn roundtrip_example() {
        use crate::{Module, ProducersField, ProducersSection};
        use wasmparser::{Parser, Payload, ProducersSectionReader};

        // Create a new producers section.
        let mut field = ProducersField::new();
        field.value("clang", "14.0.4");
        field.value("rustc", "1.66.1");
        let mut producers = ProducersSection::new();
        producers.field("processed-by", &field);

        // Add the producers section to a new Wasm module and get the encoded bytes.
        let mut module = Module::new();
        module.section(&producers);
        let wasm_bytes = module.finish();

        let mut parser = Parser::new(0).parse_all(&wasm_bytes);
        let payload = parser
            .next()
            .expect("parser is not empty")
            .expect("element is a payload");
        match payload {
            Payload::Version { .. } => {}
            _ => panic!(""),
        }
        let payload = parser
            .next()
            .expect("parser is not empty")
            .expect("element is a payload");
        match payload {
            Payload::CustomSection(c) => {
                assert_eq!(c.name(), "producers");
                let mut section = ProducersSectionReader::new(c.data(), c.data_offset())
                    .expect("readable as a producers section")
                    .into_iter();
                let field = section
                    .next()
                    .expect("section has an element")
                    .expect("element is a producers field");
                assert_eq!(field.name, "processed-by");
                let mut values = field.values.into_iter();
                let value = values
                    .next()
                    .expect("values has an element")
                    .expect("element is a producers field value");
                assert_eq!(value.name, "clang");
                assert_eq!(value.version, "14.0.4");

                let value = values
                    .next()
                    .expect("values has another element")
                    .expect("element is a producers field value");
                assert_eq!(value.name, "rustc");
                assert_eq!(value.version, "1.66.1");
            }
            _ => panic!("unexpected payload"),
        }
    }
}
