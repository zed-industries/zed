//! The auto generated API for [`neovim`](crate::neovim::Neovim)
//!
//! Auto generated {{date}}
use futures::io::AsyncWrite;

use crate::{
  error::CallError,
  neovim::*,
  rpc::{unpack::TryUnpack, *},
  Buffer, Tabpage, Window,
};

{% for etype in exttypes %}

impl<W> {{ etype.name }}<W>
  where W: AsyncWrite + Send + Unpin + 'static
  {
    #[must_use]
    pub fn new(code_data: Value, neovim: Neovim<W>) -> {{ etype.name }}<W>
    {
        {{ etype.name }} {
            code_data,
            neovim
        }
    }

    /// Internal value, that represent type
    #[must_use]
    pub fn get_value(&self) -> &Value {
        &self.code_data
    }

    {% for f in functions if f.ext and f.name.startswith(etype.prefix) %}
    /// since: {{f.since}}
    pub async fn {{f.name|replace(etype.prefix, '')}}(&self, {{f.argstring}}) -> Result<{{f.return_type.native_type_ret}}, Box<CallError>>
    {
        self.neovim.call("{{f.name}}",
                          call_args![self.code_data.clone()
                          {% if f.parameters|count > 0 %}
                          , {{ f.parameters|map(attribute = "name")|join(", ") }}
                          {% endif %}
                          ])
                    .await??
                    .try_unpack()
                    .map_err(|v| Box::new(CallError::WrongValueType(v)))
    }
    {% endfor %}
}

{% endfor %}


impl<W> Neovim<W>
where
      W: AsyncWrite + Send + Unpin + 'static,
{
    {% for f in functions if not f.ext %}
    pub async fn {{f.name|replace('nvim_', '')}}(&self, {{f.argstring}}) -> Result<{{f.return_type.native_type_ret}}, Box<CallError>> {
        self.call("{{f.name}}",
                          call_args![{{ f.parameters|map(attribute = "name")|join(", ") }}])
                    .await??
                    .try_unpack()
                    .map_err(|v| Box::new(CallError::WrongValueType(v)))
    }

    {% endfor %}
}
