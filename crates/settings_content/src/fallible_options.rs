use std::cell::RefCell;

use serde::Deserialize;

use crate::ParseStatus;

thread_local! {
    static ERRORS: RefCell<Option<Vec<anyhow::Error>>> = const { RefCell::new(None) };
}

pub fn parse_json<'de, T>(json: &'de str) -> (Option<T>, ParseStatus)
where
    T: Deserialize<'de>,
{
    ERRORS.with_borrow_mut(|errors| {
        errors.replace(Vec::default());
    });

    let mut deserializer = serde_json_lenient::Deserializer::from_str(json);
    let value = serde_path_to_error::deserialize::<_, T>(&mut deserializer);
    let value = match value {
        Ok(value) => value,
        Err(error) => {
            return (
                None,
                ParseStatus::Failed {
                    error: error.into_inner().to_string(),
                },
            );
        }
    };

    if let Some(errors) = ERRORS.with_borrow_mut(|errors| errors.take().filter(|e| !e.is_empty())) {
        let error = errors
            .into_iter()
            .map(|e| e.to_string())
            .flat_map(|e| ["\n".to_owned(), e])
            .skip(1)
            .collect::<String>();
        return (Some(value), ParseStatus::Failed { error });
    }

    (Some(value), ParseStatus::Success)
}

pub(crate) fn deserialize<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    D: serde::Deserializer<'de>,
    T: serde::Deserialize<'de> + FallibleOption,
{
    match T::deserialize(deserializer) {
        Ok(value) => Ok(value),
        Err(e) => ERRORS.with_borrow_mut(|errors| {
            if let Some(errors) = errors {
                errors.push(anyhow::anyhow!("{}", e));
                Ok(Default::default())
            } else {
                Err(e)
            }
        }),
    }
}

pub trait FallibleOption: Default {}
impl<T> FallibleOption for Option<T> {}

macro_rules! flattened_deserialize {
    ($type_name:ty {
        sections: { $($section:ident),* $(,)? },
        options: { $($option_field:ident),* $(,)? },
        defaults: { $($default_field:ident),* $(,)? } $(,)?
    }) => {
        impl $type_name {
            #[doc(hidden)]
            pub const NAMED_DESERIALIZE_KEYS: &'static [&'static str] = &[
                $(stringify!($option_field),)*
                $(stringify!($default_field),)*
            ];
        }

        impl<'de> serde::Deserialize<'de> for $type_name {
            fn deserialize<D: serde::Deserializer<'de>>(
                deserializer: D,
            ) -> Result<Self, D::Error> {
                let mut object =
                    serde_json::Map::<String, serde_json::Value>::deserialize(deserializer)?;
                (|| -> Result<Self, serde_json::Error> {
                    $(
                        let $option_field =
                            $crate::fallible_options::take_option_field(
                                &mut object,
                                stringify!($option_field),
                            )?;
                    )*
                    $(
                        let $default_field =
                            $crate::fallible_options::take_default_field(
                                &mut object,
                                stringify!($default_field),
                            )?;
                    )*
                    let rest = serde_json::Value::Object(object);
                    Ok(Self {
                        $($section: $crate::fallible_options::section(&rest)?,)*
                        $($option_field,)*
                        $($default_field,)*
                    })
                })()
                .map_err(serde::de::Error::custom)
            }
        }
    };
}
pub(crate) use flattened_deserialize;

pub(crate) fn take_option_field<T>(
    object: &mut serde_json::Map<String, serde_json::Value>,
    key: &str,
) -> Result<T, serde_json::Error>
where
    T: serde::de::DeserializeOwned + FallibleOption,
{
    match object.remove(key) {
        None => Ok(T::default()),
        Some(value) => deserialize(&value),
    }
}

pub(crate) fn take_default_field<T>(
    object: &mut serde_json::Map<String, serde_json::Value>,
    key: &str,
) -> Result<T, serde_json::Error>
where
    T: serde::de::DeserializeOwned + Default,
{
    match object.remove(key) {
        None => Ok(T::default()),
        Some(value) => T::deserialize(&value),
    }
}

pub(crate) fn section<T>(rest: &serde_json::Value) -> Result<T, serde_json::Error>
where
    T: serde::de::DeserializeOwned,
{
    T::deserialize(rest)
}

#[cfg(test)]
mod tests {
    use serde::Deserialize;
    use settings_macros::with_fallible_options;

    use crate::ParseStatus;

    #[with_fallible_options]
    #[derive(Deserialize, Debug, PartialEq)]
    struct Foo {
        foo: Option<String>,
        bar: Option<usize>,
        baz: Option<bool>,
    }

    #[test]
    fn test_fallible() {
        let input = r#"
            {"foo": "bar",
            "bar": "foo",
            "baz": 3,
            }
        "#;

        let (settings, result) = crate::fallible_options::parse_json::<Foo>(&input);
        assert_eq!(
            settings.unwrap(),
            Foo {
                foo: Some("bar".into()),
                bar: None,
                baz: None,
            }
        );

        assert!(settings_json::parse_json_with_comments::<Foo>(&input).is_err());

        let ParseStatus::Failed { error } = result else {
            panic!("Expected parse to fail")
        };

        assert_eq!(
            error,
            "invalid type: string \"foo\", expected usize at line 3 column 24\ninvalid type: integer `3`, expected a boolean at line 4 column 20".to_string()
        )
    }
}
