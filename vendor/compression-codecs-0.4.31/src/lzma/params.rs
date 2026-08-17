//! Clone-able structs to build the non-clone-ables in liblzma

use std::convert::TryFrom;
#[cfg(feature = "xz-parallel")]
use std::num::NonZeroU32;

/// Used to control how the LZMA stream is created.
#[derive(Debug, Clone)]
pub enum LzmaEncoderParams {
    Easy {
        preset: u32,
        check: liblzma::stream::Check,
    },
    Lzma {
        options: LzmaOptions,
    },
    Raw {
        filters: LzmaFilters,
    },
    Stream {
        filters: LzmaFilters,
        check: liblzma::stream::Check,
    },

    #[cfg(feature = "xz-parallel")]
    MultiThread {
        builder: MtStreamBuilder,
    },
}

impl TryFrom<&LzmaEncoderParams> for liblzma::stream::Stream {
    type Error = liblzma::stream::Error;

    fn try_from(value: &LzmaEncoderParams) -> Result<Self, Self::Error> {
        let stream = match value {
            LzmaEncoderParams::Easy { preset, check } => Self::new_easy_encoder(*preset, *check)?,
            LzmaEncoderParams::Lzma { options } => {
                let options = liblzma::stream::LzmaOptions::try_from(options)?;
                Self::new_lzma_encoder(&options)?
            }
            LzmaEncoderParams::Raw { filters } => {
                let filters = liblzma::stream::Filters::try_from(filters)?;
                Self::new_raw_encoder(&filters)?
            }
            LzmaEncoderParams::Stream { filters, check } => {
                let filters = liblzma::stream::Filters::try_from(filters)?;
                Self::new_stream_encoder(&filters, *check)?
            }

            #[cfg(feature = "xz-parallel")]
            LzmaEncoderParams::MultiThread { builder } => {
                let builder = liblzma::stream::MtStreamBuilder::try_from(builder)?;
                builder.encoder()?
            }
        };

        Ok(stream)
    }
}

/// Directly translate to how the stream is constructed
#[derive(Clone, Debug)]
pub enum LzmaDecoderParams {
    Auto {
        mem_limit: u64,
        flags: u32,
    },
    Lzip {
        mem_limit: u64,
        flags: u32,
    },
    Lzma {
        mem_limit: u64,
    },
    Raw {
        filters: LzmaFilters,
    },
    Stream {
        mem_limit: u64,
        flags: u32,
    },

    #[cfg(feature = "xz-parallel")]
    MultiThread {
        builder: MtStreamBuilder,
    },
}

impl TryFrom<&LzmaDecoderParams> for liblzma::stream::Stream {
    type Error = liblzma::stream::Error;

    fn try_from(value: &LzmaDecoderParams) -> Result<Self, Self::Error> {
        let stream = match value {
            LzmaDecoderParams::Auto { mem_limit, flags } => {
                Self::new_auto_decoder(*mem_limit, *flags)?
            }
            LzmaDecoderParams::Lzip { mem_limit, flags } => {
                Self::new_lzip_decoder(*mem_limit, *flags)?
            }
            LzmaDecoderParams::Lzma { mem_limit } => Self::new_lzma_decoder(*mem_limit)?,
            LzmaDecoderParams::Stream { mem_limit, flags } => {
                Self::new_stream_decoder(*mem_limit, *flags)?
            }
            LzmaDecoderParams::Raw { filters } => {
                let filters = liblzma::stream::Filters::try_from(filters)?;
                Self::new_raw_decoder(&filters)?
            }
            #[cfg(feature = "xz-parallel")]
            LzmaDecoderParams::MultiThread { builder } => {
                let builder = liblzma::stream::MtStreamBuilder::try_from(builder)?;
                builder.decoder()?
            }
        };

        Ok(stream)
    }
}

/// Clone-able `liblzma::Filters`.
#[derive(Default, Clone, Debug)]
pub struct LzmaFilters {
    filters: Vec<LzmaFilter>,
}

impl LzmaFilters {
    /// Add `LzmaFilter` to the collection
    pub fn add_filter(mut self, filter: LzmaFilter) -> Self {
        self.filters.push(filter);
        self
    }
}

/// An individual filter directly corresponding to liblzma Filters method calls
#[derive(Debug, Clone)]
pub enum LzmaFilter {
    Arm(Option<Vec<u8>>),
    Arm64(Option<Vec<u8>>),
    ArmThumb(Option<Vec<u8>>),
    Delta(Option<Vec<u8>>),
    Ia64(Option<Vec<u8>>),
    Lzma1(LzmaOptions),
    Lzma1Properties(Vec<u8>),
    Lzma2(LzmaOptions),
    Lzma2Properties(Vec<u8>),
    PowerPc(Option<Vec<u8>>),
    Sparc(Option<Vec<u8>>),
    X86(Option<Vec<u8>>),
}

