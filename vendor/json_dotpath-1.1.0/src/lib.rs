use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::{Map, Value};
use std::mem;
use thiserror::Error;

#[cfg(test)]
#[macro_use]
extern crate serde_derive;

/// Errors from dot_path methods
#[derive(Debug, Error)]
pub enum Error {
    /// Path hit a value in the JSON object that is not array or map
    /// and could not continue the traversal.
    ///
    /// (e.g. `foo.bar` in `{"foo": 123}`)
    #[error("Unexpected value reached while traversing path")]
    BadPathElement,

    /// Array index out of range
    #[error("Invalid array index: {0}")]
    BadIndex(usize),

    /// Invalid (usually empty) key used in Map or Array.
    /// If the key is valid but out of bounds, `BadIndex` will be used.
    #[error("Invalid key: {0}")]
    InvalidKey(String),

    /// Error serializing or deserializing a value
    #[error("Invalid array or map key")]
    SerdeError(#[from] serde_json::Error),
}

use crate::Error::{BadIndex, BadPathElement, InvalidKey};

pub type Result<T> = std::result::Result<T, Error>;

/// Convert Some(Value::Null) to None.
trait NullToNone<T> {
    fn null_to_none(self) -> Option<T>;
}

impl NullToNone<Value> for Option<Value> {
    fn null_to_none(self) -> Option<Value> {
        match self {
            None | Some(Value::Null) => None,
            Some(v) => Some(v),
        }
    }
}

impl<'a> NullToNone<&'a Value> for Option<&'a Value> {
    fn null_to_none(self) -> Option<&'a Value> {
        match self {
            None | Some(&Value::Null) => None,
            Some(v) => Some(v),
        }
    }
}

/// Access and mutate nested JSON elements by dotted paths
///
/// The path is composed of keys separated by dots, e.g. `foo.bar.1`.
///
/// All symbols in a path may be escaped by backslash (`\`) to have them treated literally,
/// e.g. to access a key containing a period.
///
/// Arrays are indexed by numeric strings or special keys (see `dot_get()` and `dot_set()`).
///
/// This trait is implemented for `serde_json::Value`, specifically the
/// `Map`, `Array`, and `Null` variants. Empty path can also be used to access a scalar.
///
/// Methods on this trait do not panic, errors are passed to the caller.
pub trait DotPaths {
    /// Get an item by path, if present.
    ///
    /// If the element does not exist or is `null`, None is returned.
    /// Accessing array index out of range raises `Err(BadIndex)`.
    ///
    /// The path does not need to reach a leaf node, i.e. it is possible to extract a subtree
    /// of a JSON object this way.
    ///
    /// # Special symbols
    /// - `>` ... last element of an array
    /// - `<` ... first element of an array (same as `0`)
    fn dot_get<T>(&self, path: &str) -> Result<Option<T>>
    where
        T: DeserializeOwned;

    /// Get an item by path, or a default value if it does not exist.
    ///
    /// This method is best suited for JSON objects (`Map`) or nullable fields.
    ///
    /// See `dot_get()` for more details.
    fn dot_get_or<T>(&self, path: &str, def: T) -> Result<T>
    where
        T: DeserializeOwned,
    {
        self.dot_get(path).map(|o| o.unwrap_or(def))
    }

    /// Get an item, or a default value using the Default trait
    ///
    /// This method is best suited for JSON objects (`Map`) or nullable fields.
    ///
    /// See `dot_get()` for more details.
    fn dot_get_or_default<T>(&self, path: &str) -> Result<T>
    where
        T: DeserializeOwned + Default,
    {
        self.dot_get_or(path, T::default())
    }

    /// Check if a value exists under a dotted path.
    /// Returns error if the path is invalid, a string key was used to index into an array.
    ///
    /// # Special symbols
    /// - `>` ... last element of an array
    /// - `<` ... first element of an array (same as `0`)
    fn dot_has_checked(&self, path: &str) -> Result<bool>;

    /// Check if a value exists under a dotted path.
    /// Returns false also when the path is invalid.
    ///
    /// Use `dot_has_checked` if you want to distinguish non-existent values from path errors.
    ///
    /// # Special symbols
    /// - `>` ... last element of an array
    /// - `<` ... first element of an array (same as `0`)
    fn dot_has(&self, path: &str) -> bool {
        self.dot_has_checked(path)
            .unwrap_or_default()
    }

    /// Get a mutable reference to an item
    ///
    /// If the path does not exist but a value on the path can be created (i.e. because the path
    /// reaches `null`, array or object), a `null` value is inserted in that location (creating
    /// its parent nodes as needed) and a mutable reference to this new `null` node is returned.
    ///
    /// The path does not need to reach a leaf node, i.e. it is possible to extract a subtree
    /// of a JSON object this way.
    ///
    /// # Special keys
    /// - `>` ... last element of an array
    /// - `<` ... first element of an array (same as `0`)
    fn dot_get_mut(&mut self, path: &str) -> Result<&mut Value>;

