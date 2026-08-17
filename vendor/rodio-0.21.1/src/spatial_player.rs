use std::f32;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use dasp_sample::FromSample;

use crate::mixer::Mixer;
use crate::source::{SeekError, Spatial};
use crate::{Float, Player, Source};

/// A sink that allows changing the position of the source and the listeners
/// ears while playing. The sources played are then transformed to give a simple
/// spatial effect. See [`Spatial`] for details.
pub struct SpatialPlayer {
    player: Player,
    positions: Arc<Mutex<SoundPositions>>,
}

struct SoundPositions {
    emitter_position: [f32; 3],
    left_ear: [f32; 3],
    right_ear: [f32; 3],
}

impl SpatialPlayer {
    /// Builds a new `SpatialPlayer`.
    pub fn connect_new(
        mixer: &Mixer,
        emitter_position: [f32; 3],
        left_ear: [f32; 3],
        right_ear: [f32; 3],
    ) -> SpatialPlayer {
        SpatialPlayer {
            player: Player::connect_new(mixer),
            positions: Arc::new(Mutex::new(SoundPositions {
                emitter_position,
                left_ear,
                right_ear,
            })),
        }
    }

    /// Sets the position of the sound emitter in 3 dimensional space.
    pub fn set_emitter_position(&self, pos: [f32; 3]) {
        self.positions.lock().unwrap().emitter_position = pos;
    }

    /// Sets the position of the left ear in 3 dimensional space.
    pub fn set_left_ear_position(&self, pos: [f32; 3]) {
        self.positions.lock().unwrap().left_ear = pos;
    }

    /// Sets the position of the right ear in 3 dimensional space.
    pub fn set_right_ear_position(&self, pos: [f32; 3]) {
        self.positions.lock().unwrap().right_ear = pos;
    }

    /// Appends a sound to the queue of sounds to play.
    #[inline]
    pub fn append<S>(&self, source: S)
    where
        S: Source + Send + 'static,
        f32: FromSample<S::Item>,
    {
        let positions = self.positions.clone();
        let pos_lock = self.positions.lock().unwrap();
        let source = Spatial::new(
            source,
            pos_lock.emitter_position,
            pos_lock.left_ear,
            pos_lock.right_ear,
        )
        .periodic_access(Duration::from_millis(10), move |i| {
            let pos = positions.lock().unwrap();
            i.set_positions(pos.emitter_position, pos.left_ear, pos.right_ear);
        });
        self.player.append(source);
    }

    // Gets the volume of the sound.
    ///
    /// The value `1.0` is the "normal" volume (unfiltered input). Any value other than 1.0 will
    /// multiply each sample by this value.
    #[inline]
    pub fn volume(&self) -> Float {
        self.player.volume()
    }

    /// Changes the volume of the sound.
    ///
    /// The value `1.0` is the "normal" volume (unfiltered input). Any value other than 1.0 will
    /// multiply each sample by this value.
    #[inline]
    pub fn set_volume(&self, value: Float) {
        self.player.set_volume(value);
    }

    /// Changes the play speed of the sound. Does not adjust the samples, only the playback speed.
    ///
    /// # Note:
    /// 1. **Increasing the speed will increase the pitch by the same factor**
    /// - If you set the speed to 0.5 this will halve the frequency of the sound
    ///   lowering its pitch.
    /// - If you set the speed to 2 the frequency will double raising the
    ///   pitch of the sound.
    /// 2. **Change in the speed affect the total duration inversely**
    /// - If you set the speed to 0.5, the total duration will be twice as long.
    /// - If you set the speed to 2 the total duration will be halve of what it
    ///   was.
    ///
    /// See [`Speed`](crate::source::Speed) for details
    #[inline]
    pub fn speed(&self) -> f32 {
        self.player.speed()
    }

    /// Changes the speed of the sound.
    ///
    /// The value `1.0` is the "normal" speed (unfiltered input). Any value other than `1.0` will
    /// change the play speed of the sound.
    #[inline]
    pub fn set_speed(&self, value: f32) {
        self.player.set_speed(value)
    }

    /// Resumes playback of a paused sound.
    ///
    /// No effect if not paused.
    #[inline]
    pub fn play(&self) {
        self.player.play();
    }

    /// Pauses playback of this player.
    ///
    /// No effect if already paused.
    ///
    /// A paused sound can be resumed with `play()`.
    pub fn pause(&self) {
        self.player.pause();
    }

    /// Gets if a sound is paused
    ///
    /// Sounds can be paused and resumed using pause() and play(). This gets if a sound is paused.
    pub fn is_paused(&self) -> bool {
        self.player.is_paused()
    }

    /// Removes all currently loaded `Source`s from the `SpatialPlayer` and pauses it.
    ///
    /// See `pause()` for information about pausing a `Player`.
    #[inline]
    pub fn clear(&self) {
        self.player.clear();
    }

    /// Stops the sink by emptying the queue.
    #[inline]
    pub fn stop(&self) {
        self.player.stop()
    }

    /// Destroys the sink without stopping the sounds that are still playing.
    #[inline]
    pub fn detach(self) {
        self.player.detach();
    }

    /// Sleeps the current thread until the sound ends.
    #[inline]
    pub fn sleep_until_end(&self) {
        self.player.sleep_until_end();
    }

    /// Returns true if this sink has no more sounds to play.
    #[inline]
    pub fn empty(&self) -> bool {
        self.player.empty()
    }

    /// Returns the number of sounds currently in the queue.
    #[allow(clippy::len_without_is_empty)]
    #[inline]
    pub fn len(&self) -> usize {
        self.player.len()
    }

    /// Attempts to seek to a given position in the current source.
    ///
    /// This blocks between 0 and ~5 milliseconds.
    ///
    /// As long as the duration of the source is known seek is guaranteed to saturate
    /// at the end of the source. For example given a source that reports a total duration
    /// of 42 seconds calling `try_seek()` with 60 seconds as argument will seek to
    /// 42 seconds.
    ///
    /// # Errors
    /// This function will return [`SeekError::NotSupported`] if one of the underlying
    /// sources does not support seeking.  
    ///
    /// It will return an error if an implementation ran
    /// into one during the seek.  
    ///
    /// When seeking beyond the end of a source this
    /// function might return an error if the duration of the source is not known.
    pub fn try_seek(&self, pos: Duration) -> Result<(), SeekError> {
        self.player.try_seek(pos)
    }

    /// Returns the position of the sound that's being played.
    ///
    /// This takes into account any speedup or delay applied.
    ///
    /// Example: if you apply a speedup of *2* to an mp3 decoder source and
    /// [`get_pos()`](Player::get_pos) returns *5s* then the position in the mp3
    /// recording is *10s* from its start.
    #[inline]
    pub fn get_pos(&self) -> Duration {
        self.player.get_pos()
    }
}
