use crate::algorithm::Printer;
use syn::File;

impl Printer {
    pub fn file(&mut self, file: &File) {
        self.cbox(0);
        if let Some(shebang) = &file.shebang {
            self.word(shebang.clone());
            self.hardbreak();
        }
        self.inner_attrs(&file.attrs);
        for item in &file.items {
            self.item(item);
        }
        self.end();
    }
}