    /// Insert an item by path. The original value is dropped, if any.
    ///
    /// # Special symbols
    /// Arrays can be modified using special keys in the path:
    /// - `+` or `>>` ... append
    /// - `-` or `<<` ... prepend
    /// - `>n` ... insert after an index `n`
    /// - `<n` ... insert before an index `n`
    /// - `>` ... last element of an array
    /// - `<` ... first element of an array (same as `0`)
    fn dot_set<T>(&mut self, path: &str, value: T) -> Result<()>
    where
        T: Serialize;

    /// Replace a value by path with a new value.
    /// The value types do not have to match.
    ///
    /// Returns `Ok(None)` if the path was previously empty or `null`.
    ///
    /// # Special keys
    /// - `>` ... last element of an array
    /// - `<` ... first element of an array (same as `0`)
    fn dot_replace<NEW, OLD>(&mut self, path: &str, value: NEW) -> Result<Option<OLD>>
    where
        NEW: Serialize,
        OLD: DeserializeOwned;

    /// Get an item using a path, removing it from the object.
    ///
    /// Value becomes `null` when taken by an empty path, map entry is removed,
    /// and array item is extracted, shifting the remainder forward.
    ///
    /// Returns `Ok(None)` if the path was previously empty or `null`.
    ///
    /// # Special keys
    /// - `>` ... last element of an array
    /// - `<` ... first element of an array (same as `0`)
    fn dot_take<T>(&mut self, path: &str) -> Result<Option<T>>
    where
        T: DeserializeOwned;

    /// Remove and drop an item matching a key.
    /// Returns true if any item was removed.
    ///
    /// # Special keys
    /// - `>` ... last element of an array
    /// - `<` ... first element of an array (same as `0`)
    fn dot_remove(&mut self, path: &str) -> Result<()> {
        let _ = self.dot_take::<Value>(path)?; // Original value is dropped
        Ok(())
    }
}

/// Split the path string by dot, if present.
///
/// Returns a tuple of (before_dot, after_dot), removing escapes
fn path_split(path: &str) -> (String, Option<&str>) {
    let mut buf = String::new();
    let mut escaped = false;
    for (n, c) in path.char_indices() {
        match c {
            _ if escaped => {
                buf.push(c);
                escaped = false;
            }
            '\\' => {
                escaped = true;
            }
            '.' => {
                return (buf, Some(&path[n + 1..]));
            }
            _ => {
                buf.push(c);
            }
        }
    }

    // trailing slash is discarded
    (buf, None)
}

impl DotPaths for serde_json::Value {
    fn dot_get<T>(&self, path: &str) -> Result<Option<T>>
    where
        T: DeserializeOwned,
    {
        match self {
            Value::Array(vec) => vec.dot_get(path),
            Value::Object(map) => map.dot_get(path),
            Value::Null => Ok(None),
            _ => {
                if path.is_empty() {
                    // Get the whole value.
                    // We know it's not null - checked above
                    Ok(Some(serde_json::from_value(self.to_owned())?))
                } else {
                    // Path continues, but we can't traverse into a scalar
                    Err(BadPathElement)
                }
            }
        }
    }

    fn dot_has_checked(&self, path: &str) -> Result<bool> {
        match self {
            Value::Array(vec) => vec.dot_has_checked(path),
            Value::Object(map) => map.dot_has_checked(path),
            Value::Null => Ok(false),
            _ => {
                if path.is_empty() {
                    Ok(true)
                } else {
                    // Path continues, but we can't traverse into a scalar
                    Ok(false)
                }
            }
        }
    }

    fn dot_get_mut(&mut self, path: &str) -> Result<&mut Value> {
        match self {
            Value::Array(vec) => vec.dot_get_mut(path),
            Value::Object(map) => map.dot_get_mut(path),
            _ => {
                if path.is_empty() {
                    Ok(self)
                } else {
                    if self.is_null() {
                        // Spawn parents
                        self.dot_set(path, Value::Null)?;
                        // Now it will succeed
                        return self.dot_get_mut(path);
                    }

                    // Path continues, but we can't traverse into a scalar
                    Err(BadPathElement)
                }
            }
        }
    }

    fn dot_replace<NEW, OLD>(&mut self, path: &str, value: NEW) -> Result<Option<OLD>>
    where
        NEW: Serialize,
        OLD: DeserializeOwned,
    {
        match self {
            Value::Array(vec) => vec.dot_replace(path, value),
            Value::Object(map) => map.dot_replace(path, value),
            Value::Null => {
                // spawn new
                *self = new_by_path_root(path, value)?;
                Ok(None)
            }
            _ => {
                if path.is_empty() {
                    let new = serde_json::to_value(value)?;
                    let old = mem::replace(self, new);
                    Ok(serde_json::from_value(old)?)
                } else {
                    // Path continues, but we can't traverse into a scalar
                    Err(BadPathElement)
                }
            }
        }
    }

