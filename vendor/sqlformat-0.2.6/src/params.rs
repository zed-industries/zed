use crate::tokenizer::Token;
use crate::QueryParams;

pub(crate) struct Params<'a> {
    index: usize,
    params: &'a QueryParams,
}

impl<'a> Params<'a> {
    pub fn new(params: &'a QueryParams) -> Self {
        Params { index: 0, params }
    }

    pub fn get(&mut self, token: &'a Token<'a>) -> &'a str {
        let named_placeholder_token = token.key.as_ref().map_or(false, |key| key.named() != "");

        match self.params {
            QueryParams::Named(params) => token
                .key
                .as_ref()
                .and_then(|key| {
                    params
                        .iter()
                        .find(|param| param.0 == key.named())
                        .map(|param| param.1.as_str())
                })
                .unwrap_or(token.value),
            QueryParams::Indexed(params) if !named_placeholder_token => {
                if let Some(key) = token.key.as_ref().and_then(|key| key.indexed()) {
                    params
                        .get(key)
                        .map(|param| param.as_str())
                        .unwrap_or(token.value)
                } else {
                    let value = params
                        .get(self.index)
                        .map(|param| param.as_str())
                        .unwrap_or(token.value);
                    self.index += 1;
                    value
                }
            }
            _ => token.value,
        }
    }
}
