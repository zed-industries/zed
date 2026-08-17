// Copyright 2019 TiKV Project Authors. Licensed under Apache-2.0.

#[cfg(feature = "protobuf")]
mod pb;
mod text;

#[cfg(feature = "protobuf")]
pub use self::pb::{ProtobufEncoder, PROTOBUF_FORMAT};
pub use self::text::{TextEncoder, TEXT_FORMAT};

use std::io::Write;

use crate::errors::{Error, Result};
use crate::proto::MetricFamily;

/// An interface for encoding metric families into an underlying wire protocol.
pub trait Encoder {
    /// `encode` converts a slice of MetricFamily proto messages into target
    /// format and writes the resulting lines to `writer`. This function does not
    /// perform checks on the content of the metrics and label names,
    /// i.e. invalid metrics or label names will result in invalid text format
    /// output.
    fn encode<W: Write>(&self, mfs: &[MetricFamily], writer: &mut W) -> Result<()>;

    /// `format_type` returns target format.
    fn format_type(&self) -> &str;
}

fn check_metric_family(mf: &MetricFamily) -> Result<()> {
    if mf.get_metric().is_empty() {
        return Err(Error::Msg(format!("MetricFamily has no metrics: {:?}", mf)));
    }
    if mf.name().is_empty() {
        return Err(Error::Msg(format!("MetricFamily has no name: {:?}", mf)));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::counter::CounterVec;
    use crate::encoder::Encoder;
    use crate::metrics::Collector;
    use crate::metrics::Opts;

    #[test]
    #[cfg(feature = "protobuf")]
    fn test_bad_proto_metrics() {
        let mut writer = Vec::<u8>::new();
        let pb_encoder = ProtobufEncoder::new();
        let cv = CounterVec::new(
            Opts::new("test_counter_vec", "help information"),
            &["labelname"],
        )
        .unwrap();

        // Empty metrics
        let mfs = cv.collect();
        check_metric_family(&mfs[0]).unwrap_err();
        pb_encoder.encode(&mfs, &mut writer).unwrap_err();
        assert_eq!(writer.len(), 0);

        // Add a sub metric
        cv.with_label_values(&["foo"]).inc();
        let mut mfs = cv.collect();

        // Empty name
        (&mut mfs[0]).clear_name();
        check_metric_family(&mfs[0]).unwrap_err();
        pb_encoder.encode(&mfs, &mut writer).unwrap_err();
        assert_eq!(writer.len(), 0);
    }

    #[test]
    fn test_bad_text_metrics() {
        let mut writer = Vec::<u8>::new();
        let text_encoder = TextEncoder::new();
        let cv = CounterVec::new(
            Opts::new("test_counter_vec", "help information"),
            &["labelname"],
        )
        .unwrap();

        // Empty metrics
        let mfs = cv.collect();
        check_metric_family(&mfs[0]).unwrap_err();
        text_encoder.encode(&mfs, &mut writer).unwrap_err();
        assert_eq!(writer.len(), 0);

        // Add a sub metric
        cv.with_label_values(&["foo"]).inc();
        let mut mfs = cv.collect();

        // Empty name
        (&mut mfs[0]).clear_name();
        check_metric_family(&mfs[0]).unwrap_err();
        text_encoder.encode(&mfs, &mut writer).unwrap_err();
        assert_eq!(writer.len(), 0);
    }
}
