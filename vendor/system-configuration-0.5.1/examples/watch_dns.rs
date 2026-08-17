use core_foundation::{
    array::CFArray,
    base::{CFType, TCFType, ToVoid},
    dictionary::CFDictionary,
    propertylist::CFPropertyList,
    runloop::{kCFRunLoopCommonModes, CFRunLoop},
    string::CFString,
};
use system_configuration::{
    dynamic_store::{SCDynamicStore, SCDynamicStoreBuilder, SCDynamicStoreCallBackContext},
    sys::schema_definitions::kSCPropNetDNSServerAddresses,
};

// This example will watch the dynamic store for changes to any DNS setting. As soon as a change
// is detected, it will be printed to stdout.

fn main() {
    let callback_context = SCDynamicStoreCallBackContext {
        callout: my_callback,
        info: Context { call_count: 0 },
    };

    let store = SCDynamicStoreBuilder::new("my-watch-dns-store")
        .callback_context(callback_context)
        .build();

    let watch_keys: CFArray<CFString> = CFArray::from_CFTypes(&[]);
    let watch_patterns =
        CFArray::from_CFTypes(&[CFString::from("(State|Setup):/Network/Service/.*/DNS")]);

    if store.set_notification_keys(&watch_keys, &watch_patterns) {
        println!("Registered for notifications");
    } else {
        panic!("Unable to register notifications");
    }

    let run_loop_source = store.create_run_loop_source();
    let run_loop = CFRunLoop::get_current();
    run_loop.add_source(&run_loop_source, unsafe { kCFRunLoopCommonModes });

    println!("Entering run loop");
    CFRunLoop::run_current();
}

/// This struct acts as a user provided context/payload to each notification callback.
/// Here one can store any type of data or state needed in the callback function.
#[derive(Debug)]
struct Context {
    call_count: u64,
}

#[allow(clippy::needless_pass_by_value)]
fn my_callback(store: SCDynamicStore, changed_keys: CFArray<CFString>, context: &mut Context) {
    context.call_count += 1;
    println!("Callback call count: {}", context.call_count);

    for key in changed_keys.iter() {
        if let Some(addresses) = get_dns(&store, key.clone()) {
            println!("{} changed DNS to {:?}", *key, addresses);
        } else {
            println!("{} removed DNS", *key);
        }
    }
}

fn get_dns(store: &SCDynamicStore, path: CFString) -> Option<Vec<String>> {
    let dns_settings = store
        .get(path)
        .and_then(CFPropertyList::downcast_into::<CFDictionary>)?;
    let address_array = dns_settings
        .find(unsafe { kSCPropNetDNSServerAddresses }.to_void())
        .map(|ptr| unsafe { CFType::wrap_under_get_rule(*ptr) })
        .and_then(CFType::downcast_into::<CFArray>)?;
    let mut result = Vec::with_capacity(address_array.len() as usize);
    for address_ptr in &address_array {
        let address =
            unsafe { CFType::wrap_under_get_rule(*address_ptr) }.downcast_into::<CFString>()?;
        result.push(address.to_string())
    }
    Some(result)
}
