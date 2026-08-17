use crate::io::{PgBufMutExt, PortalId, StatementId};
use crate::message::{FrontendMessage, FrontendMessageFormat};
use crate::PgValueFormat;
use std::num::Saturating;

/// <https://www.postgresql.org/docs/current/protocol-message-formats.html#PROTOCOL-MESSAGE-FORMATS-BIND>
///
/// ## Note:
///
/// The integer values for number of bind parameters, number of parameter format codes,
/// and number of result format codes all are interpreted as *unsigned*!
#[derive(Debug)]
pub struct Bind<'a> {
    /// The ID of the destination portal (`PortalId::UNNAMED` selects the unnamed portal).
    pub portal: PortalId,

    /// The id of the source prepared statement.
    pub statement: StatementId,

    /// The parameter format codes. Each must presently be zero (text) or one (binary).
    ///
    /// There can be zero to indicate that there are no parameters or that the parameters all use the
    /// default format (text); or one, in which case the specified format code is applied to all
    /// parameters; or it can equal the actual number of parameters.
    pub formats: &'a [PgValueFormat],

    // Note: interpreted as unsigned, as is `formats.len()` and `result_formats.len()`
    /// The number of parameters.
    ///
    /// May be different from `formats.len()`
    pub num_params: u16,

    /// The value of each parameter, in the indicated format.
    pub params: &'a [u8],

    /// The result-column format codes. Each must presently be zero (text) or one (binary).
    ///
    /// There can be zero to indicate that there are no result columns or that the
    /// result columns should all use the default format (text); or one, in which
    /// case the specified format code is applied to all result columns (if any);
    /// or it can equal the actual number of result columns of the query.
    pub result_formats: &'a [PgValueFormat],
}

impl FrontendMessage for Bind<'_> {
    const FORMAT: FrontendMessageFormat = FrontendMessageFormat::Bind;

    fn body_size_hint(&self) -> Saturating<usize> {
        let mut size = Saturating(0);
        size += self.portal.name_len();
        size += self.statement.name_len();

        // Parameter formats and length prefix
        size += 2;
        size += self.formats.len();

        // `num_params`
        size += 2;

        size += self.params.len();

        // Result formats and length prefix
        size += 2;
        size += self.result_formats.len();

        size
    }

    fn encode_body(&self, buf: &mut Vec<u8>) -> Result<(), crate::Error> {
        buf.put_portal_name(self.portal);

        buf.put_statement_name(self.statement);

        // NOTE: the integer values for the number of parameters and format codes in this message
        // are all interpreted as *unsigned*!
        //
        // https://github.com/launchbadge/sqlx/issues/3464
        let formats_len = u16::try_from(self.formats.len()).map_err(|_| {
            err_protocol!("too many parameter format codes ({})", self.formats.len())
        })?;

        buf.extend(formats_len.to_be_bytes());

        for &format in self.formats {
            buf.extend((format as i16).to_be_bytes());
        }

        buf.extend(self.num_params.to_be_bytes());

        buf.extend(self.params);

        let result_formats_len = u16::try_from(self.formats.len())
            .map_err(|_| err_protocol!("too many result format codes ({})", self.formats.len()))?;

        buf.extend(result_formats_len.to_be_bytes());

        for &format in self.result_formats {
            buf.extend((format as i16).to_be_bytes());
        }

        Ok(())
    }
}

// TODO: Unit Test Bind
// TODO: Benchmark Bind