    fn dot_take<T>(&mut self, path: &str) -> Result<Option<T>>
    where
        T: DeserializeOwned,
    {
        match self {
            Value::Array(vec) => vec.dot_take(path),
            Value::Object(map) => map.dot_take(path),
            Value::Null => Ok(None),
            _ => {
                if path.is_empty() {
                    // This won't happen with array or object, they really remove the value.
                    // Value is replaced with NULL only when dot_take() is called
                    // with an empty path.
                    let old = mem::replace(self, Value::Null); // we know it's not null, checked above
                    Ok(Some(serde_json::from_value(old)?))
                } else {
                    // Path continues, but we can't traverse into a scalar
                    Err(BadPathElement)
                }
            }
        }
    }

    fn dot_set<T>(&mut self, path: &str, value: T) -> Result<()>
    where
        T: Serialize,
    {
        match self {
            // Special case for Vec, which implements additional path symbols
            Value::Array(a) => a.dot_set(path, value),
            Value::Object(m) => m.dot_set(path, value),
            _ => {
                let _ = self.dot_replace::<T, Value>(path, value)?; // Original value is dropped
                Ok(())
            }
        }
    }
}

/// Create a Value::Object or Value::Array based on a nested path.
///
/// Builds the parent path to a non-existent key in set-type operations.
fn new_by_path_root<T>(path: &str, value: T) -> Result<Value>
where
    T: Serialize,
{
    if path.is_empty() {
        return Ok(serde_json::to_value(value)?);
    }

    let escaped = path.starts_with('\\');
    let (sub1, _) = path_split(path);
    if !escaped && ["0", "+", "-", "<", ">", "<<", ">>"].contains(&sub1.as_str()) {
        // new vec
        let mut new_vec = vec![];
        new_vec.dot_set(path, value)?;
        Ok(Value::Array(new_vec))
    } else {
        // new map
        let mut new_map = Map::new();
        new_map.dot_set(path, value)?;
        Ok(Value::Object(new_map))
    }
}

impl DotPaths for serde_json::Map<String, serde_json::Value> {
    fn dot_get<T>(&self, path: &str) -> Result<Option<T>>
    where
        T: DeserializeOwned,
    {
        let (my, sub) = path_split(path);

        if my.is_empty() {
            return Err(InvalidKey(my));
        }

        if let Some(sub_path) = sub {
            match self.get(&my).null_to_none() {
                None => Ok(None),
                Some(child) => child.dot_get(sub_path),
            }
        } else {
            match self.get(&my).null_to_none() {
                None => Ok(None),
                Some(m) => Ok(Some(serde_json::from_value::<T>(m.to_owned())?)),
            }
        }
    }

    fn dot_has_checked(&self, path: &str) -> Result<bool> {
        let (my, sub) = path_split(path);

        if my.is_empty() {
            return Err(InvalidKey(my));
        }

        if let Some(sub_path) = sub {
            match self.get(&my).null_to_none() {
                None => Ok(false),
                Some(child) => child.dot_has_checked(sub_path),
            }
        } else {
            match self.get(&my).null_to_none() {
                None => Ok(false),
                Some(_) => Ok(true),
            }
        }
    }

    #[allow(clippy::collapsible_if)]
    fn dot_get_mut(&mut self, path: &str) -> Result<&mut Value> {
        let (my, sub) = path_split(path);

        if my.is_empty() {
            return Err(InvalidKey(my));
        }

        if let Some(sub_path) = sub {
            if self.contains_key(&my) {
                self.get_mut(&my)
                    .unwrap() // OK
                    .dot_get_mut(sub_path)
            } else {
                // create a subtree
                self.insert(my.clone(), new_by_path_root(sub_path, Value::Null)?);

                // get reference to the new Null
                self.get_mut(&my)
                    .unwrap() // OK, we just inserted it
                    .dot_get_mut(sub_path)
            }
        } else {
            if self.contains_key(&my) {
                Ok(self.get_mut(&my).unwrap())
            } else {
                // create a null value
                self.insert(my.clone(), Value::Null);
                Ok(self.get_mut(&my).unwrap())
            }
        }
    }

    fn dot_set<T>(&mut self, path: &str, value: T) -> Result<()> where
        T: Serialize {
        let (my, sub) = path_split(path);

        if my.is_empty() {
            return Err(InvalidKey(my));
        }

        if let Some(subpath) = sub {
            if self.contains_key(&my) {
                self.get_mut(&my).unwrap().dot_set(subpath, value)
            } else {
                // Build new subpath
                let _ = self.insert(my, new_by_path_root(subpath, value)?); // always returns None here
                Ok(())
            }
        } else {
            let packed = serde_json::to_value(value)?;
            self.insert(my, packed);
            Ok(())
        }
    }