impl TryFrom<&LzmaFilters> for liblzma::stream::Filters {
    type Error = liblzma::stream::Error;

    fn try_from(value: &LzmaFilters) -> Result<Self, Self::Error> {
        let mut filters = liblzma::stream::Filters::new();
        for f in value.filters.iter() {
            match f {
                LzmaFilter::Arm(Some(p)) => filters.arm_properties(p)?,
                LzmaFilter::Arm(None) => filters.arm(),
                LzmaFilter::Arm64(Some(p)) => filters.arm64_properties(p)?,
                LzmaFilter::Arm64(None) => filters.arm64(),
                LzmaFilter::ArmThumb(Some(p)) => filters.arm_thumb_properties(p)?,
                LzmaFilter::ArmThumb(None) => filters.arm_thumb(),
                LzmaFilter::Delta(Some(p)) => filters.delta_properties(p)?,
                LzmaFilter::Delta(None) => filters.delta(),
                LzmaFilter::Ia64(Some(p)) => filters.ia64_properties(p)?,
                LzmaFilter::Ia64(None) => filters.ia64(),
                LzmaFilter::Lzma1(opts) => {
                    let opts = liblzma::stream::LzmaOptions::try_from(opts)?;
                    filters.lzma1(&opts)
                }
                LzmaFilter::Lzma1Properties(p) => filters.lzma1_properties(p)?,
                LzmaFilter::Lzma2(opts) => {
                    let opts = liblzma::stream::LzmaOptions::try_from(opts)?;
                    filters.lzma2(&opts)
                }
                LzmaFilter::Lzma2Properties(p) => filters.lzma2_properties(p)?,
                LzmaFilter::PowerPc(Some(p)) => filters.powerpc_properties(p)?,
                LzmaFilter::PowerPc(None) => filters.powerpc(),
                LzmaFilter::Sparc(Some(p)) => filters.sparc_properties(p)?,
                LzmaFilter::Sparc(None) => filters.sparc(),
                LzmaFilter::X86(Some(p)) => filters.x86_properties(p)?,
                LzmaFilter::X86(None) => filters.x86(),
            };
        }

        Ok(filters)
    }
}

/// A builder for liblzma::LzmaOptions, so that it can be cloned
#[derive(Default, Clone)]
pub struct LzmaOptions {
    preset: Option<u32>,
    depth: Option<u32>,
    dict_size: Option<u32>,
    literal_context_bits: Option<u32>,
    literal_position_bits: Option<u32>,
    match_finder: Option<liblzma::stream::MatchFinder>,
    mode: Option<liblzma::stream::Mode>,
    nice_len: Option<u32>,
    position_bits: Option<u32>,
}

impl std::fmt::Debug for LzmaOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let match_finder = self.match_finder.map(|m| match m {
            liblzma::stream::MatchFinder::HashChain3 => "HashChain3",
            liblzma::stream::MatchFinder::HashChain4 => "HashChain4",
            liblzma::stream::MatchFinder::BinaryTree2 => "BTree2",
            liblzma::stream::MatchFinder::BinaryTree3 => "BTree3",
            liblzma::stream::MatchFinder::BinaryTree4 => "BTree4",
        });

        let mode = self.mode.map(|m| match m {
            liblzma::stream::Mode::Fast => "Fast",
            liblzma::stream::Mode::Normal => "Normal",
        });

        f.debug_struct("LzmaOptions")
            .field("preset", &self.preset)
            .field("depth", &self.depth)
            .field("dict_size", &self.dict_size)
            .field("literal_context_bits", &self.literal_context_bits)
            .field("literal_position_bits", &self.literal_position_bits)
            .field("match_finder", &match_finder)
            .field("mode", &mode)
            .field("nice_len", &self.nice_len)
            .field("position_bits", &self.position_bits)
            .finish()
    }
}

impl LzmaOptions {
    pub fn preset(mut self, value: u32) -> Self {
        self.preset = Some(value);
        self
    }
    pub fn depth(mut self, value: u32) -> Self {
        self.depth = Some(value);
        self
    }
    pub fn dict_size(mut self, value: u32) -> Self {
        self.dict_size = Some(value);
        self
    }
    pub fn literal_context_bits(mut self, value: u32) -> Self {
        self.literal_context_bits = Some(value);
        self
    }
    pub fn literal_position_bits(mut self, value: u32) -> Self {
        self.literal_position_bits = Some(value);
        self
    }
    pub fn match_finder(mut self, value: liblzma::stream::MatchFinder) -> Self {
        self.match_finder = Some(value);
        self
    }
    pub fn mode(mut self, value: liblzma::stream::Mode) -> Self {
        self.mode = Some(value);
        self
    }
    pub fn nice_len(mut self, value: u32) -> Self {
        self.nice_len = Some(value);
        self
    }
    pub fn position_bits(mut self, value: u32) -> Self {
        self.position_bits = Some(value);
        self
    }
}

