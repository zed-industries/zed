# CHANGELOG

## 4.4.2

- Remove dependency on `dirs` crate due to MPL licensing in its tree. Use `home` crate instead. (@Xaeroxe)

## 4.4.1

- Add tilde expansion for home directory (@Xaeroxe)
- Swap out libc for rustix, forbid unsafe (@notgull)
