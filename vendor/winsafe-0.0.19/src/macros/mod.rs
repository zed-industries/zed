//! All macros used throughout the library are declared in this module, which is
//! declared before everything in lib.rs. This is necessary because macros must
//! exist before a module declaration to be used inside of the module.

#[macro_use] mod com;
#[macro_use] mod consts;
#[macro_use] mod ffis;
#[macro_use] mod gui_events;
#[macro_use] mod handles;
#[macro_use] mod messages;
#[macro_use] mod seq_ids;
#[macro_use] mod structs;
