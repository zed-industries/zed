extern "C" {
    #[cfg(feature = "NSMapTable")]
    pub fn NSFreeMapTable(table: *mut crate::Foundation::NSMapTable);
}

// TODO: Add `-[NSKeyedUnarchiverDelegate unarchiver:didDecodeObject:]`
