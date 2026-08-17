use crate::rand;
use crate::server::ProducesTickets;
use crate::Error;

use ring::aead;
use std::mem;
use std::sync::{Arc, Mutex, MutexGuard};
use std::time;

/// The timebase for expiring and rolling tickets and ticketing
/// keys.  This is UNIX wall time in seconds.
///
/// This is guaranteed to be on or after the UNIX epoch.
#[derive(Clone, Copy, Debug)]
pub struct TimeBase(time::Duration);

impl TimeBase {
    #[inline]
    pub fn now() -> Result<Self, time::SystemTimeError> {
        Ok(Self(
            time::SystemTime::now().duration_since(time::UNIX_EPOCH)?,
        ))
    }

    #[inline]
    pub fn as_secs(&self) -> u64 {
        self.0.as_secs()
    }
}

/// This is a `ProducesTickets` implementation which uses
/// any *ring* `aead::Algorithm` to encrypt and authentication
/// the ticket payload.  It does not enforce any lifetime
/// constraint.
struct AeadTicketer {
    alg: &'static aead::Algorithm,
    key: aead::LessSafeKey,
    lifetime: u32,
}

impl AeadTicketer {
    /// Make a ticketer with recommended configuration and a random key.
    fn new() -> Result<Self, rand::GetRandomFailed> {
        let mut key = [0u8; 32];
        rand::fill_random(&mut key)?;

        let alg = &aead::CHACHA20_POLY1305;
        let key = aead::UnboundKey::new(alg, &key).unwrap();

        Ok(Self {
            alg,
            key: aead::LessSafeKey::new(key),
            lifetime: 60 * 60 * 12,
        })
    }
}

impl ProducesTickets for AeadTicketer {
    fn enabled(&self) -> bool {
        true
    }
    fn lifetime(&self) -> u32 {
        self.lifetime
    }

    /// Encrypt `message` and return the ciphertext.
    fn encrypt(&self, message: &[u8]) -> Option<Vec<u8>> {
        // Random nonce, because a counter is a privacy leak.
        let mut nonce_buf = [0u8; 12];
        rand::fill_random(&mut nonce_buf).ok()?;
        let nonce = aead::Nonce::assume_unique_for_key(nonce_buf);
        let aad = aead::Aad::empty();

        let mut ciphertext =
            Vec::with_capacity(nonce_buf.len() + message.len() + self.key.algorithm().tag_len());
        ciphertext.extend(nonce_buf);
        ciphertext.extend(message);
        self.key
            .seal_in_place_separate_tag(nonce, aad, &mut ciphertext[nonce_buf.len()..])
            .map(|tag| {
                ciphertext.extend(tag.as_ref());
                ciphertext
            })
            .ok()
    }

    /// Decrypt `ciphertext` and recover the original message.
    fn decrypt(&self, ciphertext: &[u8]) -> Option<Vec<u8>> {
        // Non-panicking `let (nonce, ciphertext) = ciphertext.split_at(...)`.
        let nonce = ciphertext.get(..self.alg.nonce_len())?;
        let ciphertext = ciphertext.get(nonce.len()..)?;

        // This won't fail since `nonce` has the required length.
        let nonce = aead::Nonce::try_assume_unique_for_key(nonce).ok()?;

        let mut out = Vec::from(ciphertext);

        let plain_len = self
            .key
            .open_in_place(nonce, aead::Aad::empty(), &mut out)
            .ok()?
            .len();
        out.truncate(plain_len);

        Some(out)
    }
}

struct TicketSwitcherState {
    next: Option<Box<dyn ProducesTickets>>,
    current: Box<dyn ProducesTickets>,
    previous: Option<Box<dyn ProducesTickets>>,
    next_switch_time: u64,
}

/// A ticketer that has a 'current' sub-ticketer and a single
/// 'previous' ticketer.  It creates a new ticketer every so
/// often, demoting the current ticketer.
struct TicketSwitcher {
    generator: fn() -> Result<Box<dyn ProducesTickets>, rand::GetRandomFailed>,
    lifetime: u32,
    state: Mutex<TicketSwitcherState>,
}

