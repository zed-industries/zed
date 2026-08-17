//! Creates a task-local value.

use std::cell::Cell;

use async_std::prelude::*;
use async_std::task;

task_local! {
    static VAR: Cell<i32> = Cell::new(1);
}

fn main() {
    task::block_on(async {
        println!("var = {}", VAR.with(|v| v.get()));
        VAR.with(|v| v.set(2));
        println!("var = {}", VAR.with(|v| v.get()));
    })
}
