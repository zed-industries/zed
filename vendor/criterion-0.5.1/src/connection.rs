use crate::report::BenchmarkId as InternalBenchmarkId;
use crate::Throughput;
use std::cell::RefCell;
use std::convert::TryFrom;
use std::io::{Read, Write};
use std::mem::size_of;
use std::net::TcpStream;

#[derive(Debug)]
pub enum MessageError {
    Deserialization(ciborium::de::Error<std::io::Error>),
    Serialization(ciborium::ser::Error<std::io::Error>),
    Io(std::io::Error),
}
impl From<ciborium::de::Error<std::io::Error>> for MessageError {
    fn from(other: ciborium::de::Error<std::io::Error>) -> Self {
        MessageError::Deserialization(other)
    }
}
impl From<ciborium::ser::Error<std::io::Error>> for MessageError {
    fn from(other: ciborium::ser::Error<std::io::Error>) -> Self {
        MessageError::Serialization(other)
    }
}
impl From<std::io::Error> for MessageError {
    fn from(other: std::io::Error) -> Self {
        MessageError::Io(other)
    }
}
impl std::fmt::Display for MessageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MessageError::Deserialization(error) => write!(
                f,
                "Failed to deserialize message to Criterion.rs benchmark:\n{}",
                error
            ),
            MessageError::Serialization(error) => write!(
                f,
                "Failed to serialize message to Criterion.rs benchmark:\n{}",
                error
            ),
            MessageError::Io(error) => write!(
                f,
                "Failed to read or write message to Criterion.rs benchmark:\n{}",
                error
            ),
        }
    }
}
impl std::error::Error for MessageError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            MessageError::Deserialization(err) => Some(err),
            MessageError::Serialization(err) => Some(err),
            MessageError::Io(err) => Some(err),
        }
    }
}

// Use str::len as a const fn once we bump MSRV over 1.39.
const RUNNER_MAGIC_NUMBER: &str = "cargo-criterion";
const RUNNER_HELLO_SIZE: usize = 15 //RUNNER_MAGIC_NUMBER.len() // magic number
    + (size_of::<u8>() * 3); // version number

const BENCHMARK_MAGIC_NUMBER: &str = "Criterion";
const BENCHMARK_HELLO_SIZE: usize = 9 //BENCHMARK_MAGIC_NUMBER.len() // magic number
    + (size_of::<u8>() * 3) // version number
    + size_of::<u16>() // protocol version
    + size_of::<u16>(); // protocol format
const PROTOCOL_VERSION: u16 = 1;
const PROTOCOL_FORMAT: u16 = 1;

#[derive(Debug)]
struct InnerConnection {
    socket: TcpStream,
    receive_buffer: Vec<u8>,
    send_buffer: Vec<u8>,
    // runner_version: [u8; 3],
}
impl InnerConnection {
    pub fn new(mut socket: TcpStream) -> Result<Self, std::io::Error> {
        // read the runner-hello
        let mut hello_buf = [0u8; RUNNER_HELLO_SIZE];
        socket.read_exact(&mut hello_buf)?;
        assert_eq!(
            &hello_buf[0..RUNNER_MAGIC_NUMBER.len()],
            RUNNER_MAGIC_NUMBER.as_bytes(),
            "Not connected to cargo-criterion."
        );

        let i = RUNNER_MAGIC_NUMBER.len();
        let runner_version = [hello_buf[i], hello_buf[i + 1], hello_buf[i + 2]];

        info!("Runner version: {:?}", runner_version);

        // now send the benchmark-hello
        let mut hello_buf = [0u8; BENCHMARK_HELLO_SIZE];
        hello_buf[0..BENCHMARK_MAGIC_NUMBER.len()]
            .copy_from_slice(BENCHMARK_MAGIC_NUMBER.as_bytes());
        let mut i = BENCHMARK_MAGIC_NUMBER.len();
        hello_buf[i] = env!("CARGO_PKG_VERSION_MAJOR").parse().unwrap();
        hello_buf[i + 1] = env!("CARGO_PKG_VERSION_MINOR").parse().unwrap();
        hello_buf[i + 2] = env!("CARGO_PKG_VERSION_PATCH").parse().unwrap();
        i += 3;
        hello_buf[i..i + 2].clone_from_slice(&PROTOCOL_VERSION.to_be_bytes());
        i += 2;
        hello_buf[i..i + 2].clone_from_slice(&PROTOCOL_FORMAT.to_be_bytes());

        socket.write_all(&hello_buf)?;

        Ok(InnerConnection {
            socket,
            receive_buffer: vec![],
            send_buffer: vec![],
            // runner_version,
        })
    }

