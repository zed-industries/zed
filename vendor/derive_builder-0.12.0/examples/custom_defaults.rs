#[macro_use]
extern crate derive_builder;

#[derive(Builder, PartialEq, Debug)]
struct Lorem {
    ipsum: String,
    #[builder(default = "self.default_dolor()?")]
    dolor: String,
    #[builder(default = "self.default_sit()?")]
    sit: String,
    #[builder(setter(skip), default = "self.default_amet()")]
    amet: String,
}

impl LoremBuilder {
    fn default_dolor(&self) -> Result<String, String> {
        self.ipsum
            .clone()
            .ok_or_else(|| "ipsum must be initialized to build dolor".to_string())
    }

    fn default_sit(&self) -> Result<String, String> {
        match self.ipsum {
            Some(ref x) if x.chars().count() > 3 => Ok(format!("sit {}", x)),
            _ => Err("ipsum must at least 3 chars to build sit".to_string()),
        }
    }

    fn default_amet(&self) -> String {
        if let Some(ref x) = self.ipsum {
            format!("amet {}", x)
        } else {
            "..nothing there".to_string()
        }
    }
}

fn main() {
    let x = LoremBuilder::default()
        .ipsum("ipsum".to_string())
        .build()
        .unwrap();

    assert_eq!(
        x,
        Lorem {
            ipsum: "ipsum".to_string(),
            dolor: "ipsum".to_string(),
            sit: "sit ipsum".to_string(),
            amet: "amet ipsum".to_string(),
        }
    );
}
