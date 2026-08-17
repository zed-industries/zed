// Check that `!Send` types fail early.

/** ```compile_fail,E0277

use rayon::prelude::*;
use std::ptr::null;

#[derive(Copy, Clone)]
struct NoSend(*const ());

unsafe impl Sync for NoSend {}

let x = Some(NoSend(null()));

x.par_iter()
    .map(|&x| x) //~ ERROR
    .count(); //~ ERROR

``` */
mod map {}

/** ```compile_fail,E0277

use rayon::prelude::*;
use std::ptr::null;

#[derive(Copy, Clone)]
struct NoSend(*const ());

unsafe impl Sync for NoSend {}

let x = Some(NoSend(null()));

x.par_iter()
    .filter_map(|&x| Some(x)) //~ ERROR
    .count(); //~ ERROR

``` */
mod filter_map {}

/** ```compile_fail,E0277

use rayon::prelude::*;
use std::ptr::null;

#[derive(Copy, Clone)]
struct NoSend(*const ());

unsafe impl Sync for NoSend {}

let x = Some(NoSend(null()));

x.par_iter()
    .cloned() //~ ERROR
    .count(); //~ ERROR

``` */
mod cloned {}
