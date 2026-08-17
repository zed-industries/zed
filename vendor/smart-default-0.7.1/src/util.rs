use syn::parse::Error;
use syn::spanned::Spanned;

/// Return the value that fulfills the predicate if there is one in the slice. Panic if there is
/// more than one.
pub fn find_only<T, F>(iter: impl Iterator<Item = T>, pred: F) -> Result<Option<T>, Error>
where
    T: Spanned,
    F: Fn(&T) -> Result<bool, Error>,
{
    let mut result = None;
    for item in iter {
        if pred(&item)? {
            if result.is_some() {
                return Err(Error::new(item.span(), "Multiple defaults"));
            }
            result = Some(item);
        }
    }
    Ok(result)
}