    #[allow(dead_code)]
    pub fn recv(&mut self) -> Result<IncomingMessage, MessageError> {
        let mut length_buf = [0u8; 4];
        self.socket.read_exact(&mut length_buf)?;
        let length = u32::from_be_bytes(length_buf);
        self.receive_buffer.resize(length as usize, 0u8);
        self.socket.read_exact(&mut self.receive_buffer)?;
        let value = ciborium::de::from_reader(&self.receive_buffer[..])?;
        Ok(value)
    }

    pub fn send(&mut self, message: &OutgoingMessage) -> Result<(), MessageError> {
        self.send_buffer.truncate(0);
        ciborium::ser::into_writer(message, &mut self.send_buffer)?;
        let size = u32::try_from(self.send_buffer.len()).unwrap();
        let length_buf = size.to_be_bytes();
        self.socket.write_all(&length_buf)?;
        self.socket.write_all(&self.send_buffer)?;
        Ok(())
    }
}

/// This is really just a holder to allow us to send messages through a shared reference to the
/// connection.
#[derive(Debug)]
pub struct Connection {
    inner: RefCell<InnerConnection>,
}
impl Connection {
    pub fn new(socket: TcpStream) -> Result<Self, std::io::Error> {
        Ok(Connection {
            inner: RefCell::new(InnerConnection::new(socket)?),
        })
    }

    #[allow(dead_code)]
    pub fn recv(&self) -> Result<IncomingMessage, MessageError> {
        self.inner.borrow_mut().recv()
    }

    pub fn send(&self, message: &OutgoingMessage) -> Result<(), MessageError> {
        self.inner.borrow_mut().send(message)
    }

    pub fn serve_value_formatter(
        &self,
        formatter: &dyn crate::measurement::ValueFormatter,
    ) -> Result<(), MessageError> {
        loop {
            let response = match self.recv()? {
                IncomingMessage::FormatValue { value } => OutgoingMessage::FormattedValue {
                    value: formatter.format_value(value),
                },
                IncomingMessage::FormatThroughput { value, throughput } => {
                    OutgoingMessage::FormattedValue {
                        value: formatter.format_throughput(&throughput, value),
                    }
                }
                IncomingMessage::ScaleValues {
                    typical_value,
                    mut values,
                } => {
                    let unit = formatter.scale_values(typical_value, &mut values);
                    OutgoingMessage::ScaledValues {
                        unit,
                        scaled_values: values,
                    }
                }
                IncomingMessage::ScaleThroughputs {
                    typical_value,
                    throughput,
                    mut values,
                } => {
                    let unit = formatter.scale_throughputs(typical_value, &throughput, &mut values);
                    OutgoingMessage::ScaledValues {
                        unit,
                        scaled_values: values,
                    }
                }
                IncomingMessage::ScaleForMachines { mut values } => {
                    let unit = formatter.scale_for_machines(&mut values);
                    OutgoingMessage::ScaledValues {
                        unit,
                        scaled_values: values,
                    }
                }
                IncomingMessage::Continue => break,
                _ => panic!(),
            };
            self.send(&response)?;
        }
        Ok(())
    }
}

/// Enum defining the messages we can receive
#[derive(Debug, Deserialize)]
pub enum IncomingMessage {
    // Value formatter requests
    FormatValue {
        value: f64,
    },
    FormatThroughput {
        value: f64,
        throughput: Throughput,
    },
    ScaleValues {
        typical_value: f64,
        values: Vec<f64>,
    },
    ScaleThroughputs {
        typical_value: f64,
        values: Vec<f64>,
        throughput: Throughput,
    },
    ScaleForMachines {
        values: Vec<f64>,
    },
    Continue,

