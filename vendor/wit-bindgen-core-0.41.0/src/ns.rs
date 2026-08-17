use std::collections::HashSet;

#[derive(Default)]
pub struct Ns {
    defined: HashSet<String>,
    tmp: usize,
}

impl Ns {
    pub fn insert(&mut self, name: &str) -> Result<(), String> {
        if self.defined.insert(name.to_string()) {
            Ok(())
        } else {
            Err(format!("name `{}` already defined", name))
        }
    }

    pub fn tmp(&mut self, name: &str) -> String {
        let mut ret = name.to_string();
        while self.defined.contains(&ret) {
            ret = format!("{}{}", name, self.tmp);
            self.tmp += 1;
        }
        self.defined.insert(ret.clone());
        ret
    }
}
