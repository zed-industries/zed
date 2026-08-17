#![allow(unused_imports)]
use alloc::borrow::ToOwned;

use objc2::rc::Retained;
use objc2::Message;

use crate::{NSCopying, NSMutableCopying};

#[cfg(feature = "NSArray")]
impl<ObjectType: Message> ToOwned for crate::NSArray<ObjectType> {
    type Owned = Retained<Self>;
    fn to_owned(&self) -> Self::Owned {
        self.copy()
    }
}

#[cfg(feature = "NSArray")]
impl<ObjectType: Message> ToOwned for crate::NSMutableArray<ObjectType> {
    type Owned = Retained<Self>;
    fn to_owned(&self) -> Self::Owned {
        self.mutableCopy()
    }
}

#[cfg(feature = "NSData")]
impl ToOwned for crate::NSData {
    type Owned = Retained<Self>;
    fn to_owned(&self) -> Self::Owned {
        self.copy()
    }
}

#[cfg(feature = "NSData")]
impl ToOwned for crate::NSMutableData {
    type Owned = Retained<Self>;
    fn to_owned(&self) -> Self::Owned {
        self.mutableCopy()
    }
}

#[cfg(feature = "NSException")]
impl ToOwned for crate::NSException {
    type Owned = Retained<Self>;
    fn to_owned(&self) -> Self::Owned {
        self.copy()
    }
}

#[cfg(feature = "NSSet")]
impl<ObjectType: Message> ToOwned for crate::NSSet<ObjectType> {
    type Owned = Retained<Self>;
    fn to_owned(&self) -> Self::Owned {
        self.copy()
    }
}

#[cfg(feature = "NSSet")]
impl<ObjectType: Message> ToOwned for crate::NSMutableSet<ObjectType> {
    type Owned = Retained<Self>;
    fn to_owned(&self) -> Self::Owned {
        self.mutableCopy()
    }
}

#[cfg(feature = "NSString")]
impl ToOwned for crate::NSString {
    type Owned = Retained<Self>;
    fn to_owned(&self) -> Self::Owned {
        self.copy()
    }
}

#[cfg(feature = "NSString")]
impl ToOwned for crate::NSMutableString {
    type Owned = Retained<Self>;
    fn to_owned(&self) -> Self::Owned {
        self.mutableCopy()
    }
}

#[cfg(feature = "NSAttributedString")]
impl ToOwned for crate::NSAttributedString {
    type Owned = Retained<Self>;
    fn to_owned(&self) -> Self::Owned {
        self.copy()
    }
}

#[cfg(feature = "NSAttributedString")]
impl ToOwned for crate::NSMutableAttributedString {
    type Owned = Retained<Self>;
    fn to_owned(&self) -> Self::Owned {
        self.mutableCopy()
    }
}

#[cfg(feature = "NSUUID")]
impl ToOwned for crate::NSUUID {
    type Owned = Retained<Self>;
    fn to_owned(&self) -> Self::Owned {
        self.copy()
    }
}

#[cfg(feature = "NSValue")]
impl ToOwned for crate::NSValue {
    type Owned = Retained<Self>;
    fn to_owned(&self) -> Self::Owned {
        self.copy()
    }
}

#[cfg(feature = "NSValue")]
impl ToOwned for crate::NSNumber {
    type Owned = Retained<Self>;
    fn to_owned(&self) -> Self::Owned {
        self.copy()
    }
}
