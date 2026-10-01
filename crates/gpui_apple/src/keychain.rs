use objc2_core_foundation::{CFData, CFDictionary, CFRetained, CFString, CFType, kCFBooleanTrue};
use objc2_security::{
    SecItemAdd, SecItemCopyMatching, SecItemDelete, SecItemUpdate, errSecItemNotFound,
    errSecSuccess, errSecUserCanceled, kSecAttrAccount, kSecAttrServer, kSecClass,
    kSecClassInternetPassword, kSecReturnAttributes, kSecReturnData, kSecValueData,
};
use std::ptr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeychainOperation {
    Read,
    Update,
    Create,
    Delete,
}

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum KeychainError {
    #[error("keychain {operation:?} failed with OSStatus {status}")]
    Security {
        operation: KeychainOperation,
        status: i32,
    },
    #[error("invalid keychain response: {0}")]
    InvalidResponse(&'static str),
}

pub fn write_credentials(
    server: &str,
    account: &str,
    password: &[u8],
) -> Result<(), KeychainError> {
    let server = CFString::from_str(server);
    let account = CFString::from_str(account);
    let password = CFData::from_bytes(password);
    // SAFETY: Reading immutable constants exported by the Security framework.
    let (class_key, server_key, account_key, data_key, internet_password) = unsafe {
        (
            kSecClass,
            kSecAttrServer,
            kSecAttrAccount,
            kSecValueData,
            kSecClassInternetPassword,
        )
    };
    let query = CFDictionary::<CFString, CFType>::from_slices(
        &[class_key, server_key],
        &[internet_password, &server],
    );
    let attributes = CFDictionary::<CFString, CFType>::from_slices(
        &[account_key, data_key],
        &[&account, &password],
    );
    let mut operation = KeychainOperation::Update;
    // SAFETY: Both dictionaries map Security attribute keys to CF values.
    let mut status = unsafe { SecItemUpdate(query.as_opaque(), attributes.as_opaque()) };
    if status == errSecItemNotFound {
        operation = KeychainOperation::Create;
        let attributes = CFDictionary::<CFString, CFType>::from_slices(
            &[class_key, server_key, account_key, data_key],
            &[internet_password, &server, &account, &password],
        );
        // SAFETY: The dictionary maps Security attribute keys to CF values, and a
        // null result pointer asks Security not to return the new item.
        status = unsafe { SecItemAdd(attributes.as_opaque(), ptr::null_mut()) };
    }
    check_status(operation, status)
}

pub fn read_credentials(server: &str) -> Result<Option<(String, Vec<u8>)>, KeychainError> {
    let server = CFString::from_str(server);
    // SAFETY: Reading immutable constants exported by Core Foundation and Security.
    let (
        true_value,
        class_key,
        server_key,
        return_attributes_key,
        return_data_key,
        internet_password,
    ) = unsafe {
        (
            kCFBooleanTrue,
            kSecClass,
            kSecAttrServer,
            kSecReturnAttributes,
            kSecReturnData,
            kSecClassInternetPassword,
        )
    };
    let true_value = true_value.ok_or(KeychainError::InvalidResponse(
        "Core Foundation true value unavailable",
    ))?;
    let query = CFDictionary::<CFString, CFType>::from_slices(
        &[
            class_key,
            server_key,
            return_attributes_key,
            return_data_key,
        ],
        &[internet_password, &server, true_value, true_value],
    );
    let mut result = ptr::null();
    // SAFETY: The query maps Security attribute keys to CF values, and `result`
    // is a valid out-pointer.
    let status = unsafe { SecItemCopyMatching(query.as_opaque(), &mut result) };
    if !read_status(status)? {
        return Ok(None);
    }
    let result = ptr::NonNull::new(result.cast_mut())
        .ok_or(KeychainError::InvalidResponse("keychain returned no item"))?;
    // SAFETY: SecItemCopyMatching follows the Create rule, so we own this reference.
    let result = unsafe { CFRetained::from_raw(result) }
        .downcast::<CFDictionary>()
        .map_err(|_| KeychainError::InvalidResponse("keychain item was not a dictionary"))?;
    // SAFETY: Security returns CFType keys and values; each value is downcast below.
    let result = unsafe { result.cast_unchecked::<CFType, CFType>() };
    // SAFETY: Reading immutable constants exported by the Security framework.
    let (account_key, data_key) = unsafe { (kSecAttrAccount, kSecValueData) };
    let account = result
        .get(account_key)
        .ok_or(KeychainError::InvalidResponse(
            "account was missing from keychain item",
        ))?
        .downcast::<CFString>()
        .map_err(|_| KeychainError::InvalidResponse("account was not a string"))?;
    let password = result
        .get(data_key)
        .ok_or(KeychainError::InvalidResponse(
            "password was missing from keychain item",
        ))?
        .downcast::<CFData>()
        .map_err(|_| KeychainError::InvalidResponse("password was not data"))?;
    Ok(Some((account.to_string(), password.to_vec())))
}

pub fn delete_credentials(server: &str) -> Result<(), KeychainError> {
    let server = CFString::from_str(server);
    // SAFETY: Reading immutable constants exported by the Security framework.
    let (class_key, server_key, internet_password) =
        unsafe { (kSecClass, kSecAttrServer, kSecClassInternetPassword) };
    let query = CFDictionary::<CFString, CFType>::from_slices(
        &[class_key, server_key],
        &[internet_password, &server],
    );
    // SAFETY: The query maps Security attribute keys to CF values.
    let status = unsafe { SecItemDelete(query.as_opaque()) };
    check_status(KeychainOperation::Delete, status)
}

fn read_status(status: i32) -> Result<bool, KeychainError> {
    if status == errSecItemNotFound || status == errSecUserCanceled {
        Ok(false)
    } else {
        check_status(KeychainOperation::Read, status)?;
        Ok(true)
    }
}

fn check_status(operation: KeychainOperation, status: i32) -> Result<(), KeychainError> {
    if status == errSecSuccess {
        Ok(())
    } else {
        Err(KeychainError::Security { operation, status })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use objc2_security::errSecAuthFailed;

    #[test]
    fn read_distinguishes_absence_and_cancellation_from_errors() {
        assert!(read_status(errSecSuccess).unwrap());
        assert!(!read_status(errSecItemNotFound).unwrap());
        assert!(!read_status(errSecUserCanceled).unwrap());
        assert_eq!(
            read_status(errSecAuthFailed),
            Err(KeychainError::Security {
                operation: KeychainOperation::Read,
                status: errSecAuthFailed,
            })
        );
    }

    #[test]
    fn platform_error_keeps_native_operation_and_status() {
        let error: anyhow::Error = check_status(KeychainOperation::Update, errSecAuthFailed)
            .unwrap_err()
            .into();
        assert_eq!(
            error.downcast_ref::<KeychainError>(),
            Some(&KeychainError::Security {
                operation: KeychainOperation::Update,
                status: errSecAuthFailed,
            })
        );
    }

    #[test]
    fn delete_preserves_macos_missing_item_error() {
        assert_eq!(
            check_status(KeychainOperation::Delete, errSecSuccess),
            Ok(())
        );
        for status in [errSecItemNotFound, errSecUserCanceled, errSecAuthFailed] {
            assert_eq!(
                check_status(KeychainOperation::Delete, status),
                Err(KeychainError::Security {
                    operation: KeychainOperation::Delete,
                    status,
                })
            );
        }
    }
}
