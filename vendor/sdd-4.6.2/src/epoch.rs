/// [`Epoch`] is a unit of time that dictates the lifetime of retired memory regions.
///
/// The global epoch rotates `64` [`Epoch`] values in a range of `[0..63]`, instead of monotonically
/// increasing to reduce the memory footprint of the [`Epoch`] values.
#[derive(Clone, Copy, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
pub struct Epoch {
    value: u8,
}

impl Epoch {
    /// Rotates `64` epoch values.
    pub(super) const NUM_EPOCHS: u8 = 64;

    /// Returns a future [`Epoch`] when the current readers will not be present.
    ///
    /// The current [`Epoch`] may lag behind the global epoch value by `1`, therefore this method
    /// returns an [`Epoch`] three epochs next to `self`.
    ///
    /// # Examples
    ///
    /// ```
    /// use sdd::Epoch;
    ///
    /// let initial = Epoch::default();
    ///
    /// let next_generation = initial.next_generation();
    /// assert_eq!(next_generation, initial.next().next().next());
    /// ```
    #[inline]
    #[must_use]
    pub const fn next_generation(self) -> Epoch {
        self.next().next().next()
    }

    /// Checks if the current [`Epoch`] is in the same generation as the given [`Epoch`].
    ///
    /// This operation is not commutative, e.g., `a.in_same_generation(b)` is not the same as
    /// `b.in_same_generation(a)`. This returns `true` if the other [`Epoch`] is either the same
    /// with the current one, the next one, or the next one after that. The meaning of `false`
    /// returned by this method is that a memory region retired in the current [`Epoch`] will no
    /// longer be reachable in the other [`Epoch`].
    ///
    /// # Examples
    ///
    /// ```
    /// use sdd::Epoch;
    ///
    /// let initial = Epoch::default();
    ///
    /// let next_generation = initial.next_generation();
    /// assert!(initial.in_same_generation(initial.next().next()));
    /// assert!(!initial.in_same_generation(initial.next().next().next()));
    /// ```
    #[inline]
    #[must_use]
    pub const fn in_same_generation(self, other: Epoch) -> bool {
        other.value == self.value
            || other.value == self.next().value
            || other.value == self.next().next().value
    }

    /// Returns the next [`Epoch`] value.
    ///
    /// # Examples
    ///
    /// ```
    /// use sdd::Epoch;
    ///
    /// let initial = Epoch::default();
    ///
    /// let next = initial.next();
    /// assert!(initial < next);
    ///
    /// let next_prev = next.prev();
    /// assert_eq!(initial, next_prev);
    /// ```
    #[inline]
    #[must_use]
    pub const fn next(self) -> Epoch {
        Epoch {
            value: (self.value + 1) % Self::NUM_EPOCHS,
        }
    }

    /// Returns the previous [`Epoch`] value.
    ///
    /// # Examples
    ///
    /// ```
    /// use sdd::Epoch;
    ///
    /// let initial = Epoch::default();
    ///
    /// let prev = initial.prev();
    /// assert!(initial < prev);
    ///
    /// let prev_next = prev.next();
    /// assert_eq!(initial, prev_next);
    /// ```
    #[inline]
    #[must_use]
    pub const fn prev(self) -> Epoch {
        Epoch {
            value: (self.value + Self::NUM_EPOCHS - 1) % Self::NUM_EPOCHS,
        }
    }

    /// Construct an [`Epoch`] from a [`u8`] value.
    #[inline]
    pub(super) const fn from_u8(value: u8) -> Epoch {
        Epoch { value }
    }
}

impl TryFrom<u8> for Epoch {
    type Error = Epoch;

    #[inline]
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value < Self::NUM_EPOCHS {
            Ok(Epoch { value })
        } else {
            Err(Epoch {
                value: value % Self::NUM_EPOCHS,
            })
        }
    }
}

impl From<Epoch> for u8 {
    #[inline]
    fn from(epoch: Epoch) -> Self {
        epoch.value
    }
}
