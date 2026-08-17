extern crate objc_test_utils;

use {Block, RcBlock};

pub fn get_int_block_with(i: i32) -> RcBlock<(), i32> {
    unsafe {
        let ptr = objc_test_utils::get_int_block_with(i);
        RcBlock::new(ptr as *mut _)
    }
}

pub fn get_add_block_with(i: i32) -> RcBlock<(i32,), i32> {
    unsafe {
        let ptr = objc_test_utils::get_add_block_with(i);
        RcBlock::new(ptr as *mut _)
    }
}

pub fn invoke_int_block(block: &Block<(), i32>) -> i32 {
    let ptr = block as *const _;
    unsafe {
        objc_test_utils::invoke_int_block(ptr as *mut _)
    }
}

pub fn invoke_add_block(block: &Block<(i32,), i32>, a: i32) -> i32 {
    let ptr = block as *const _;
    unsafe {
        objc_test_utils::invoke_add_block(ptr as *mut _, a)
    }
}
