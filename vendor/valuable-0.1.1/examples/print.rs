use valuable::{NamedValues, Valuable, Value, Visit};

struct Print(String);

impl Print {
    fn indent(&self) -> Print {
        Print(format!("{}    ", self.0))
    }
}

impl Visit for Print {
    fn visit_value(&mut self, value: Value<'_>) {
        match value {
            Value::Structable(v) => {
                let def = v.definition();
                // Print the struct name
                println!("{}{}:", self.0, def.name());

                // Visit fields
                let mut visit = self.indent();
                v.visit(&mut visit);
            }
            Value::Enumerable(v) => {
                let def = v.definition();
                let variant = v.variant();
                // Print the enum name
                println!("{}{}::{}:", self.0, def.name(), variant.name());

                // Visit fields
                let mut visit = self.indent();
                v.visit(&mut visit);
            }
            Value::Listable(v) => {
                println!("{}", self.0);

                // Visit fields
                let mut visit = self.indent();
                v.visit(&mut visit);
            }
            Value::Mappable(v) => {
                println!("{}", self.0);

                // Visit fields
                let mut visit = self.indent();
                v.visit(&mut visit);
            }
            // Primitive or unknown type, just render Debug
            v => println!("{:?}", v),
        }
    }

    fn visit_named_fields(&mut self, named_values: &NamedValues<'_>) {
        for (field, value) in named_values {
            print!("{}- {}: ", self.0, field.name());
            value.visit(self);
        }
    }

    fn visit_unnamed_fields(&mut self, values: &[Value<'_>]) {
        for value in values {
            print!("{}- ", self.0);
            value.visit(self);
        }
    }

    fn visit_entry(&mut self, key: Value<'_>, value: Value<'_>) {
        print!("{}- {:?}: ", self.0, key);
        value.visit(self);
    }
}

#[derive(Valuable)]
struct Person {
    name: String,
    age: u32,
    addresses: Vec<Address>,
}

#[derive(Valuable)]
struct Address {
    street: String,
    city: String,
    zip: String,
}

fn main() {
    let person = Person {
        name: "Angela Ashton".to_string(),
        age: 31,
        addresses: vec![
            Address {
                street: "123 1st Ave".to_string(),
                city: "Townsville".to_string(),
                zip: "12345".to_string(),
            },
            Address {
                street: "555 Main St.".to_string(),
                city: "New Old Town".to_string(),
                zip: "55555".to_string(),
            },
        ],
    };

    let mut print = Print("".to_string());
    valuable::visit(&person, &mut print);
}
