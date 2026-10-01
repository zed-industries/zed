use objc2_core_foundation::{CFData, CFDictionary, CFRetained, CFString, CFType, kCFBooleanTrue};
use objc2_security::*;
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
    unsafe {
        let query = CFDictionary::<CFString, CFType>::from_slices(
            &[kSecClass, kSecAttrServer],
            &[kSecClassInternetPassword, &server],
        );
        let attributes = CFDictionary::<CFString, CFType>::from_slices(
            &[kSecAttrAccount, kSecValueData],
            &[&account, &password],
        );
        let mut operation = KeychainOperation::Update;
        let mut status = SecItemUpdate(query.as_opaque(), attributes.as_opaque());
        if status == errSecItemNotFound {
            operation = KeychainOperation::Create;
            let attributes = CFDictionary::<CFString, CFType>::from_slices(
                &[kSecClass, kSecAttrServer, kSecAttrAccount, kSecValueData],
                &[kSecClassInternetPassword, &server, &account, &password],
            );
            status = SecItemAdd(attributes.as_opaque(), ptr::null_mut());
        }
        check_status(operation, status)?;
    }
    Ok(())
}

pub fn read_credentials(server: &str) -> Result<Option<(String, Vec<u8>)>, KeychainError> {
    let server = CFString::from_str(server);
    unsafe {
        let cf_true = kCFBooleanTrue.ok_or(KeychainError::InvalidResponse(
            "Core Foundation true value unavailable",
        ))?;
        let query = CFDictionary::<CFString, CFType>::from_slices(
            &[
                kSecClass,
                kSecAttrServer,
                kSecReturnAttributes,
                kSecReturnData,
            ],
            &[kSecClassInternetPassword, &server, cf_true, cf_true],
        );
        let mut result = ptr::null();
        let status = SecItemCopyMatching(query.as_opaque(), &mut result);
        if !read_status(status)? {
            return Ok(None);
        }
        let result = ptr::NonNull::new(result.cast_mut())
            .ok_or(KeychainError::InvalidResponse("keychain returned no item"))?;
        let result = CFRetained::from_raw(result)
            .downcast::<CFDictionary>()
            .map_err(|_| KeychainError::InvalidResponse("keychain item was not a dictionary"))?;
        // Security returns CFType keys and values; check each concrete value below.
        let result = result.cast_unchecked::<CFType, CFType>();
        let account = result
            .get(kSecAttrAccount)
            .ok_or(KeychainError::InvalidResponse(
                "account was missing from keychain item",
            ))?
            .downcast::<CFString>()
            .map_err(|_| KeychainError::InvalidResponse("account was not a string"))?;
        let password = result
            .get(kSecValueData)
            .ok_or(KeychainError::InvalidResponse(
                "password was missing from keychain item",
            ))?
            .downcast::<CFData>()
            .map_err(|_| KeychainError::InvalidResponse("password was not data"))?;
        Ok(Some((account.to_string(), password.to_vec())))
    }
}

pub fn delete_credentials(server: &str) -> Result<(), KeychainError> {
    let server = CFString::from_str(server);
    unsafe {
        let query = CFDictionary::<CFString, CFType>::from_slices(
            &[kSecClass, kSecAttrServer],
            &[kSecClassInternetPassword, &server],
        );
        check_status(KeychainOperation::Delete, SecItemDelete(query.as_opaque()))
    }
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
