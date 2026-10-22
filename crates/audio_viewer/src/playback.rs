use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::time::Duration;

use anyhow::{Context as _, Result, anyhow};
use rodio::{Decoder, DeviceSinkBuilder, MixerDeviceSink, Player, Source as _};

pub type TrackDecoder = Decoder<BufReader<File>>;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TrackInfo {
    /// `None` when the container does not record the length, such as some MP3 files without a
    /// Xing header.
    pub duration: Option<Duration>,
    pub sample_rate: u32,
    pub channels: u16,
}

pub fn open_track(path: &Path) -> Result<(TrackDecoder, TrackInfo)> {
    let file = File::open(path).with_context(|| format!("opening {}", path.display()))?;
    let byte_len = file.metadata()?.len();
    let mut builder = Decoder::builder()
        .with_data(BufReader::new(file))
        .with_byte_len(byte_len)
        .with_seekable(true);
    if let Some(extension) = path.extension().and_then(|extension| extension.to_str()) {
        builder = builder.with_hint(extension);
    }
    let decoder = builder
        .build()
        .map_err(|error| anyhow!("This audio file cannot be decoded: {error}"))?;
    let info = TrackInfo {
        duration: decoder.total_duration(),
        sample_rate: decoder.sample_rate().get(),
        channels: decoder.channels().get(),
    };
    Ok((decoder, info))
}

/// An open output device playing one track.
pub struct Playback {
    // Dropping the sink closes the output device, so it lives as long as the player.
    _sink: MixerDeviceSink,
    player: Player,
}

impl Playback {
    pub fn start(decoder: TrackDecoder, volume: f32) -> Result<Self> {
        let mut sink = DeviceSinkBuilder::open_default_sink()
            .map_err(|error| anyhow!("Cannot open the audio output: {error}"))?;
        sink.log_on_drop(false);
        let player = Player::connect_new(sink.mixer());
        player.set_volume(volume);
        player.append(decoder);
        Ok(Self {
            _sink: sink,
            player,
        })
    }

    /// Queues the track again after it played to the end.
    pub fn restart(&self, decoder: TrackDecoder) {
        self.player.append(decoder);
        self.player.play();
    }

    pub fn play(&self) {
        self.player.play();
    }

    pub fn pause(&self) {
        self.player.pause();
    }

    pub fn is_paused(&self) -> bool {
        self.player.is_paused()
    }

    pub fn is_finished(&self) -> bool {
        self.player.empty()
    }

    pub fn position(&self) -> Duration {
        self.player.get_pos()
    }

    pub fn seek(&self, position: Duration) -> Result<()> {
        self.player
            .try_seek(position)
            .map_err(|error| anyhow!("Seeking failed: {error}"))
    }

    pub fn set_volume(&self, volume: f32) {
        self.player.set_volume(volume);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Writes a mono 16-bit PCM WAV file with `sample_count` samples of silence.
    fn write_wav(path: &Path, sample_rate: u32, sample_count: u32) {
        let data_len = sample_count * 2;
        let mut bytes = Vec::new();
        bytes.extend_from_slice(b"RIFF");
        bytes.extend_from_slice(&(36 + data_len).to_le_bytes());
        bytes.extend_from_slice(b"WAVEfmt ");
        bytes.extend_from_slice(&16u32.to_le_bytes());
        bytes.extend_from_slice(&1u16.to_le_bytes());
        bytes.extend_from_slice(&1u16.to_le_bytes());
        bytes.extend_from_slice(&sample_rate.to_le_bytes());
        bytes.extend_from_slice(&(sample_rate * 2).to_le_bytes());
        bytes.extend_from_slice(&2u16.to_le_bytes());
        bytes.extend_from_slice(&16u16.to_le_bytes());
        bytes.extend_from_slice(b"data");
        bytes.extend_from_slice(&data_len.to_le_bytes());
        bytes.resize(bytes.len() + data_len as usize, 0);
        std::fs::write(path, bytes).expect("test WAV should be written");
    }

    #[test]
    fn test_open_track_reads_track_info() {
        let directory = tempfile::tempdir().expect("temp dir should be created");
        let path = directory.path().join("silence.wav");
        write_wav(&path, 8000, 16000);

        let (mut decoder, info) = open_track(&path).expect("WAV should decode");
        assert_eq!(
            info,
            TrackInfo {
                duration: Some(Duration::from_secs(2)),
                sample_rate: 8000,
                channels: 1,
            }
        );
        decoder
            .try_seek(Duration::from_secs(1))
            .expect("WAV should be seekable");
        assert_eq!(decoder.next(), Some(0.0));
    }

    #[test]
    fn test_open_track_rejects_unknown_data() {
        let directory = tempfile::tempdir().expect("temp dir should be created");
        let path = directory.path().join("noise.mp3");
        std::fs::write(&path, b"this is not audio").expect("test file should be written");

        let error = open_track(&path).err().expect("unknown data should fail");
        assert!(
            error
                .to_string()
                .starts_with("This audio file cannot be decoded"),
            "unexpected error: {error}"
        );
    }
}