impl TryFrom<&LzmaOptions> for liblzma::stream::LzmaOptions {
    type Error = liblzma::stream::Error;

    fn try_from(value: &LzmaOptions) -> Result<Self, Self::Error> {
        let mut s = match value.preset {
            Some(preset) => liblzma::stream::LzmaOptions::new_preset(preset)?,
            None => liblzma::stream::LzmaOptions::new(),
        };
        if let Some(depth) = value.depth {
            s.depth(depth);
        }
        if let Some(dict_size) = value.dict_size {
            s.dict_size(dict_size);
        }
        if let Some(bits) = value.literal_context_bits {
            s.literal_context_bits(bits);
        }
        if let Some(bits) = value.literal_position_bits {
            s.literal_position_bits(bits);
        }
        if let Some(mf) = value.match_finder {
            s.match_finder(mf);
        }
        if let Some(mode) = value.mode {
            s.mode(mode);
        }
        if let Some(len) = value.nice_len {
            s.nice_len(len);
        }

        if let Some(bits) = value.position_bits {
            s.position_bits(bits);
        }

        Ok(s)
    }
}

#[cfg(feature = "xz-parallel")]
#[derive(Default, Clone)]
/// Used to build a clonable mt stream builder
pub struct MtStreamBuilder {
    block_size: Option<u64>,
    preset: Option<u32>,
    check: Option<liblzma::stream::Check>,
    filters: Option<LzmaFilters>,
    mem_limit_stop: Option<u64>,
    mem_limit_threading: Option<u64>,
    threads: Option<NonZeroU32>,
    timeout_ms: Option<u32>,
}

#[cfg(feature = "xz-parallel")]
impl std::fmt::Debug for MtStreamBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let check = self.check.map(|s| match s {
            liblzma::stream::Check::None => "None",
            liblzma::stream::Check::Crc32 => "Crc32",
            liblzma::stream::Check::Crc64 => "Crc64",
            liblzma::stream::Check::Sha256 => "Sha256",
        });
        f.debug_struct("MtStreamBuilder")
            .field("block_size", &self.block_size)
            .field("preset", &self.preset)
            .field("check", &check)
            .field("filters", &self.filters)
            .field("mem_limit_stop", &self.mem_limit_stop)
            .field("mem_limit_threading", &self.mem_limit_threading)
            .field("threads", &self.threads)
            .field("timeout_ms", &self.timeout_ms)
            .finish()
    }
}

#[cfg(feature = "xz-parallel")]
impl MtStreamBuilder {
    pub fn block_size(&mut self, block_size: u64) -> &mut Self {
        self.block_size = Some(block_size);
        self
    }
    pub fn preset(&mut self, preset: u32) -> &mut Self {
        self.preset = Some(preset);
        self
    }
    pub fn check(&mut self, check: liblzma::stream::Check) -> &mut Self {
        self.check = Some(check);
        self
    }

    pub fn filters(&mut self, filters: LzmaFilters) -> &mut Self {
        self.filters = Some(filters);
        self
    }
    pub fn mem_limit_stop(&mut self, mem_limit_stop: u64) -> &mut Self {
        self.mem_limit_stop = Some(mem_limit_stop);
        self
    }
    pub fn mem_limit_threading(&mut self, mem_limit_threading: u64) -> &mut Self {
        self.mem_limit_threading = Some(mem_limit_threading);
        self
    }
    pub fn threads(&mut self, threads: NonZeroU32) -> &mut Self {
        self.threads = Some(threads);
        self
    }
    pub fn timeout_ms(&mut self, timeout_ms: u32) -> &mut Self {
        self.timeout_ms = Some(timeout_ms);
        self
    }
}

#[cfg(feature = "xz-parallel")]
impl TryFrom<&MtStreamBuilder> for liblzma::stream::MtStreamBuilder {
    type Error = liblzma::stream::Error;

    fn try_from(value: &MtStreamBuilder) -> Result<Self, Self::Error> {
        let mut mt = liblzma::stream::MtStreamBuilder::new();
        if let Some(block_size) = value.block_size {
            mt.block_size(block_size);
        }
        if let Some(preset) = value.preset {
            mt.preset(preset);
        }
        if let Some(check) = value.check {
            mt.check(check);
        }
        if let Some(filters) = &value.filters {
            let filters = liblzma::stream::Filters::try_from(filters)?;
            mt.filters(filters);
        }
        if let Some(memlimit) = value.mem_limit_stop {
            mt.memlimit_stop(memlimit);
        }
        if let Some(memlimit) = value.mem_limit_threading {
            mt.memlimit_threading(memlimit);
        }
        if let Some(threads) = value.threads {
            mt.threads(threads.get());
        }
        if let Some(timeout) = value.timeout_ms {
            mt.timeout_ms(timeout);
        }
        Ok(mt)
    }
}
