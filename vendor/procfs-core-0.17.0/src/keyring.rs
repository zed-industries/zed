//! Functions related to the in-kernel key management and retention facility
//!
//! For more details on this facility, see the `keyrings(7)` man page.
use crate::{build_internal_error, expect, from_str, ProcResult};
use bitflags::bitflags;
#[cfg(feature = "serde1")]
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, io::BufRead, time::Duration};

bitflags! {
    /// Various key flags
    #[cfg_attr(feature = "serde1", derive(Serialize, Deserialize))]
    #[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, PartialOrd, Ord)]
    pub struct KeyFlags: u32 {
            /// The key has been instantiated
            const INSTANTIATED = 0x01;
            /// THe key has been revoked
            const REVOKED = 0x02;
            /// The key is dead
            ///
            /// I.e. the key type has been unregistered.  A key may be briefly in this state during garbage collection.
            const DEAD = 0x04;
            /// The key contributes to the user's quota
            const QUOTA = 0x08;
            /// The key is under construction via a callback to user space
            const UNDER_CONSTRUCTION = 0x10;
            /// The key is negatively instantiated
            const NEGATIVE = 0x20;
            /// The key has been invalidated
            const INVALID = 0x40;
    }
}

bitflags! {
    /// Bitflags that represent the permissions for a key
    #[cfg_attr(feature = "serde1", derive(Serialize, Deserialize))]
    #[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, PartialOrd, Ord)]
    pub struct PermissionFlags: u32 {
        /// The attributes of the key may be read
        ///
        /// This includes the type, description, and access rights (excluding the security label)
        const VIEW = 0x01;
        /// For a key: the payload of the key may be read.  For a keyring: the list of serial numbers (keys) to which the keyring has links may be read.
        const READ = 0x02;
        /// The payload of the key may be updated and the key may be revoked.
        ///
        /// For a keyring, links may be added to or removed from the keyring, and the keyring
        /// may be cleared completely (all links are removed).
        const WRITE = 0x04;
        /// The key may be found by a search.
        ///
        /// For keyrings: keys and keyrings that are linked to by the keyring may be searched.
        const SEARCH = 0x08;
        /// Links may be created from keyrings to the key.
        ///
        /// The initial link to a key that is established when the key is created doesn't require this permission.
        const LINK = 0x10;
        /// The ownership details and security label of the key may be changed, the key's expiration
        /// time may be set, and the key may be revoked.
        const SETATTR = 0x20;
        const ALL = Self::VIEW.bits() | Self::READ.bits() | Self::WRITE.bits() | Self::SEARCH.bits() | Self::LINK.bits() | Self::SETATTR.bits();
    }
}

