#[macro_use]
extern crate derive_builder;

#[derive(Debug, Builder, PartialEq)]
#[builder(build_fn(skip))]
pub struct Lorem {
    percentile: u8,
}

#[derive(Debug, Builder, PartialEq)]
#[builder(build_fn(name = "finish"))]
pub struct Ipsum {
    percentile: u8,
}

impl Lorem {
    pub fn new(pct: u8) -> Result<Self, String> {
        if pct <= 100 {
            Ok(Lorem { percentile: pct })
        } else {
            Err(format!("Percentile must be between 0 and 100; was {}", pct))
        }
    }
}

impl LoremBuilder {
    pub fn build(&self) -> Result<Lorem, String> {
        if let Some(ref pct) = self.percentile {
            Lorem::new(*pct)
        } else {
            Err("Percentile was not initialized".to_string())
        }
    }
}

impl IpsumBuilder {
    /// This should be fine, because we renamed the generated build_fn.
    #[allow(dead_code)]
    fn build(&self) -> Result<Self, String> {
        unimplemented!()
    }
}

#[test]
fn happy_path() {
    let lorem = LoremBuilder::default().percentile(80).build().unwrap();
    assert_eq!(lorem, Lorem { percentile: 80 });
}

#[test]
fn uninitialized() {
    let lorem_err = LoremBuilder::default().build().unwrap_err();
    assert_eq!("Percentile was not initialized", &lorem_err);
}

#[test]
fn out_of_range() {
    let lorem_err = LoremBuilder::default().percentile(120).build().unwrap_err();
    assert_eq!("Percentile must be between 0 and 100; was 120", &lorem_err);
}

#[test]
fn rename() {
    let ipsum = IpsumBuilder::default().percentile(110).finish().unwrap();
    assert_eq!(Ipsum { percentile: 110 }, ipsum);
}