    fn dot_replace<NEW, OLD>(&mut self, path: &str, value: NEW) -> Result<Option<OLD>>
    where
        NEW: Serialize,
        OLD: DeserializeOwned,
    {
        let (my, sub) = path_split(path);

        if my.is_empty() {
            return Err(InvalidKey(my));
        }

        if let Some(subpath) = sub {
            if self.contains_key(&my) {
                self.get_mut(&my).unwrap().dot_replace(subpath, value)
            } else {
                // Build new subpath
                let _ = self.insert(my, new_by_path_root(subpath, value)?); // always returns None here
                Ok(None)
            }
        } else {
            let packed = serde_json::to_value(value)?;
            match self.insert(my, packed).null_to_none() {
                None => Ok(None),
                Some(old) => Ok(serde_json::from_value(old)?),
            }
        }
    }

    fn dot_take<T>(&mut self, path: &str) -> Result<Option<T>>
    where
        T: DeserializeOwned,
    {
        let (my, sub) = path_split(path);

        if my.is_empty() {
            return Err(InvalidKey(my));
        }

        if let Some(subpath) = sub {
            if let Some(item) = self.get_mut(&my) {
                item.dot_take(subpath)
            } else {
                // no such element
                Ok(None)
            }
        } else {
            match self.remove(&my).null_to_none() {
                None => Ok(None),
                Some(old) => Ok(serde_json::from_value(old)?),
            }
        }
    }
}

impl DotPaths for Vec<serde_json::Value> {
    fn dot_get<T>(&self, path: &str) -> Result<Option<T>>
    where
        T: DeserializeOwned,
    {
        let (my, sub) = path_split(path);

        if my.is_empty() {
            return Err(InvalidKey(my));
        }

        if self.is_empty() {
            return Ok(None);
        }

        let index: usize = match my.as_str() {
            ">" => self.len() - 1, // non-empty checked above
            "<" => 0,
            _ => my.parse().map_err(|_| {
                InvalidKey(my)
            })?,
        };

        if index >= self.len() {
            return Err(BadIndex(index));
        }

        if let Some(subpath) = sub {
            match self.get(index).null_to_none() {
                None => Ok(None),
                Some(child) => child.dot_get(subpath),
            }
        } else {
            match self.get(index).null_to_none() {
                None => Ok(None),
                Some(value) => Ok(serde_json::from_value(value.to_owned())?),
            }
        }
    }

    fn dot_has_checked(&self, path: &str) -> Result<bool> {
        let (my, sub) = path_split(path);

        if my.is_empty() {
            return Err(InvalidKey(my));
        }

        if self.is_empty() {
            return Ok(false);
        }

        let index: usize = match my.as_str() {
            ">" => self.len() - 1, // non-empty checked above
            "<" => 0,
            _ => my.parse().map_err(|_| {
                InvalidKey(my)
            })?,
        };

        if index >= self.len() {
            return Ok(false);
        }

        if let Some(subpath) = sub {
            match self.get(index).null_to_none() {
                None => Ok(false),
                Some(child) => child.dot_has_checked(subpath),
            }
        } else {
            match self.get(index).null_to_none() {
                // null is reported as unset
                None => Ok(false),
                Some(_) => Ok(true),
            }
        }
    }

    #[allow(clippy::collapsible_if)]
    fn dot_get_mut(&mut self, path: &str) -> Result<&mut Value> {
        let (my, sub) = path_split(path);

        if my.is_empty() {
            return Err(InvalidKey(my));
        }

        let index: usize = match my.as_str() {
            ">" => {
                if self.is_empty() {
                    0
                } else {
                    self.len() - 1
                }
            }
            "<" => 0,
            _ => my.parse().map_err(|_| {
                InvalidKey(my)
            })?,
        };

        if index > self.len() {
            return Err(BadIndex(index));
        }

        if let Some(subpath) = sub {
            if index < self.len() {
                self.get_mut(index).unwrap().dot_get_mut(subpath)
            } else {
                // create a subtree
                self.push(new_by_path_root(subpath, Value::Null)?);

                // get reference to the new Null
                self.get_mut(index)
                    .unwrap() // we just inserted it
                    .dot_get_mut(subpath)
            }
        } else {
            if index < self.len() {
                Ok(self.get_mut(index).unwrap())
            } else {
                // create a subtree
                self.push(Value::Null);

                // get reference to the new Null
                Ok(self.get_mut(index).unwrap()) // unwrap is safe now
            }
        }
    }