impl TicketSwitcher {
    /// `lifetime` is in seconds, and is how long the current ticketer
    /// is used to generate new tickets.  Tickets are accepted for no
    /// longer than twice this duration.  `generator` produces a new
    /// `ProducesTickets` implementation.
    fn new(
        lifetime: u32,
        generator: fn() -> Result<Box<dyn ProducesTickets>, rand::GetRandomFailed>,
    ) -> Result<Self, Error> {
        let now = TimeBase::now()?;
        Ok(Self {
            generator,
            lifetime,
            state: Mutex::new(TicketSwitcherState {
                next: Some(generator()?),
                current: generator()?,
                previous: None,
                next_switch_time: now
                    .as_secs()
                    .saturating_add(u64::from(lifetime)),
            }),
        })
    }

    /// If it's time, demote the `current` ticketer to `previous` (so it
    /// does no new encryptions but can do decryption) and use next for a
    /// new `current` ticketer.
    ///
    /// Calling this regularly will ensure timely key erasure.  Otherwise,
    /// key erasure will be delayed until the next encrypt/decrypt call.
    ///
    /// For efficiency, this is also responsible for locking the state mutex
    /// and returning the mutexguard.
    fn maybe_roll(&self, now: TimeBase) -> Option<MutexGuard<TicketSwitcherState>> {
        // The code below aims to make switching as efficient as possible
        // in the common case that the generator never fails. To achieve this
        // we run the following steps:
        //  1. If no switch is necessary, just return the mutexguard
        //  2. Shift over all of the ticketers (so current becomes previous,
        //     and next becomes current). After this, other threads can
        //     start using the new current ticketer.
        //  3. unlock mutex and generate new ticketer.
        //  4. Place new ticketer in next and return current
        //
        // There are a few things to note here. First, we don't check whether
        // a new switch might be needed in step 4, even though, due to locking
        // and entropy collection, significant amounts of time may have passed.
        // This is to guarantee that the thread doing the switch will eventually
        // make progress.
        //
        // Second, because next may be None, step 2 can fail. In that case
        // we enter a recovery mode where we generate 2 new ticketers, one for
        // next and one for the current ticketer. We then take the mutex a
        // second time and redo the time check to see if a switch is still
        // necessary.
        //
        // This somewhat convoluted approach ensures good availability of the
        // mutex, by ensuring that the state is usable and the mutex not held
        // during generation. It also ensures that, so long as the inner
        // ticketer never generates panics during encryption/decryption,
        // we are guaranteed to never panic when holding the mutex.

        let now = now.as_secs();
        let mut are_recovering = false; // Are we recovering from previous failure?
        {
            // Scope the mutex so we only take it for as long as needed
            let mut state = self.state.lock().ok()?;

            // Fast path in case we do not need to switch to the next ticketer yet
            if now <= state.next_switch_time {
                return Some(state);
            }

            // Make the switch, or mark for recovery if not possible
            if let Some(next) = state.next.take() {
                state.previous = Some(mem::replace(&mut state.current, next));
                state.next_switch_time = now.saturating_add(u64::from(self.lifetime));
            } else {
                are_recovering = true;
            }
        }

        // We always need a next, so generate it now
        let next = (self.generator)().ok()?;
        if !are_recovering {
            // Normal path, generate new next and place it in the state
            let mut state = self.state.lock().ok()?;
            state.next = Some(next);
            Some(state)
        } else {
            // Recovering, generate also a new current ticketer, and modify state
            // as needed. (we need to redo the time check, otherwise this might
            // result in very rapid switching of ticketers)
            let new_current = (self.generator)().ok()?;
            let mut state = self.state.lock().ok()?;
            state.next = Some(next);
            if now > state.next_switch_time {
                state.previous = Some(mem::replace(&mut state.current, new_current));
                state.next_switch_time = now.saturating_add(u64::from(self.lifetime));
            }
            Some(state)
        }
    }
}

impl ProducesTickets for TicketSwitcher {
    fn lifetime(&self) -> u32 {
        self.lifetime * 2
    }

    fn enabled(&self) -> bool {
        true
    }