    __Other,
}

/// Enum defining the messages we can send
#[derive(Debug, Serialize)]
pub enum OutgoingMessage<'a> {
    BeginningBenchmarkGroup {
        group: &'a str,
    },
    FinishedBenchmarkGroup {
        group: &'a str,
    },
    BeginningBenchmark {
        id: RawBenchmarkId,
    },
    SkippingBenchmark {
        id: RawBenchmarkId,
    },
    Warmup {
        id: RawBenchmarkId,
        nanos: f64,
    },
    MeasurementStart {
        id: RawBenchmarkId,
        sample_count: u64,
        estimate_ns: f64,
        iter_count: u64,
    },
    MeasurementComplete {
        id: RawBenchmarkId,
        iters: &'a [f64],
        times: &'a [f64],
        plot_config: PlotConfiguration,
        sampling_method: SamplingMethod,
        benchmark_config: BenchmarkConfig,
    },
    // value formatter responses
    FormattedValue {
        value: String,
    },
    ScaledValues {
        scaled_values: Vec<f64>,
        unit: &'a str,
    },
}

// Also define serializable variants of certain things, either to avoid leaking
// serializability into the public interface or because the serialized form
// is a bit different from the regular one.

#[derive(Debug, Serialize)]
pub struct RawBenchmarkId {
    group_id: String,
    function_id: Option<String>,
    value_str: Option<String>,
    throughput: Vec<Throughput>,
}
impl From<&InternalBenchmarkId> for RawBenchmarkId {
    fn from(other: &InternalBenchmarkId) -> RawBenchmarkId {
        RawBenchmarkId {
            group_id: other.group_id.clone(),
            function_id: other.function_id.clone(),
            value_str: other.value_str.clone(),
            throughput: other.throughput.iter().cloned().collect(),
        }
    }
}

#[derive(Debug, Serialize)]
pub enum AxisScale {
    Linear,
    Logarithmic,
}
impl From<crate::AxisScale> for AxisScale {
    fn from(other: crate::AxisScale) -> Self {
        match other {
            crate::AxisScale::Linear => AxisScale::Linear,
            crate::AxisScale::Logarithmic => AxisScale::Logarithmic,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct PlotConfiguration {
    summary_scale: AxisScale,
}
impl From<&crate::PlotConfiguration> for PlotConfiguration {
    fn from(other: &crate::PlotConfiguration) -> Self {
        PlotConfiguration {
            summary_scale: other.summary_scale.into(),
        }
    }
}

#[derive(Debug, Serialize)]
struct Duration {
    secs: u64,
    nanos: u32,
}
impl From<std::time::Duration> for Duration {
    fn from(other: std::time::Duration) -> Self {
        Duration {
            secs: other.as_secs(),
            nanos: other.subsec_nanos(),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct BenchmarkConfig {
    confidence_level: f64,
    measurement_time: Duration,
    noise_threshold: f64,
    nresamples: usize,
    sample_size: usize,
    significance_level: f64,
    warm_up_time: Duration,
}
impl From<&crate::benchmark::BenchmarkConfig> for BenchmarkConfig {
    fn from(other: &crate::benchmark::BenchmarkConfig) -> Self {
        BenchmarkConfig {
            confidence_level: other.confidence_level,
            measurement_time: other.measurement_time.into(),
            noise_threshold: other.noise_threshold,
            nresamples: other.nresamples,
            sample_size: other.sample_size,
            significance_level: other.significance_level,
            warm_up_time: other.warm_up_time.into(),
        }
    }
}

/// Currently not used; defined for forwards compatibility with cargo-criterion.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum SamplingMethod {
    Linear,
    Flat,
}
impl From<crate::ActualSamplingMode> for SamplingMethod {
    fn from(other: crate::ActualSamplingMode) -> Self {
        match other {
            crate::ActualSamplingMode::Flat => SamplingMethod::Flat,
            crate::ActualSamplingMode::Linear => SamplingMethod::Linear,
        }
    }
}
