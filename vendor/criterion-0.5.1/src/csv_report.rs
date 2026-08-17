use crate::error::Result;
use crate::measurement::ValueFormatter;
use crate::report::{BenchmarkId, MeasurementData, Report, ReportContext};
use crate::Throughput;
use csv::Writer;
use std::io::Write;
use std::path::Path;

#[derive(Serialize)]
struct CsvRow<'a> {
    group: &'a str,
    function: Option<&'a str>,
    value: Option<&'a str>,
    throughput_num: Option<&'a str>,
    throughput_type: Option<&'a str>,
    sample_measured_value: f64,
    unit: &'static str,
    iteration_count: u64,
}

struct CsvReportWriter<W: Write> {
    writer: Writer<W>,
}
impl<W: Write> CsvReportWriter<W> {
    fn write_data(
        &mut self,
        id: &BenchmarkId,
        data: &MeasurementData<'_>,
        formatter: &dyn ValueFormatter,
    ) -> Result<()> {
        let mut data_scaled: Vec<f64> = data.sample_times().as_ref().into();
        let unit = formatter.scale_for_machines(&mut data_scaled);
        let group = id.group_id.as_str();
        let function = id.function_id.as_deref();
        let value = id.value_str.as_deref();
        let (throughput_num, throughput_type) = match id.throughput {
            Some(Throughput::Bytes(bytes)) => (Some(format!("{}", bytes)), Some("bytes")),
            Some(Throughput::BytesDecimal(bytes)) => (Some(format!("{}", bytes)), Some("bytes")),
            Some(Throughput::Elements(elems)) => (Some(format!("{}", elems)), Some("elements")),
            None => (None, None),
        };
        let throughput_num = throughput_num.as_deref();

        for (count, measured_value) in data.iter_counts().iter().zip(data_scaled.into_iter()) {
            let row = CsvRow {
                group,
                function,
                value,
                throughput_num,
                throughput_type,
                sample_measured_value: measured_value,
                unit,
                iteration_count: (*count) as u64,
            };
            self.writer.serialize(row)?;
        }
        Ok(())
    }
}

pub struct FileCsvReport;
impl FileCsvReport {
    fn write_file(
        &self,
        path: &Path,
        id: &BenchmarkId,
        measurements: &MeasurementData<'_>,
        formatter: &dyn ValueFormatter,
    ) -> Result<()> {
        let writer = Writer::from_path(path)?;
        let mut writer = CsvReportWriter { writer };
        writer.write_data(id, measurements, formatter)?;
        Ok(())
    }
}

impl Report for FileCsvReport {
    fn measurement_complete(
        &self,
        id: &BenchmarkId,
        context: &ReportContext,
        measurements: &MeasurementData<'_>,
        formatter: &dyn ValueFormatter,
    ) {
        let mut path = context.output_directory.clone();
        path.push(id.as_directory_name());
        path.push("new");
        path.push("raw.csv");
        log_if_err!(self.write_file(&path, id, measurements, formatter));
    }
}
