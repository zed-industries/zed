use core_foundation::{
    array::CFArray,
    base::{TCFType, ToVoid},
    dictionary::CFDictionary,
    propertylist::CFPropertyList,
    string::{CFString, CFStringRef},
};
use system_configuration::{
    dynamic_store::{SCDynamicStore, SCDynamicStoreBuilder},
    sys::schema_definitions::{kSCDynamicStorePropNetPrimaryService, kSCPropNetDNSServerAddresses},
};

// This example will change the DNS settings on the primary
// network interface to 8.8.8.8 and 8.8.4.4

fn main() {
    let store = SCDynamicStoreBuilder::new("my-test-dyn-store").build();
    let primary_service_uuid = get_primary_service_uuid(&store).expect("No PrimaryService active");
    println!("PrimaryService UUID: {}", primary_service_uuid);

    let primary_service_path = CFString::new(&format!(
        "State:/Network/Service/{}/DNS",
        primary_service_uuid
    ));
    println!("PrimaryService path: {}", primary_service_path);

    let dns_dictionary = create_dns_dictionary(&[
        CFString::from_static_string("8.8.8.8"),
        CFString::from_static_string("8.8.4.4"),
    ]);

    let success = store.set(primary_service_path, dns_dictionary);
    println!("success? {}", success);
}

fn get_primary_service_uuid(store: &SCDynamicStore) -> Option<CFString> {
    let dictionary = store
        .get("State:/Network/Global/IPv4")
        .and_then(CFPropertyList::downcast_into::<CFDictionary>)?;
    dictionary
        .find(unsafe { kSCDynamicStorePropNetPrimaryService }.to_void())
        .map(|ptr| unsafe { CFString::wrap_under_get_rule(*ptr as CFStringRef) })
}

fn create_dns_dictionary(addresses: &[CFString]) -> CFDictionary {
    let key = unsafe { CFString::wrap_under_get_rule(kSCPropNetDNSServerAddresses) };
    let value = CFArray::from_CFTypes(addresses);
    let typed_dict = CFDictionary::from_CFType_pairs(&[(key, value)]);
    unsafe { CFDictionary::wrap_under_get_rule(typed_dict.as_concrete_TypeRef()) }
}