impl KeyFlags {
    fn from_str(s: &str) -> KeyFlags {
        let mut me = KeyFlags::empty();

        let mut chars = s.chars();
        match chars.next() {
            Some(c) if c == 'I' => me.insert(KeyFlags::INSTANTIATED),
            _ => {}
        }
        match chars.next() {
            Some(c) if c == 'R' => me.insert(KeyFlags::REVOKED),
            _ => {}
        }
        match chars.next() {
            Some(c) if c == 'D' => me.insert(KeyFlags::DEAD),
            _ => {}
        }
        match chars.next() {
            Some(c) if c == 'Q' => me.insert(KeyFlags::QUOTA),
            _ => {}
        }
        match chars.next() {
            Some(c) if c == 'U' => me.insert(KeyFlags::UNDER_CONSTRUCTION),
            _ => {}
        }
        match chars.next() {
            Some(c) if c == 'N' => me.insert(KeyFlags::NEGATIVE),
            _ => {}
        }
        match chars.next() {
            Some(c) if c == 'i' => me.insert(KeyFlags::INVALID),
            _ => {}
        }

        me
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde1", derive(Serialize, Deserialize))]
pub struct Permissions {
    pub possessor: PermissionFlags,
    pub user: PermissionFlags,
    pub group: PermissionFlags,
    pub other: PermissionFlags,
}
impl Permissions {
    fn from_str(s: &str) -> ProcResult<Permissions> {
        let possessor = PermissionFlags::from_bits(from_str!(u32, &s[0..2], 16))
            .ok_or_else(|| build_internal_error!(format!("Unable to parse {:?} as PermissionFlags", s)))?;

        let user = PermissionFlags::from_bits(from_str!(u32, &s[2..4], 16))
            .ok_or_else(|| build_internal_error!(format!("Unable to parse {:?} as PermissionFlags", s)))?;

        let group = PermissionFlags::from_bits(from_str!(u32, &s[4..6], 16))
            .ok_or_else(|| build_internal_error!(format!("Unable to parse {:?} as PermissionFlags", s)))?;

        let other = PermissionFlags::from_bits(from_str!(u32, &s[6..8], 16))
            .ok_or_else(|| build_internal_error!(format!("Unable to parse {:?} as PermissionFlags", s)))?;

        Ok(Permissions {
            possessor,
            user,
            group,
            other,
        })
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "serde1", derive(Serialize, Deserialize))]
pub enum KeyTimeout {
    Permanent,
    Expired,
    Timeout(Duration),
}

impl KeyTimeout {
    fn from_str(s: &str) -> ProcResult<KeyTimeout> {
        if s == "perm" {
            Ok(KeyTimeout::Permanent)
        } else if s == "expd" {
            Ok(KeyTimeout::Expired)
        } else {
            let (val, unit) = s.split_at(s.len() - 1);
            let val = from_str!(u64, val);
            match unit {
                "s" => Ok(KeyTimeout::Timeout(Duration::from_secs(val))),
                "m" => Ok(KeyTimeout::Timeout(Duration::from_secs(val * 60))),
                "h" => Ok(KeyTimeout::Timeout(Duration::from_secs(val * 60 * 60))),
                "d" => Ok(KeyTimeout::Timeout(Duration::from_secs(val * 60 * 60 * 24))),
                "w" => Ok(KeyTimeout::Timeout(Duration::from_secs(val * 60 * 60 * 24 * 7))),
                _ => Err(build_internal_error!(format!("Unable to parse keytimeout of {:?}", s))),
            }
        }
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde1", derive(Serialize, Deserialize))]
pub enum KeyType {
    /// This is a general-purpose key type.
    ///
    /// The key is kept entirely within kernel memory.  The payload may be read and updated by
    ///  user-space applications. The payload for keys of this type is a blob of arbitrary
    /// data of up to 32,767 bytes.

    /// The description may be any valid string, though it is preferred that it start
    /// with a colon-delimited prefix representing the service to which the key is of
    /// interest (for instance "afs:mykey").
    User,

    /// Keyrings are special keys which store a set of links to other keys (including
    /// other keyrings), analogous to a directory holding links to files.   The  main
    /// purpose  of  a  keyring is to prevent other keys from being garbage collected
    /// because nothing refers to them.
    ///
    /// Keyrings with descriptions (names) that begin with a  period  ('.')  are  re‐
    /// served to the implementation.
    Keyring,

    /// This  key  type  is  essentially  the same as "user", but it does not provide
    /// reading (i.e., the keyctl(2) KEYCTL_READ operation),  meaning  that  the  key
    /// payload is never visible from user space.  This is suitable for storing user‐
    /// name-password pairs that should not be readable from user space.
    ///
    /// The description of a "logon" key must start with a  non-empty colon-delimited
    /// prefix  whose  purpose  is  to identify the service to which the key belongs.
    /// (Note that this differs from keys of the "user" type, where the inclusion  of
    /// a prefix is recommended but is not enforced.)
    Logon,

    /// This key type is similar to the "user" key type, but it may hold a payload of
    /// up to 1 MiB in size.  This key type is useful for purposes  such  as  holding
    /// Kerberos ticket caches.
    ///
    /// The  payload  data may be stored in a tmpfs filesystem, rather than in kernel
    /// memory, if the data size exceeds the overhead of  storing  the  data  in  the
    /// filesystem.  (Storing the data in a filesystem requires filesystem structures
    /// to be allocated in the kernel.  The size of these structures  determines  the
    /// size  threshold  above  which the tmpfs storage method is used.)  Since Linux
    /// 4.8, the payload data is encrypted when stored in tmpfs,  thereby  preventing
    /// it from being written unencrypted into swap space.
    BigKey,

    /// Other specialized, but rare keys types
    Other(String),
}

impl KeyType {
    fn from_str(s: &str) -> KeyType {
        match s {
            "keyring" => KeyType::Keyring,
            "user" => KeyType::User,
            "logon" => KeyType::Logon,
            "big_key" => KeyType::BigKey,
            other => KeyType::Other(other.to_string()),
        }
    }
}

/// A key
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde1", derive(Serialize, Deserialize))]
pub struct Key {
    /// The ID (serial number) of the key
    pub id: u64,

    /// A set of flags describing the state of the key
    pub flags: KeyFlags,

    /// Count of the number of kernel credential structures that are
    /// pinning  the  key  (approximately: the number of threads and open file
    /// references that refer to this key).
    pub usage: u32,

    /// Key timeout
    pub timeout: KeyTimeout,

    /// Key permissions
    pub permissions: Permissions,

    /// The user ID of the key owner
    pub uid: u32,

    /// The group ID of the key.
    ///
    /// The value of `None` here means that the key has no group ID; this can occur in certain circumstances for
    /// keys created by the kernel.
    pub gid: Option<u32>,

    /// The type of key
    pub key_type: KeyType,

    /// The key description
    pub description: String,
}

impl Key {
    fn from_line(s: &str) -> ProcResult<Key> {
        let mut s = s.split_whitespace();

        let id = from_str!(u64, expect!(s.next()), 16);
        let s_flags = expect!(s.next());
        let usage = from_str!(u32, expect!(s.next()));
        let s_timeout = expect!(s.next());
        let s_perms = expect!(s.next());
        let uid = from_str!(u32, expect!(s.next()));
        let s_gid = expect!(s.next());
        let s_type = expect!(s.next());
        let desc: Vec<_> = s.collect();

        Ok(Key {
            id,
            flags: KeyFlags::from_str(s_flags),
            usage,
            timeout: KeyTimeout::from_str(s_timeout)?,
            permissions: Permissions::from_str(s_perms)?,
            uid,
            gid: if s_gid == "-1" {
                None
            } else {
                Some(from_str!(u32, s_gid))
            },
            key_type: KeyType::from_str(s_type),
            description: desc.join(" "),
        })
    }
}

/// A set of keys.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde1", derive(Serialize, Deserialize))]
pub struct Keys(pub Vec<Key>);

impl crate::FromBufRead for Keys {
    fn from_buf_read<R: BufRead>(r: R) -> ProcResult<Self> {
        let mut v = Vec::new();

        for line in r.lines() {
            let line = line?;
            v.push(Key::from_line(&line)?);
        }
        Ok(Keys(v))
    }
}

/// Information about a user with at least one key
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde1", derive(Serialize, Deserialize))]
pub struct KeyUser {
    /// The user that owns the key
    pub uid: u32,
    /// The kernel-internal usage count for the kernel structure used to record key users
    pub usage: u32,
    /// The total number of keys owned by the user
    pub nkeys: u32,
    /// THe number of keys that have been instantiated
    pub nikeys: u32,
    /// The number of keys owned by the user
    pub qnkeys: u32,
    /// The maximum number of keys that the user may own
    pub maxkeys: u32,
    /// The number of bytes consumed in playloads of the keys owned by this user
    pub qnbytes: u32,
    /// The upper limit on the number of bytes in key payloads for this user
    pub maxbytes: u32,
}

impl KeyUser {
    fn from_str(s: &str) -> ProcResult<KeyUser> {
        let mut s = s.split_whitespace();
        let uid = expect!(s.next());
        let usage = from_str!(u32, expect!(s.next()));
        let keys = expect!(s.next());
        let qkeys = expect!(s.next());
        let qbytes = expect!(s.next());

        let (nkeys, nikeys) = {
            let mut s = keys.split('/');
            (from_str!(u32, expect!(s.next())), from_str!(u32, expect!(s.next())))
        };
        let (qnkeys, maxkeys) = {
            let mut s = qkeys.split('/');
            (from_str!(u32, expect!(s.next())), from_str!(u32, expect!(s.next())))
        };
        let (qnbytes, maxbytes) = {
            let mut s = qbytes.split('/');
            (from_str!(u32, expect!(s.next())), from_str!(u32, expect!(s.next())))
        };

        Ok(KeyUser {
            uid: from_str!(u32, &uid[0..uid.len() - 1]),
            usage,
            nkeys,
            nikeys,
            qnkeys,
            maxkeys,
            qnbytes,
            maxbytes,
        })
    }
}

/// Information about a set of users with at least one key.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde1", derive(Serialize, Deserialize))]
pub struct KeyUsers(pub HashMap<u32, KeyUser>);

impl crate::FromBufRead for KeyUsers {
    fn from_buf_read<R: BufRead>(r: R) -> ProcResult<Self> {
        let mut map = HashMap::new();

        for line in r.lines() {
            let line = line?;
            let user = KeyUser::from_str(&line)?;
            map.insert(user.uid, user);
        }
        Ok(KeyUsers(map))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn key_flags() {
        assert_eq!(KeyFlags::from_str("I------"), KeyFlags::INSTANTIATED);
        assert_eq!(KeyFlags::from_str("IR"), KeyFlags::INSTANTIATED | KeyFlags::REVOKED);
        assert_eq!(KeyFlags::from_str("IRDQUNi"), KeyFlags::all());
    }

    #[test]
    fn timeout() {
        assert_eq!(KeyTimeout::from_str("perm").unwrap(), KeyTimeout::Permanent);
        assert_eq!(KeyTimeout::from_str("expd").unwrap(), KeyTimeout::Expired);
        assert_eq!(
            KeyTimeout::from_str("2w").unwrap(),
            KeyTimeout::Timeout(Duration::from_secs(1209600))
        );
        assert_eq!(
            KeyTimeout::from_str("14d").unwrap(),
            KeyTimeout::Timeout(Duration::from_secs(1209600))
        );
        assert_eq!(
            KeyTimeout::from_str("336h").unwrap(),
            KeyTimeout::Timeout(Duration::from_secs(1209600))
        );
        assert_eq!(
            KeyTimeout::from_str("20160m").unwrap(),
            KeyTimeout::Timeout(Duration::from_secs(1209600))
        );
        assert_eq!(
            KeyTimeout::from_str("1209600s").unwrap(),
            KeyTimeout::Timeout(Duration::from_secs(1209600))
        );
    }
}