    #[allow(clippy::collapsible_if)]
    fn dot_set<T>(&mut self, path: &str, value: T) -> Result<()>
    where
        T: Serialize,
    {
        // implemented separately from replace because of the special index handling
        let (my_s, sub) = path_split(path);

        if my_s.is_empty() {
            return Err(InvalidKey(my_s));
        }

        let my = my_s.as_str(); // so it can be cropped

        let mut insert = false;

        let index = match my {
            "<" => 0, // first
            ">" => {
                if self.is_empty() {
                    0
                } else {
                    self.len() - 1
                }
            }
            "-" | "<<" => {
                // prepend
                insert = true;
                0
            }
            "+" | ">>" => {
                // append
                self.len()
            }
            _ if my.starts_with('>') => {
                // insert after
                insert = true;
                (&my[1..]).parse::<usize>()
                    .map_err(|_| {
                        InvalidKey(my_s)
                    })? + 1
            }
            _ if my.starts_with('<') => {
                // insert before
                insert = true;
                (&my[1..]).parse::<usize>()
                    .map_err(|_| {
                        InvalidKey(my_s)
                    })?
            }
            _ => my.parse::<usize>()
                .map_err(|_| {
                    InvalidKey(my_s)
                })?,
        };

        if index > self.len() {
            // equal is OK, that means append. More is an error
            return Err(BadIndex(index));
        }

        if let Some(subpath) = sub {
            if index < self.len() {
                if insert {
                    self.insert(index, new_by_path_root(subpath, value)?);
                } else {
                    // replace
                    self[index].dot_set(subpath, value)?;
                }
            } else {
                // Append
                self.push(new_by_path_root(subpath, value)?);
            }
        } else {
            if index < self.len() {
                if insert {
                    self.insert(index, serde_json::to_value(value)?);
                } else {
                    // replace
                    self[index] = serde_json::to_value(value)?; // old value is dropped
                }
            } else {
                // Append
                self.push(serde_json::to_value(value)?);
            }
        }

        Ok(())
    }

    fn dot_replace<T, U>(&mut self, path: &str, value: T) -> Result<Option<U>>
    where
        T: Serialize,
        U: DeserializeOwned,
    {
        let (my, sub) = path_split(path);

        if my.is_empty() {
            return Err(InvalidKey(my));
        }

        let index: usize = match my.as_str() {
            ">" => {
                if self.is_empty() {
                    0
                } else {
                    self.len() - 1 // last element
                }
            }
            "<" => 0,
            _ => my.parse().map_err(|_| {
                InvalidKey(my)
            })?,
        };

        if index >= self.len() {
            // appending is not supported in replace
            return Err(BadIndex(index));
        }

        if let Some(subpath) = sub {
            self.get_mut(index)
                .unwrap() // Bounds checked above
                .dot_replace(subpath, value)
        } else {
            // No subpath
            let new = serde_json::to_value(value)?;
            let old = mem::replace(&mut self[index], new);
            if old.is_null() {
                Ok(None)
            } else {
                Ok(serde_json::from_value(old)?)
            }
        }
    }

