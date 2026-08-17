// Take a look at the license at the top of the repository in the LICENSE file.

// This was the OSX-based solution. It provides enough information, but what a mess!
// pub fn get_users_list() -> Vec<User> {
//     let mut users = Vec::new();
//     let node_name = b"/Local/Default\0";

//     unsafe {
//         let node_name = ffi::CFStringCreateWithCStringNoCopy(
//             std::ptr::null_mut(),
//             node_name.as_ptr() as *const c_char,
//             ffi::kCFStringEncodingMacRoman,
//             ffi::kCFAllocatorNull as *mut c_void,
//         );
//         let node_ref = ffi::ODNodeCreateWithName(
//             ffi::kCFAllocatorDefault,
//             ffi::kODSessionDefault,
//             node_name,
//             std::ptr::null_mut(),
//         );
//         let query = ffi::ODQueryCreateWithNode(
//             ffi::kCFAllocatorDefault,
//             node_ref,
//             ffi::kODRecordTypeUsers as _, // kODRecordTypeGroups
//             std::ptr::null(),
//             0,
//             std::ptr::null(),
//             std::ptr::null(),
//             0,
//             std::ptr::null_mut(),
//         );
//         if query.is_null() {
//             return users;
//         }
//         let results = ffi::ODQueryCopyResults(
//             query,
//             false as _,
//             std::ptr::null_mut(),
//         );
//         let len = ffi::CFArrayGetCount(results);
//         for i in 0..len {
//             let name = match get_user_name(ffi::CFArrayGetValueAtIndex(results, i)) {
//                 Some(n) => n,
//                 None => continue,
//             };
//             users.push(User { name });
//         }

//         ffi::CFRelease(results as *const c_void);
//         ffi::CFRelease(query as *const c_void);
//         ffi::CFRelease(node_ref as *const c_void);
//         ffi::CFRelease(node_name as *const c_void);
//     }
//     users.sort_unstable_by(|x, y| x.name.partial_cmp(&y.name).unwrap());
//     return users;
// }

// fn get_user_name(result: *const c_void) -> Option<String> {
//     let user_name = ffi::ODRecordGetRecordName(result as _);
//     let ptr = ffi::CFStringGetCharactersPtr(user_name);
//     String::from_utf16(&if ptr.is_null() {
//         let len = ffi::CFStringGetLength(user_name); // It returns the len in UTF-16 code pairs.
//         if len == 0 {
//             continue;
//         }
//         let mut v = Vec::with_capacity(len as _);
//         for x in 0..len {
//             v.push(ffi::CFStringGetCharacterAtIndex(user_name, x));
//         }
//         v
//     } else {
//         let mut v: Vec<u16> = Vec::new();
//         let mut x = 0;
//         loop {
//             let letter = *ptr.offset(x);
//             if letter == 0 {
//                 break;
//             }
//             v.push(letter);
//             x += 1;
//         }
//         v
//     }.ok()
// }
