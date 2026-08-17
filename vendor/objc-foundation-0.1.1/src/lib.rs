#![crate_name = "objc_foundation"]
#![crate_type = "lib"]

#[macro_use]
extern crate objc;
extern crate objc_id;
extern crate block;

pub use self::array::{
    INSArray, INSMutableArray,
    NSArray, NSComparisonResult, NSMutableArray, NSRange,
    NSMutableSharedArray, NSSharedArray,
};
pub use self::data::{INSData, INSMutableData, NSData, NSMutableData};
pub use self::dictionary::{INSDictionary, NSDictionary};
pub use self::enumerator::{INSFastEnumeration, NSEnumerator, NSFastEnumerator};
pub use self::object::{INSObject, NSObject};
pub use self::string::{INSCopying, INSMutableCopying, INSString, NSString};
pub use self::value::{INSValue, NSValue};

#[link(name = "Foundation", kind = "framework")]
extern { }

#[macro_use]
mod macros;

mod array;
mod data;
mod dictionary;
mod enumerator;
mod object;
mod string;
mod value;
