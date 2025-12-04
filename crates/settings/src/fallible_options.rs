use std::cell::RefCell;

use serde::Deserialize;

use crate::ParseStatus;

thread_local! {
    static ERRORS: RefCell<Option<Vec<anyhow::Error>>> = const { RefCell::new(None) };
}

pub(crate) fn parse_json<'de, T>(json: &'de str) -> (Option<T>, ParseStatus)
where
    T: Deserialize<'de>,
{
    ERRORS.with_borrow_mut(|errors| {
        errors.replace(Vec::default());
    });

    let mut deserializer = serde_json_lenient::Deserializer::from_str(json);
    let value = T::deserialize(&mut deserializer);
    let value = match value {
        Ok(value) => value,
        Err(error) => {
            return (
                None,
                ParseStatus::Failed {
                    error: error.to_string(),
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

        assert!(crate::parse_json_with_comments::<Foo>(&input).is_err());

        let ParseStatus::Failed { error } = result else {
            panic!("Expected parse to fail")
        };

        assert_eq!(
            error,
            "invalid type: string \"foo\", expected usize at line 3 column 24\ninvalid type: integer `3`, expected a boolean at line 4 column 20".to_string()
        )
    }
}