    fn dot_take<T>(&mut self, path: &str) -> Result<Option<T>>
    where
        T: DeserializeOwned,
    {
        let (my, sub) = path_split(path);

        if my.is_empty() {
            return Err(InvalidKey(my));
        }

        let index: usize = match my.as_str() {
            ">" => {
                if self.is_empty() {
                    0
                } else {
                    self.len() - 1
                }
            }
            "<" => 0,
            _ => my.parse().map_err(|_| {
                InvalidKey(my)
            })?,
        };

        if index >= self.len() {
            return Err(BadIndex(index));
        }

        if let Some(subpath) = sub {
            self[index].dot_take(subpath)
        } else {
            // bounds are checked above
            Ok(serde_json::from_value(self.remove(index))?)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::DotPaths;
    use serde_json::json;
    use serde_json::Value;

    #[test]
    fn get_scalar_with_empty_path() {
        let value = Value::String("Hello".to_string());
        assert_eq!(Some("Hello".to_string()), value.dot_get("").unwrap());
    }

    #[test]
    fn cant_get_scalar_with_path() {
        let value = Value::String("Hello".to_string());
        assert!(value.dot_get::<Value>("1.2.3").is_err());
    }

    #[test]
    fn set_null() {
        let mut item = Value::Null;
        item.dot_set("", "foo").unwrap();
        assert_eq!(Value::String("foo".into()), item);
    }

    #[test]
    fn replace_null() {
        let mut item = Value::Null;
        assert_eq!(None, item.dot_replace::<_, Value>("", "foo").unwrap());
        assert_eq!(Value::String("foo".into()), item);
    }

    #[test]
    fn take_null() {
        let mut item = Value::Null;
        assert_eq!(None, item.dot_take::<Value>("").unwrap());
        assert_eq!(Value::Null, item);

        let mut item = Value::Bool(true);
        assert_eq!(Some(true), item.dot_take::<bool>("").unwrap());
        assert_eq!(Value::Null, item);
    }

    #[test]
    fn set_vec() {
        let mut vec = Value::Array(vec![]);
        vec.dot_set("0", "first").unwrap();
        vec.dot_set("0", "second").unwrap(); // this replaces it
        vec.dot_set("1", "third").unwrap();
        vec.dot_set("+", "append").unwrap();
        vec.dot_set(">>", "append2").unwrap();
        vec.dot_set("-", "prepend").unwrap();
        vec.dot_set("<<", "prepend2").unwrap();
        vec.dot_set("<0", "prepend3").unwrap();
        vec.dot_set(">1", "insert after 1").unwrap();
        vec.dot_set(">0", "after0").unwrap();
        vec.dot_set("<2", "before2").unwrap();
        assert_eq!(
            json!([
                "prepend3",
                "after0",
                "before2",
                "prepend2",
                "insert after 1",
                "prepend",
                "second",
                "third",
                "append",
                "append2"
            ]),
            vec
        );
    }

    #[test]
    fn set_vec_err_bad_index() {
        let mut vec = Value::Array(vec![]);
        assert!(vec.dot_set("1", "first").is_err());
    }

    #[test]
    fn set_vec_err_index_not_numeric() {
        let mut vec = Value::Array(vec![]);
        assert!(vec.dot_set("abc", "first").is_err());
    }

    #[test]
    fn set_vec_spawn() {
        let mut vec = Value::Array(vec![]);

        vec.dot_set("0.0.0", "first").unwrap();
        vec.dot_set("+", "append").unwrap();
        vec.dot_set("<1", "in between").unwrap();
        assert_eq!(json!([[["first"]], "in between", "append"]), vec);

        vec.dot_set("0.0.+", "second").unwrap();
        assert_eq!(json!([[["first", "second"]], "in between", "append"]), vec);

        vec.dot_set("0.+", "mmm").unwrap();
        assert_eq!(
            json!([[["first", "second"], "mmm"], "in between", "append"]),
            vec
        );

        vec.dot_set("0.+.0", "xyz").unwrap();
        assert_eq!(
            json!([
                [["first", "second"], "mmm", ["xyz"]],
                "in between",
                "append"
            ]),
            vec
        );

        // append and prepend can also spawn a new array
        let mut vec = Value::Array(vec![]);
        vec.dot_set(">>.>>.>>", "first").unwrap();
        assert_eq!(json!([[["first"]]]), vec);
        vec.dot_set(">>.<<.>>", "second").unwrap();
        assert_eq!(json!([[["first"]], [["second"]]]), vec);
    }

    #[test]
    fn array_append1() {
        let mut test_array = Value::Array(vec![]);
        test_array.dot_set(">>", Value::String(String::from("Go to class"))).unwrap();
        test_array.dot_set(">>", Value::String(String::from("Fish"))).unwrap();
        assert_eq!(json!(["Go to class","Fish"]), test_array);
    }

    #[test]
    fn array_append2() {
        let mut test_array = Value::Array(vec![]);
        test_array.dot_set("+", Value::String(String::from("Go to class"))).unwrap();
        test_array.dot_set("+", Value::String(String::from("Fish"))).unwrap();
        assert_eq!(json!(["Go to class","Fish"]), test_array);
    }

    #[test]
    fn array_append_in_object1() {
        let mut test_inner_array = Value::Null;

        test_inner_array.dot_set("todos.+", Value::String(String::from("Go to class"))).unwrap();
        assert_eq!(json!({"todos" : ["Go to class"] }), test_inner_array);

        test_inner_array.dot_set("todos.+", Value::String(String::from("Fish"))).unwrap();
        assert_eq!(json!({"todos" : ["Go to class","Fish"] }), test_inner_array);
    }

    #[test]
    fn array_append_in_object2() {
        let mut test_inner_array = Value::Null;
        test_inner_array.dot_set("name", Value::String(String::from("Google"))).unwrap();
        assert_eq!(json!({"name" : "Google"}), test_inner_array);

        test_inner_array.dot_set("todos.+", Value::String(String::from("Go to class"))).unwrap();
        assert_eq!(json!({"name" : "Google", "todos" : ["Go to class"] }), test_inner_array);

        test_inner_array.dot_set("todos.+", Value::String(String::from("Fish"))).unwrap();
        assert_eq!(json!({"name" : "Google", "todos" : ["Go to class","Fish"] }), test_inner_array);
    }

    #[test]
    fn get_vec() {
        let vec = json!([
            [["first", "second"], "mmm", ["xyz"]],
            "in between",
            "append"
        ]);
        assert_eq!(Some("first".to_string()), vec.dot_get("0.0.0").unwrap());
        assert_eq!(Some("second".to_string()), vec.dot_get("0.0.1").unwrap());

        // this one doesn't exist
        assert!(vec.dot_get::<String>("0.0.3").is_err());

        // get last
        assert_eq!(Some(json!(["xyz"])), vec.dot_get("0.>").unwrap());

        // retrieve a Value
        assert_eq!(
            Some(json!([["first", "second"], "mmm", ["xyz"]])),
            vec.dot_get("0").unwrap()
        );
        assert_eq!(
            Some(json!(["first", "second"])),
            vec.dot_get("0.0").unwrap()
        );
    }

    #[test]
    fn get_escapes() {
        let vec = json!({
            "foo.bar": {
                "\\slashes\\\\ya.yy": 123,
                "#hash": {
                    "foobar": "<aaa>"
                }
            }
        });
        assert_eq!(
            Some(123),
            vec.dot_get("foo\\.bar.\\\\slashes\\\\\\\\ya\\.yy").unwrap()
        );
        assert_eq!(
            Some("<aaa>".to_string()),
            vec.dot_get("foo\\.bar.\\#hash.foobar").unwrap()
        );
    }

    #[test]
    fn get_vec_err_index_scalar() {
        let vec = json!([
            [["first", "second"], "mmm", ["xyz"]],
            "in between",
            "append"
        ]);
        assert!(vec.dot_get::<Value>("0.0.1.4").is_err());
    }

    #[test]
    fn take_from_vec() {
        let mut vec = json!(["a", "b", "c"]);

        // test Take
        assert_eq!(Some("b".to_string()), vec.dot_take("1").unwrap());
        assert!(vec.dot_take::<Value>("4").is_err());
        assert_eq!(json!(["a", "c"]), vec);

        let mut vec = json!([[["a"], "b"], "c"]);
        assert_eq!(Some("a".to_string()), vec.dot_take("0.0.0").unwrap());
        assert_eq!(json!([[[], "b"], "c"]), vec); // empty vec is left in place
    }

    #[test]
    fn remove_from_vec() {
        let mut vec = json!(["a", "b", "c"]);

        assert!(vec.dot_remove("1").is_ok());
        assert!(vec.dot_remove("4").is_err()); // out of bounds
        assert_eq!(json!(["a", "c"]), vec);

        let mut vec = json!([[["a"], "b"], "c"]);
        assert!(vec.dot_remove("0.0.0").is_ok());
        assert!(vec.dot_remove("0.0.0").is_err()); // doesn't exist
        assert_eq!(json!([[[], "b"], "c"]), vec); // empty vec is left in place
    }

    #[test]
    fn replace_in_vec() {
        let mut vec = json!(["a", "b", "c"]);

        assert_eq!(Some("b".to_string()), vec.dot_replace("1", "BBB").unwrap());

        let mut vec = json!([[["a"], "b"], "c"]);
        assert_eq!(
            Some("a".to_string()),
            vec.dot_replace("0.0.0", "AAA").unwrap()
        );
        assert_eq!(json!([[["AAA"], "b"], "c"]), vec);
    }

    #[test]
    fn set_map() {
        let mut vec = json!({});
        vec.dot_set("foo", "bar").unwrap();
        assert_eq!(json!({"foo": "bar"}), vec);

        let mut vec = json!({});
        vec.dot_set("foo.bar.baz", "binx").unwrap();
        assert_eq!(json!({"foo": {"bar": {"baz": "binx"}}}), vec);

        vec.dot_set("foo.bar.abc.0", "aaa").unwrap();
        assert_eq!(json!({"foo": {"bar": {"baz": "binx","abc": ["aaa"]}}}), vec);
    }

    #[test]
    fn set_map_err_bad_index() {
        let mut map = json!({});
        assert!(map.dot_set("0", "first").is_ok());
        assert_eq!(json!({"0": "first"}), map);

        assert_eq!(Some("first".to_string()), map.dot_get("0").unwrap());
    }

    #[test]
    fn get_map() {
        let vec = json!({"one": "two"});
        assert_eq!(Some("two".to_string()), vec.dot_get("one").unwrap());

        let vec = json!({"one": {"two": "three"}});
        assert_eq!(Some("three".to_string()), vec.dot_get("one.two").unwrap());

        let vec = json!({"one": "two"});
        assert_eq!(None, vec.dot_get::<Value>("xxx").unwrap());
    }

    #[test]
    fn map_escaped_keys() {
        let mut map = json!({});
        map.dot_set("\\0.\\1", 123).unwrap();
        assert_eq!(json!({"0": {"1": 123}}), map);
        map.dot_set("\\0.\\2", 456).unwrap();

        assert_eq!(Some(123), map.dot_get("\\0.\\1").unwrap());
        assert_eq!(Some(123), map.dot_take("\\0.\\1").unwrap());
        assert_eq!(json!({"0": {"2": 456}}), map);
        assert!(map.dot_remove("\\0.\\2").is_ok());

        map.dot_set("\\0.\\\\2", 456).unwrap();
        assert_eq!(json!({"0": {"\\2": 456}}), map);
    }

    #[test]
    fn remove_from_map() {
        let mut vec = json!({"one": "two", "x": "y"});
        assert!(vec.dot_remove("one").is_ok());
        assert_eq!(json!({"x": "y"}), vec);
    }

    #[test]
    fn take_from_map() {
        let mut vec = json!({"one": "two", "x": "y"});
        assert_eq!(Some("two".to_string()), vec.dot_take("one").unwrap());
        assert_eq!(json!({"x": "y"}), vec);
    }

    #[test]
    fn replace_in_map() {
        let mut vec = json!({"one": "two", "x": "y"});
        assert_eq!(
            Some("two".to_string()),
            vec.dot_replace("one", "fff").unwrap()
        );
        assert_eq!(json!({"one": "fff", "x": "y"}), vec);

        // replace value for string
        let mut vec = json!({"one": "two", "x": {"bbb": "y"}});
        assert_eq!(
            Some("y".to_string()),
            vec.dot_replace("x.bbb", "mm").unwrap()
        );
        assert_eq!(
            Some(json!({"bbb": "mm"})),
            vec.dot_replace("x", "betelgeuze").unwrap()
        );
        assert_eq!(json!({"one": "two", "x": "betelgeuze"}), vec);

        // nested replace
        let mut vec = json!({"one": "two", "x": {"bbb": ["y"]}});
        assert_eq!(
            Some("y".to_string()),
            vec.dot_replace("x.bbb.0", "betelgeuze").unwrap()
        );
        assert_eq!(json!({"one": "two", "x": {"bbb": ["betelgeuze"]}}), vec);
    }

    #[test]
    fn value_get_mut() {
        // Borrow Null as mutable
        let mut obj = Value::Null;
        let m = obj.dot_get_mut("").unwrap();
        *m = Value::from(123);
        assert_eq!(Value::from(123), obj);

        // Create a parents path
        let mut obj = Value::Null;
        let _ = obj.dot_get_mut("foo.0").unwrap();
        assert_eq!(json!({ "foo": [null] }), obj);
    }

    #[test]
    fn map_get_mut() {
        // Spawn empty element
        let mut obj = serde_json::Map::<String, Value>::new();
        let _ = obj.dot_get_mut("foo").unwrap();
        assert_eq!(json!({ "foo": null }), Value::Object(obj));

        // Spawn a subtree
        let mut obj = serde_json::Map::<String, Value>::new();
        let m = obj.dot_get_mut("foo.bar.baz").unwrap();
        m.dot_set("dog", "cat").unwrap();

        assert_eq!(
            json!({"foo": {"bar": {"baz": {"dog": "cat"}}}}),
            Value::Object(obj)
        );
    }

    #[test]
    fn vec_get_mut() {
        // Spawn empty element
        let mut obj = Vec::<Value>::new();
        let _ = obj.dot_get_mut(">").unwrap();
        assert_eq!(json!([null]), Value::Array(obj));

        // Spawn a subtree
        let mut obj = Vec::<Value>::new();
        let m = obj.dot_get_mut("0.foo.bar").unwrap();
        m.dot_set("dog", "cat").unwrap();

        assert_eq!(json!([{"foo": {"bar": {"dog": "cat"}}}]), Value::Array(obj));
    }

    #[test]
    fn has() {
        let value = json!({
            "one": "two",
            "x": [1, 2, {"foo": 123}]
        });
        assert!(value.dot_has("one"));
        assert!(!value.dot_has("two"));
        assert!(value.dot_has("x"));
        assert!(value.dot_has("x.0"));
        assert!(value.dot_has("x.<"));
        assert!(value.dot_has("x.>"));
        assert!(value.dot_has("x.>.foo"));
        assert!(!value.dot_has("x.banana"));
        assert!(!value.dot_has("x.>.foo.bar"));
        assert!(value.dot_has_checked("x.banana").is_err());

        if let Ok(false) = value.dot_has_checked("x.9999") {
            //
        } else {
            panic!("dot_has_checked failed");
        }
    }

    #[test]
    fn stamps() {
        let mut stamps = Value::Null;
        // null will become Value::Array(vec![])

        #[derive(Serialize, Deserialize, PartialEq, Default)]
        struct Stamp {
            country: String,
            year: u32,
            color: String,
            #[serde(rename = "face value")]
            face_value: String,
        };

        stamps
            .dot_set(
                "0",
                json!({
                    "country": "British Mauritius",
                    "year": 1847,
                    "color": "orange",
                    "face value": "1 penny"
                }),
            )
            .unwrap();

        // append
        stamps
            .dot_set(
                "+",
                Stamp {
                    country: "British Mauritius".to_string(),
                    year: 1847,
                    color: "blue".to_string(),
                    face_value: "2 pence".to_string(),
                },
            )
            .unwrap();

        assert_eq!(
            "orange",
            stamps.dot_get::<String>("0.color").unwrap().unwrap()
        );
        assert_eq!(
            "blue",
            stamps.dot_get::<String>("1.color").unwrap().unwrap()
        );

        assert_eq!(1847, stamps.dot_get::<Stamp>("1").unwrap().unwrap().year);

        // Remove the first stamp's "face value" attribute
        assert_eq!(
            Some("1 penny".to_string()),
            stamps.dot_get("0.face value").unwrap()
        );
        stamps.dot_remove("0.face value").unwrap();
        assert_eq!(
            Option::<Value>::None,
            stamps.dot_get("0.face value").unwrap()
        );

        // change the second stamp's year
        let old_year: u32 = stamps.dot_replace("1.year", 1850).unwrap().unwrap();
        assert_eq!(1847, old_year);
        assert_eq!(1850, stamps.dot_get::<u32>("1.year").unwrap().unwrap());
    }
}
