use std::io::Cursor;
use std::time::Duration;
use std::vec;

use rodio::{ChannelCount, Sample, SampleRate, Source};

pub struct TestSource {
    samples: vec::IntoIter<Sample>,
    channels: ChannelCount,
    sample_rate: SampleRate,
    total_duration: Duration,
}

impl Iterator for TestSource {
    type Item = Sample;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.samples.next()
    }
}

impl ExactSizeIterator for TestSource {
    #[inline]
    fn len(&self) -> usize {
        self.samples.len()
    }
}

impl Source for TestSource {
    #[inline]
    fn current_span_len(&self) -> Option<usize> {
        None // forever
    }

    #[inline]
    fn channels(&self) -> ChannelCount {
        self.channels
    }

    #[inline]
    fn sample_rate(&self) -> SampleRate {
        self.sample_rate
    }

    #[inline]
    fn total_duration(&self) -> Option<Duration> {
        Some(self.total_duration)
    }
}

pub fn music_wav() -> TestSource {
    let data = include_bytes!("../assets/music.wav");
    let cursor = Cursor::new(data);

    let duration = Duration::from_secs(10);
    let sound = rodio::Decoder::new(cursor)
        .expect("music.wav is correctly encoded & wav is supported")
        .take_duration(duration);

    TestSource {
        channels: sound.channels(),
        sample_rate: sound.sample_rate(),
        total_duration: duration,
        samples: sound.into_iter().collect::<Vec<_>>().into_iter(),
    }
}
