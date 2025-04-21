use std::rc::Rc;

use crate::thread::EvalThread;

mod file_search;

pub fn all() -> Vec<Rc<dyn EvalThread>> {
    vec![Rc::new(file_search::Thread)]
}