    fn encrypt(&self, message: &[u8]) -> Option<Vec<u8>> {
        let state = self.maybe_roll(TimeBase::now().ok()?)?;

        state.current.encrypt(message)
    }

    fn decrypt(&self, ciphertext: &[u8]) -> Option<Vec<u8>> {
        let state = self.maybe_roll(TimeBase::now().ok()?)?;

        // Decrypt with the current key; if that fails, try with the previous.
        state
            .current
            .decrypt(ciphertext)
            .or_else(|| {
                state
                    .previous
                    .as_ref()
                    .and_then(|previous| previous.decrypt(ciphertext))
            })
    }
}

/// A concrete, safe ticket creation mechanism.
pub struct Ticketer {}

fn generate_inner() -> Result<Box<dyn ProducesTickets>, rand::GetRandomFailed> {
    Ok(Box::new(AeadTicketer::new()?))
}

impl Ticketer {
    /// Make the recommended Ticketer.  This produces tickets
    /// with a 12 hour life and randomly generated keys.
    ///
    /// The encryption mechanism used in Chacha20Poly1305.
    pub fn new() -> Result<Arc<dyn ProducesTickets>, Error> {
        Ok(Arc::new(TicketSwitcher::new(6 * 60 * 60, generate_inner)?))
    }
}

#[test]
fn basic_pairwise_test() {
    let t = Ticketer::new().unwrap();
    assert!(t.enabled());
    let cipher = t.encrypt(b"hello world").unwrap();
    let plain = t.decrypt(&cipher).unwrap();
    assert_eq!(plain, b"hello world");
}

#[test]
fn ticketswitcher_switching_test() {
    let t = Arc::new(TicketSwitcher::new(1, generate_inner).unwrap());
    let now = TimeBase::now().unwrap();
    let cipher1 = t.encrypt(b"ticket 1").unwrap();
    assert_eq!(t.decrypt(&cipher1).unwrap(), b"ticket 1");
    {
        // Trigger new ticketer
        t.maybe_roll(TimeBase(now.0 + time::Duration::from_secs(10)));
    }
    let cipher2 = t.encrypt(b"ticket 2").unwrap();
    assert_eq!(t.decrypt(&cipher1).unwrap(), b"ticket 1");
    assert_eq!(t.decrypt(&cipher2).unwrap(), b"ticket 2");
    {
        // Trigger new ticketer
        t.maybe_roll(TimeBase(now.0 + time::Duration::from_secs(20)));
    }
    let cipher3 = t.encrypt(b"ticket 3").unwrap();
    assert!(t.decrypt(&cipher1).is_none());
    assert_eq!(t.decrypt(&cipher2).unwrap(), b"ticket 2");
    assert_eq!(t.decrypt(&cipher3).unwrap(), b"ticket 3");
}

#[cfg(test)]
fn fail_generator() -> Result<Box<dyn ProducesTickets>, rand::GetRandomFailed> {
    Err(rand::GetRandomFailed)
}

#[test]
fn ticketswitcher_recover_test() {
    let mut t = TicketSwitcher::new(1, generate_inner).unwrap();
    let now = TimeBase::now().unwrap();
    let cipher1 = t.encrypt(b"ticket 1").unwrap();
    assert_eq!(t.decrypt(&cipher1).unwrap(), b"ticket 1");
    t.generator = fail_generator;
    {
        // Failed new ticketer
        t.maybe_roll(TimeBase(now.0 + time::Duration::from_secs(10)));
    }
    t.generator = generate_inner;
    let cipher2 = t.encrypt(b"ticket 2").unwrap();
    assert_eq!(t.decrypt(&cipher1).unwrap(), b"ticket 1");
    assert_eq!(t.decrypt(&cipher2).unwrap(), b"ticket 2");
    {
        // recover
        t.maybe_roll(TimeBase(now.0 + time::Duration::from_secs(20)));
    }
    let cipher3 = t.encrypt(b"ticket 3").unwrap();
    assert!(t.decrypt(&cipher1).is_none());
    assert_eq!(t.decrypt(&cipher2).unwrap(), b"ticket 2");
    assert_eq!(t.decrypt(&cipher3).unwrap(), b"ticket 3");
}
