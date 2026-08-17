use {IntoBuf, Buf, BufMut};
use buf::Iter;
use debug;

use std::{cmp, fmt, mem, hash, ops, slice, ptr, usize};
use std::borrow::{Borrow, BorrowMut};
use std::io::Cursor;
use std::sync::atomic::{self, AtomicUsize, AtomicPtr};
use std::sync::atomic::Ordering::{Relaxed, Acquire, Release, AcqRel};
use std::iter::{FromIterator, Iterator};

/// A reference counted contiguous slice of memory.
///
/// `Bytes` is an efficient container for storing and operating on contiguous
/// slices of memory. It is intended for use primarily in networking code, but
/// could have applications elsewhere as well.
///
/// `Bytes` values facilitate zero-copy network programming by allowing multiple
/// `Bytes` objects to point to the same underlying memory. This is managed by
/// using a reference count to track when the memory is no longer needed and can
/// be freed.
///
/// ```
/// use bytes::Bytes;
///
/// let mut mem = Bytes::from(&b"Hello world"[..]);
/// let a = mem.slice(0, 5);
///
/// assert_eq!(&a[..], b"Hello");
///
/// let b = mem.split_to(6);
///
/// assert_eq!(&mem[..], b"world");
/// assert_eq!(&b[..], b"Hello ");
/// ```
///
/// # Memory layout
///
/// The `Bytes` struct itself is fairly small, limited to a pointer to the
/// memory and 4 `usize` fields used to track information about which segment of
/// the underlying memory the `Bytes` handle has access to.
///
/// The memory layout looks like this:
///
/// ```text
/// +-------+
/// | Bytes |
/// +-------+
///  /      \_____
/// |              \
/// v               v
/// +-----+------------------------------------+
/// | Arc |         |      Data     |          |
/// +-----+------------------------------------+
/// ```
///
/// `Bytes` keeps both a pointer to the shared `Arc` containing the full memory
/// slice and a pointer to the start of the region visible by the handle.
/// `Bytes` also tracks the length of its view into the memory.
///
/// # Sharing
///
/// The memory itself is reference counted, and multiple `Bytes` objects may
/// point to the same region. Each `Bytes` handle point to different sections within
/// the memory region, and `Bytes` handle may or may not have overlapping views
/// into the memory.
///
///
/// ```text
///
///    Arc ptrs                   +---------+
///    ________________________ / | Bytes 2 |
///   /                           +---------+
///  /          +-----------+     |         |
/// |_________/ |  Bytes 1  |     |         |
/// |           +-----------+     |         |
/// |           |           | ___/ data     | tail
/// |      data |      tail |/              |
/// v           v           v               v
/// +-----+---------------------------------+-----+
/// | Arc |     |           |               |     |
/// +-----+---------------------------------+-----+
/// ```
///
/// # Mutating
///
/// While `Bytes` handles may potentially represent overlapping views of the
/// underlying memory slice and may not be mutated, `BytesMut` handles are
/// guaranteed to be the only handle able to view that slice of memory. As such,
/// `BytesMut` handles are able to mutate the underlying memory. Note that
/// holding a unique view to a region of memory does not mean that there are no
/// other `Bytes` and `BytesMut` handles with disjoint views of the underlying
/// memory.
///
/// # Inline bytes
///
/// As an optimization, when the slice referenced by a `Bytes` or `BytesMut`
/// handle is small enough [^1], `with_capacity` will avoid the allocation by
/// inlining the slice directly in the handle. In this case, a clone is no
/// longer "shallow" and the data will be copied.  Converting from a `Vec` will
/// never use inlining.
///
/// [^1]: Small enough: 31 bytes on 64 bit systems, 15 on 32 bit systems.
///
pub struct Bytes {
    inner: Inner,
}

/// A unique reference to a contiguous slice of memory.
///
/// `BytesMut` represents a unique view into a potentially shared memory region.
/// Given the uniqueness guarantee, owners of `BytesMut` handles are able to
/// mutate the memory. It is similar to a `Vec<u8>` but with less copies and
/// allocations.
///
/// For more detail, see [Bytes](struct.Bytes.html).
///
/// # Growth
///
/// One key difference from `Vec<u8>` is that most operations **do not
/// implicitly grow the buffer**. This means that calling `my_bytes.put("hello
/// world");` could panic if `my_bytes` does not have enough capacity. Before
/// writing to the buffer, ensure that there is enough remaining capacity by
/// calling `my_bytes.remaining_mut()`. In general, avoiding calls to `reserve`
/// is preferable.
///
/// The only exception is `extend` which implicitly reserves required capacity.
///
/// # Examples
///
/// ```
/// use bytes::{BytesMut, BufMut};
///
/// let mut buf = BytesMut::with_capacity(64);
///
/// buf.put(b'h');
/// buf.put(b'e');
/// buf.put("llo");
///
/// assert_eq!(&buf[..], b"hello");
///
/// // Freeze the buffer so that it can be shared
/// let a = buf.freeze();
///
/// // This does not allocate, instead `b` points to the same memory.
/// let b = a.clone();
///
/// assert_eq!(&a[..], b"hello");
/// assert_eq!(&b[..], b"hello");
/// ```
pub struct BytesMut {
    inner: Inner,
}

// Both `Bytes` and `BytesMut` are backed by `Inner` and functions are delegated
// to `Inner` functions. The `Bytes` and `BytesMut` shims ensure that functions
// that mutate the underlying buffer are only performed when the data range
// being mutated is only available via a single `BytesMut` handle.
//
// # Data storage modes
//
// The goal of `bytes` is to be as efficient as possible across a wide range of
// potential usage patterns. As such, `bytes` needs to be able to handle buffers
// that are never shared, shared on a single thread, and shared across many
// threads. `bytes` also needs to handle both tiny buffers as well as very large
// buffers. For example, [Cassandra](http://cassandra.apache.org) values have
// been known to be in the hundreds of megabyte, and HTTP header values can be a
// few characters in size.
//
// To achieve high performance in these various situations, `Bytes` and
// `BytesMut` use different strategies for storing the buffer depending on the
// usage pattern.
//
// ## Delayed `Arc` allocation
//
// When a `Bytes` or `BytesMut` is first created, there is only one outstanding
// handle referencing the buffer. Since sharing is not yet required, an `Arc`* is
// not used and the buffer is backed by a `Vec<u8>` directly. Using an
// `Arc<Vec<u8>>` requires two allocations, so if the buffer ends up never being
// shared, that allocation is avoided.
//
// When sharing does become necessary (`clone`, `split_to`, `split_off`), that
// is when the buffer is promoted to being shareable. The `Vec<u8>` is moved
// into an `Arc` and both the original handle and the new handle use the same
// buffer via the `Arc`.
//
// * `Arc` is being used to signify an atomically reference counted cell. We
// don't use the `Arc` implementation provided by `std` and instead use our own.
// This ends up simplifying a number of the `unsafe` code snippets.
//
// ## Inlining small buffers
//
// The `Bytes` / `BytesMut` structs require 4 pointer sized fields. On 64 bit
// systems, this ends up being 32 bytes, which is actually a lot of storage for
// cases where `Bytes` is being used to represent small byte strings, such as
// HTTP header names and values.
//
// To avoid any allocation at all in these cases, `Bytes` will use the struct
// itself for storing the buffer, reserving 1 byte for meta data. This means
// that, on 64 bit systems, 31 byte buffers require no allocation at all.
//
// The byte used for metadata stores a 2 bits flag used to indicate that the
// buffer is stored inline as well as 6 bits for tracking the buffer length (the
// return value of `Bytes::len`).
//
// ## Static buffers
//
// `Bytes` can also represent a static buffer, which is created with
// `Bytes::from_static`. No copying or allocations are required for tracking
// static buffers. The pointer to the `&'static [u8]`, the length, and a flag
// tracking that the `Bytes` instance represents a static buffer is stored in
// the `Bytes` struct.
//
// # Struct layout
//
// Both `Bytes` and `BytesMut` are wrappers around `Inner`, which provides the
// data fields as well as all of the function implementations.
//
// The `Inner` struct is carefully laid out in order to support the
// functionality described above as well as being as small as possible. Size is
// important as growing the size of the `Bytes` struct from 32 bytes to 40 bytes
// added as much as 15% overhead in benchmarks using `Bytes` in an HTTP header
// map structure.
//
// The `Inner` struct contains the following fields:
//
// * `ptr: *mut u8`
// * `len: usize`
// * `cap: usize`
// * `arc: AtomicPtr<Shared>`
//
// ## `ptr: *mut u8`
//
// A pointer to start of the handle's buffer view. When backed by a `Vec<u8>`,
// this is always the `Vec`'s pointer. When backed by an `Arc<Vec<u8>>`, `ptr`
// may have been shifted to point somewhere inside the buffer.
//
// When in "inlined" mode, `ptr` is used as part of the inlined buffer.
//
// ## `len: usize`
//
// The length of the handle's buffer view. When backed by a `Vec<u8>`, this is
// always the `Vec`'s length. The slice represented by `ptr` and `len` should
// (ideally) always be initialized memory.
//
// When in "inlined" mode, `len` is used as part of the inlined buffer.
//
// ## `cap: usize`
//
// The capacity of the handle's buffer view. When backed by a `Vec<u8>`, this is
// always the `Vec`'s capacity. The slice represented by `ptr+len` and `cap-len`
// may or may not be initialized memory.
//
// When in "inlined" mode, `cap` is used as part of the inlined buffer.
//
// ## `arc: AtomicPtr<Shared>`
//
// When `Inner` is in allocated mode (backed by Vec<u8> or Arc<Vec<u8>>), this
// will be the pointer to the `Arc` structure tracking the ref count for the
// underlying buffer. When the pointer is null, then the `Arc` has not been
// allocated yet and `self` is the only outstanding handle for the underlying
// buffer.
//
// The lower two bits of `arc` are used to track the storage mode of `Inner`.
// `0b01` indicates inline storage, `0b10` indicates static storage, and `0b11`
// indicates vector storage, not yet promoted to Arc.  Since pointers to
// allocated structures are aligned, the lower two bits of a pointer will always
// be 0. This allows disambiguating between a pointer and the two flags.
//
// When in "inlined" mode, the least significant byte of `arc` is also used to
// store the length of the buffer view (vs. the capacity, which is a constant).
//
// The rest of `arc`'s bytes are used as part of the inline buffer, which means
// that those bytes need to be located next to the `ptr`, `len`, and `cap`
// fields, which make up the rest of the inline buffer. This requires special
// casing the layout of `Inner` depending on if the target platform is big or
// little endian.
//
// On little endian platforms, the `arc` field must be the first field in the
// struct. On big endian platforms, the `arc` field must be the last field in
// the struct. Since a deterministic struct layout is required, `Inner` is
// annotated with `#[repr(C)]`.
//
// # Thread safety
//
// `Bytes::clone()` returns a new `Bytes` handle with no copying. This is done
// by bumping the buffer ref count and returning a new struct pointing to the
// same buffer. However, the `Arc` structure is lazily allocated. This means
// that if `Bytes` is stored itself in an `Arc` (`Arc<Bytes>`), the `clone`
// function can be called concurrently from multiple threads. This is why an
// `AtomicPtr` is used for the `arc` field vs. a `*const`.
//
// Care is taken to ensure that the need for synchronization is minimized. Most
// operations do not require any synchronization.
//
#[cfg(target_endian = "little")]
#[repr(C)]
struct Inner {
    // WARNING: Do not access the fields directly unless you know what you are
    // doing. Instead, use the fns. See implementation comment above.
    arc: AtomicPtr<Shared>,
    ptr: *mut u8,
    len: usize,
    cap: usize,
}

#[cfg(target_endian = "big")]
#[repr(C)]
struct Inner {
    // WARNING: Do not access the fields directly unless you know what you are
    // doing. Instead, use the fns. See implementation comment above.
    ptr: *mut u8,
    len: usize,
    cap: usize,
    arc: AtomicPtr<Shared>,
}

// Thread-safe reference-counted container for the shared storage. This mostly
// the same as `std::sync::Arc` but without the weak counter. The ref counting
// fns are based on the ones found in `std`.
//
// The main reason to use `Shared` instead of `std::sync::Arc` is that it ends
// up making the overall code simpler and easier to reason about. This is due to
// some of the logic around setting `Inner::arc` and other ways the `arc` field
// is used. Using `Arc` ended up requiring a number of funky transmutes and
// other shenanigans to make it work.
struct Shared {
    vec: Vec<u8>,
    original_capacity_repr: usize,
    ref_count: AtomicUsize,
}

// Buffer storage strategy flags.
const KIND_ARC: usize = 0b00;
const KIND_INLINE: usize = 0b01;
const KIND_STATIC: usize = 0b10;
const KIND_VEC: usize = 0b11;
const KIND_MASK: usize = 0b11;

// The max original capacity value. Any `Bytes` allocated with a greater initial
// capacity will default to this.
const MAX_ORIGINAL_CAPACITY_WIDTH: usize = 17;
// The original capacity algorithm will not take effect unless the originally
// allocated capacity was at least 1kb in size.
const MIN_ORIGINAL_CAPACITY_WIDTH: usize = 10;
// The original capacity is stored in powers of 2 starting at 1kb to a max of
// 64kb. Representing it as such requires only 3 bits of storage.
const ORIGINAL_CAPACITY_MASK: usize = 0b11100;
const ORIGINAL_CAPACITY_OFFSET: usize = 2;

// When the storage is in the `Vec` representation, the pointer can be advanced
// at most this value. This is due to the amount of storage available to track
// the offset is usize - number of KIND bits and number of ORIGINAL_CAPACITY
// bits.
const VEC_POS_OFFSET: usize = 5;
const MAX_VEC_POS: usize = usize::MAX >> VEC_POS_OFFSET;
const NOT_VEC_POS_MASK: usize = 0b11111;

// Bit op constants for extracting the inline length value from the `arc` field.
const INLINE_LEN_MASK: usize = 0b11111100;
const INLINE_LEN_OFFSET: usize = 2;

// Byte offset from the start of `Inner` to where the inline buffer data
// starts. On little endian platforms, the first byte of the struct is the
// storage flag, so the data is shifted by a byte. On big endian systems, the
// data starts at the beginning of the struct.
#[cfg(target_endian = "little")]
const INLINE_DATA_OFFSET: isize = 1;
#[cfg(target_endian = "big")]
const INLINE_DATA_OFFSET: isize = 0;

#[cfg(target_pointer_width = "64")]
const PTR_WIDTH: usize = 64;
#[cfg(target_pointer_width = "32")]
const PTR_WIDTH: usize = 32;

// Inline buffer capacity. This is the size of `Inner` minus 1 byte for the
// metadata.
#[cfg(target_pointer_width = "64")]
const INLINE_CAP: usize = 4 * 8 - 1;
#[cfg(target_pointer_width = "32")]
const INLINE_CAP: usize = 4 * 4 - 1;

/*
 *
 * ===== Bytes =====
 *
 */

impl Bytes {
    /// Creates a new `Bytes` with the specified capacity.
    ///
    /// The returned `Bytes` will be able to hold at least `capacity` bytes
    /// without reallocating. If `capacity` is under `4 * size_of::<usize>() - 1`,
    /// then `BytesMut` will not allocate.
    ///
    /// It is important to note that this function does not specify the length
    /// of the returned `Bytes`, but only the capacity.
    ///
    /// # Examples
    ///
    /// ```
    /// use bytes::Bytes;
    ///
    /// let mut bytes = Bytes::with_capacity(64);
    ///
    /// // `bytes` contains no data, even though there is capacity
    /// assert_eq!(bytes.len(), 0);
    ///
    /// bytes.extend_from_slice(&b"hello world"[..]);
    ///
    /// assert_eq!(&bytes[..], b"hello world");
    /// ```
    #[inline]
    pub fn with_capacity(capacity: usize) -> Bytes {
        Bytes {
            inner: Inner::with_capacity(capacity),
        }
    }

    /// Creates a new empty `Bytes`.
    ///
    /// This will not allocate and the returned `Bytes` handle will be empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use bytes::Bytes;
    ///
    /// let b = Bytes::new();
    /// assert_eq!(&b[..], b"");
    /// ```
    #[inline]
    pub fn new() -> Bytes {
        Bytes::with_capacity(0)
    }

    /// Creates a new `Bytes` from a static slice.
    ///
    /// The returned `Bytes` will point directly to the static slice. There is
    /// no allocating or copying.
    ///
    /// # Examples
    ///
    /// ```
    /// use bytes::Bytes;
    ///
    /// let b = Bytes::from_static(b"hello");
    /// assert_eq!(&b[..], b"hello");
    /// ```
    #[inline]
    pub fn from_static(bytes: &'static [u8]) -> Bytes {
        Bytes {
            inner: Inner::from_static(bytes),
        }
    }

    /// Returns the number of bytes contained in this `Bytes`.
    ///
    /// # Examples
    ///
    /// ```
    /// use bytes::Bytes;
    ///
    /// let b = Bytes::from(&b"hello"[..]);
    /// assert_eq!(b.len(), 5);
    /// ```
    #[inline]
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    /// Returns true if the `Bytes` has a length of 0.
    ///
    /// # Examples
    ///
    /// ```
    /// use bytes::Bytes;
    ///
    /// let b = Bytes::new();
    /// assert!(b.is_empty());
    /// ```
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    /// Returns a slice of self for the index range `[begin..end)`.
    ///
    /// This will increment the reference count for the underlying memory and
    /// return a new `Bytes` handle set to the slice.
    ///
    /// This operation is `O(1)`.
    ///
    /// # Examples
    ///
    /// ```
    /// use bytes::Bytes;
    ///
    /// let a = Bytes::from(&b"hello world"[..]);
    /// let b = a.slice(2, 5);
    ///
    /// assert_eq!(&b[..], b"llo");
    /// ```
    ///
    /// # Panics
    ///
    /// Requires that `begin <= end` and `end <= self.len()`, otherwise slicing
    /// will panic.
    pub fn slice(&self, begin: usize, end: usize) -> Bytes {
        assert!(begin <= end);
        assert!(end <= self.len());

        if end - begin <= INLINE_CAP {
            return Bytes::from(&self[begin..end]);
        }

        let mut ret = self.clone();

        unsafe {
            ret.inner.set_end(end);
            ret.inner.set_start(begin);
        }

        ret
    }

    /// Returns a slice of self for the index range `[begin..self.len())`.
    ///
    /// This will increment the reference count for the underlying memory and
    /// return a new `Bytes` handle set to the slice.
    ///
    /// This operation is `O(1)` and is equivalent to `self.slice(begin,
    /// self.len())`.
    ///
    /// # Examples
    ///
    /// ```
    /// use bytes::Bytes;
    ///
    /// let a = Bytes::from(&b"hello world"[..]);
    /// let b = a.slice_from(6);
    ///
    /// assert_eq!(&b[..], b"world");
    /// ```
    ///
    /// # Panics
    ///
    /// Requires that `begin <= self.len()`, otherwise slicing will panic.
    pub fn slice_from(&self, begin: usize) -> Bytes {
        self.slice(begin, self.len())
    }

    /// Returns a slice of self for the index range `[0..end)`.
    ///
    /// This will increment the reference count for the underlying memory and
    /// return a new `Bytes` handle set to the slice.
    ///
    /// This operation is `O(1)` and is equivalent to `self.slice(0, end)`.
    ///
    /// # Examples
    ///
    /// ```
    /// use bytes::Bytes;
    ///
    /// let a = Bytes::from(&b"hello world"[..]);
    /// let b = a.slice_to(5);
    ///
    /// assert_eq!(&b[..], b"hello");
    /// ```
    ///
    /// # Panics
    ///
    /// Requires that `end <= self.len()`, otherwise slicing will panic.
    pub fn slice_to(&self, end: usize) -> Bytes {
        self.slice(0, end)
    }

    /// Returns a slice of self that is equivalent to the given `subset`.
    ///
    /// When processing a `Bytes` buffer with other tools, one often gets a
    /// `&[u8]` which is in fact a slice of the `Bytes`, i.e. a subset of it.
    /// This function turns that `&[u8]` into another `Bytes`, as if one had
    /// called `self.slice()` with the offsets that correspond to `subset`.
    ///
    /// This operation is `O(1)`.
    ///
    /// # Examples
    ///
    /// ```
    /// use bytes::Bytes;
    ///
    /// let bytes = Bytes::from(&b"012345678"[..]);
    /// let as_slice = bytes.as_ref();
    /// let subset = &as_slice[2..6];
    /// let subslice = bytes.slice_ref(&subset);
    /// assert_eq!(&subslice[..], b"2345");
    /// ```
    ///
    /// # Panics
    ///
    /// Requires that the given `sub` slice is in fact contained within the
    /// `Bytes` buffer; otherwise this function will panic.
    pub fn slice_ref(&self, subset: &[u8]) -> Bytes {
        let bytes_p = self.as_ptr() as usize;
        let bytes_len = self.len();

        let sub_p = subset.as_ptr() as usize;
        let sub_len = subset.len();

        assert!(sub_p >= bytes_p);
        assert!(sub_p + sub_len <= bytes_p + bytes_len);

        let sub_offset = sub_p - bytes_p;

        self.slice(sub_offset, sub_offset + sub_len)
    }

    /// Splits the bytes into two at the given index.
    ///
    /// Afterwards `self` contains elements `[0, at)`, and the returned `Bytes`
    /// contains elements `[at, len)`.
    ///
    /// This is an `O(1)` operation that just increases the reference count and
    /// sets a few indices.
    ///
    /// # Examples
    ///
    /// ```
    /// use bytes::Bytes;
    ///
    /// let mut a = Bytes::from(&b"hello world"[..]);
    /// let b = a.split_off(5);
    ///
    /// assert_eq!(&a[..], b"hello");
    /// assert_eq!(&b[..], b" world");
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if `at > len`.
    pub fn split_off(&mut self, at: usize) -> Bytes {
        assert!(at <= self.len());

        if at == self.len() {
            return Bytes::new();
        }

        if at == 0 {
            return mem::replace(self, Bytes::new());
        }

        Bytes {
            inner: self.inner.split_off(at),
        }
    }

    /// Splits the bytes into two at the given index.
    ///
    /// Afterwards `self` contains elements `[at, len)`, and the returned
    /// `Bytes` contains elements `[0, at)`.
    ///
    /// This is an `O(1)` operation that just increases the reference count and
    /// sets a few indices.
    ///
    /// # Examples
    ///
    /// ```
    /// use bytes::Bytes;
    ///
    /// let mut a = Bytes::from(&b"hello world"[..]);
    /// let b = a.split_to(5);
    ///
    /// assert_eq!(&a[..], b" world");
    /// assert_eq!(&b[..], b"hello");
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if `at > len`.
    pub fn split_to(&mut self, at: usize) -> Bytes {
        assert!(at <= self.len());

        if at == self.len() {
            return mem::replace(self, Bytes::new());
        }

        if at == 0 {
            return Bytes::new();
        }

        Bytes {
            inner: self.inner.split_to(at),
        }
    }

    #[deprecated(since = "0.4.1", note = "use split_to instead")]
    #[doc(hidden)]
    pub fn drain_to(&mut self, at: usize) -> Bytes {
        self.split_to(at)
    }

    /// Shortens the buffer, keeping the first `len` bytes and dropping the
    /// rest.
    ///
    /// If `len` is greater than the buffer's current length, this has no
    /// effect.
    ///
    /// The [`split_off`] method can emulate `truncate`, but this causes the
    /// excess bytes to be returned instead of dropped.
    ///
    /// # Examples
    ///
    /// ```
    /// use bytes::Bytes;
    ///
    /// let mut buf = Bytes::from(&b"hello world"[..]);
    /// buf.truncate(5);
    /// assert_eq!(buf, b"hello"[..]);
    /// ```
    ///
    /// [`split_off`]: #method.split_off
    pub fn truncate(&mut self, len: usize) {
        self.inner.truncate(len);
    }

    /// Shortens the buffer, dropping the first `cnt` bytes and keeping the
    /// rest.
    ///
    /// This is the same function as `Buf::advance`, and in the next breaking
    /// release of `bytes`, this implementation will be removed in favor of
    /// having `Bytes` implement `Buf`.
    ///
    /// # Panics
    ///
    /// This function panics if `cnt` is greater than `self.len()`
    #[inline]
    pub fn advance(&mut self, cnt: usize) {
        assert!(cnt <= self.len(), "cannot advance past `remaining`");
        unsafe { self.inner.set_start(cnt); }
    }

    /// Clears the buffer, removing all data.
    ///
    /// # Examples
    ///
    /// ```
    /// use bytes::Bytes;
    ///
    /// let mut buf = Bytes::from(&b"hello world"[..]);
    /// buf.clear();
    /// assert!(buf.is_empty());
    /// ```
    pub fn clear(&mut self) {
        self.truncate(0);
    }

    /// Attempts to convert into a `BytesMut` handle.
    ///
    /// This will only succeed if there are no other outstanding references to
    /// the underlying chunk of memory. `Bytes` handles that contain inlined
    /// bytes will always be convertable to `BytesMut`.
    ///
    /// # Examples
    ///
    /// ```
    /// use bytes::Bytes;
    ///
    /// let a = Bytes::from(&b"Mary had a little lamb, little lamb, little lamb..."[..]);
    ///
    /// // Create a shallow clone
    /// let b = a.clone();
    ///
    /// // This will fail because `b` shares a reference with `a`
    /// let a = a.try_mut().unwrap_err();
    ///
    /// drop(b);
    ///
    /// // This will succeed
    /// let mut a = a.try_mut().unwrap();
    ///
    /// a[0] = b'b';
    ///
    /// assert_eq!(&a[..4], b"bary");
    /// ```
    pub fn try_mut(mut self) -> Result<BytesMut, Bytes> {
        if self.inner.is_mut_safe() {
            Ok(BytesMut { inner: self.inner })
        } else {
            Err(self)
        }
    }

    /// Appends given bytes to this object.
    ///
    /// If this `Bytes` object has not enough capacity, it is resized first.
    /// If it is shared (`refcount > 1`), it is copied first.
    ///
    /// This operation can be less effective than the similar operation on
    /// `BytesMut`, especially on small additions.
    ///
    /// # Examples
    ///
    /// ```
    /// use bytes::Bytes;
    ///
    /// let mut buf = Bytes::from("aabb");
    /// buf.extend_from_slice(b"ccdd");
    /// buf.extend_from_slice(b"eeff");
    ///
    /// assert_eq!(b"aabbccddeeff", &buf[..]);
    /// ```
    pub fn extend_from_slice(&mut self, extend: &[u8]) {
        if extend.is_empty() {
            return;
        }

        let new_cap = self.len().checked_add(extend.len()).expect("capacity overflow");

        let result = match mem::replace(self, Bytes::new()).try_mut() {
            Ok(mut bytes_mut) => {
                bytes_mut.extend_from_slice(extend);
                bytes_mut
            },
            Err(bytes) => {
                let mut bytes_mut = BytesMut::with_capacity(new_cap);
                bytes_mut.put_slice(&bytes);
                bytes_mut.put_slice(extend);
                bytes_mut
            }
        };

        mem::replace(self, result.freeze());
    }
}

impl IntoBuf for Bytes {
    type Buf = Cursor<Self>;

    fn into_buf(self) -> Self::Buf {
        Cursor::new(self)
    }
}

impl<'a> IntoBuf for &'a Bytes {
    type Buf = Cursor<Self>;

    fn into_buf(self) -> Self::Buf {
        Cursor::new(self)
    }
}

impl Clone for Bytes {
    fn clone(&self) -> Bytes {
        Bytes {
            inner: unsafe { self.inner.shallow_clone(false) },
        }
    }
}

impl AsRef<[u8]> for Bytes {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        self.inner.as_ref()
    }
}

impl ops::Deref for Bytes {
    type Target = [u8];

    #[inline]
    fn deref(&self) -> &[u8] {
        self.inner.as_ref()
    }
}

impl From<BytesMut> for Bytes {
    fn from(src: BytesMut) -> Bytes {
        src.freeze()
    }
}

impl From<Vec<u8>> for Bytes {
    fn from(src: Vec<u8>) -> Bytes {
        BytesMut::from(src).freeze()
    }
}

impl From<String> for Bytes {
    fn from(src: String) -> Bytes {
        BytesMut::from(src).freeze()
    }
}

impl<'a> From<&'a [u8]> for Bytes {
    fn from(src: &'a [u8]) -> Bytes {
        BytesMut::from(src).freeze()
    }
}

impl<'a> From<&'a str> for Bytes {
    fn from(src: &'a str) -> Bytes {
        BytesMut::from(src).freeze()
    }
}

impl FromIterator<u8> for BytesMut {
    fn from_iter<T: IntoIterator<Item = u8>>(into_iter: T) -> Self {
        let iter = into_iter.into_iter();
        let (min, maybe_max) = iter.size_hint();

        let mut out = BytesMut::with_capacity(maybe_max.unwrap_or(min));

        for i in iter {
            out.reserve(1);
            out.put(i);
        }

        out
    }
}

impl FromIterator<u8> for Bytes {
    fn from_iter<T: IntoIterator<Item = u8>>(into_iter: T) -> Self {
        BytesMut::from_iter(into_iter).freeze()
    }
}

impl<'a> FromIterator<&'a u8> for BytesMut {
    fn from_iter<T: IntoIterator<Item = &'a u8>>(into_iter: T) -> Self {
        BytesMut::from_iter(into_iter.into_iter().map(|b| *b))
    }
}

impl<'a> FromIterator<&'a u8> for Bytes {
    fn from_iter<T: IntoIterator<Item = &'a u8>>(into_iter: T) -> Self {
        BytesMut::from_iter(into_iter).freeze()
    }
}

impl PartialEq for Bytes {
    fn eq(&self, other: &Bytes) -> bool {
        self.inner.as_ref() == other.inner.as_ref()
    }
}

impl PartialOrd for Bytes {
    fn partial_cmp(&self, other: &Bytes) -> Option<cmp::Ordering> {
        self.inner.as_ref().partial_cmp(other.inner.as_ref())
    }
}

impl Ord for Bytes {
    fn cmp(&self, other: &Bytes) -> cmp::Ordering {
        self.inner.as_ref().cmp(other.inner.as_ref())
    }
}

impl Eq for Bytes {
}

impl Default for Bytes {
    #[inline]
    fn default() -> Bytes {
        Bytes::new()
    }
}

impl fmt::Debug for Bytes {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&debug::BsDebug(&self.inner.as_ref()), fmt)
    }
}

impl hash::Hash for Bytes {
    fn hash<H>(&self, state: &mut H) where H: hash::Hasher {
        let s: &[u8] = self.as_ref();
        s.hash(state);
    }
}

impl Borrow<[u8]> for Bytes {
    fn borrow(&self) -> &[u8] {
        self.as_ref()
    }
}

impl IntoIterator for Bytes {
    type Item = u8;
    type IntoIter = Iter<Cursor<Bytes>>;

    fn into_iter(self) -> Self::IntoIter {
        self.into_buf().iter()
    }
}

impl<'a> IntoIterator for &'a Bytes {
    type Item = u8;
    type IntoIter = Iter<Cursor<&'a Bytes>>;

    fn into_iter(self) -> Self::IntoIter {
        self.into_buf().iter()
    }
}

impl Extend<u8> for Bytes {
    fn extend<T>(&mut self, iter: T) where T: IntoIterator<Item = u8> {
        let iter = iter.into_iter();

        let (lower, upper) = iter.size_hint();

        // Avoid possible conversion into mut if there's nothing to add
        if let Some(0) = upper {
            return;
        }

        let mut bytes_mut = match mem::replace(self, Bytes::new()).try_mut() {
            Ok(bytes_mut) => bytes_mut,
            Err(bytes) => {
                let mut bytes_mut = BytesMut::with_capacity(bytes.len() + lower);
                bytes_mut.put_slice(&bytes);
                bytes_mut
            }
        };

        bytes_mut.extend(iter);

        mem::replace(self, bytes_mut.freeze());
    }
}

impl<'a> Extend<&'a u8> for Bytes {
    fn extend<T>(&mut self, iter: T) where T: IntoIterator<Item = &'a u8> {
        self.extend(iter.into_iter().map(|b| *b))
    }
}

/*
 *
 * ===== BytesMut =====
 *
 */

impl BytesMut {
    /// Creates a new `BytesMut` with the specified capacity.
    ///
    /// The returned `BytesMut` will be able to hold at least `capacity` bytes
    /// without reallocating. If `capacity` is under `4 * size_of::<usize>() - 1`,
    /// then `BytesMut` will not allocate.
    ///
    /// It is important to note that this function does not specify the length
    /// of the returned `BytesMut`, but only the capacity.
    ///
    /// # Examples
    ///
    /// ```
    /// use bytes::{BytesMut, BufMut};
    ///
    /// let mut bytes = BytesMut::with_capacity(64);
    ///
    /// // `bytes` contains no data, even though there is capacity
    /// assert_eq!(bytes.len(), 0);
    ///
    /// bytes.put(&b"hello world"[..]);
    ///
    /// assert_eq!(&bytes[..], b"hello world");
    /// ```
    #[inline]
    pub fn with_capacity(capacity: usize) -> BytesMut {
        BytesMut {
            inner: Inner::with_capacity(capacity),
        }
    }

    /// Creates a new `BytesMut` with default capacity.
    ///
    /// Resulting object has length 0 and unspecified capacity.
    /// This function does not allocate.
    ///
    /// # Examples
    ///
    /// ```
    /// use bytes::{BytesMut, BufMut};
    ///
    /// let mut bytes = BytesMut::new();
    ///
    /// assert_eq!(0, bytes.len());
    ///
    /// bytes.reserve(2);
    /// bytes.put_slice(b"xy");
    ///
    /// assert_eq!(&b"xy"[..], &bytes[..]);
    /// ```
    #[inline]
    pub fn new() -> BytesMut {
        BytesMut::with_capacity(0)
    }

    /// Returns the number of bytes contained in this `BytesMut`.
    ///
    /// # Examples
    ///
    /// ```
    /// use bytes::BytesMut;
    ///
    /// let b = BytesMut::from(&b"hello"[..]);
    /// assert_eq!(b.len(), 5);
    /// ```
    #[inline]
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    /// Returns true if the `BytesMut` has a length of 0.
    ///
    /// # Examples
    ///
    /// ```
    /// use bytes::BytesMut;
    ///
    /// let b = BytesMut::with_capacity(64);
    /// assert!(b.is_empty());
    /// ```
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns the number of bytes the `BytesMut` can hold without reallocating.
    ///
    /// # Examples
    ///
    /// ```
    /// use bytes::BytesMut;
    ///
    /// let b = BytesMut::with_capacity(64);
    /// assert_eq!(b.capacity(), 64);
    /// ```
    #[inline]
    pub fn capacity(&self) -> usize {
        self.inner.capacity()
    }

    /// Converts `self` into an immutable `Bytes`.
    ///
    /// The conversion is zero cost and is used to indicate that the slice
    /// referenced by the handle will no longer be mutated. Once the conversion
    /// is done, the handle can be cloned and shared across threads.
    ///
    /// # Examples
    ///
    /// ```
    /// use bytes::{BytesMut, BufMut};
    /// use std::thread;
    ///
    /// let mut b = BytesMut::with_capacity(64);
    /// b.put("hello world");
    /// let b1 = b.freeze();
    /// let b2 = b1.clone();
    ///
    /// let th = thread::spawn(move || {
    ///     assert_eq!(&b1[..], b"hello world");
    /// });
    ///
    /// assert_eq!(&b2[..], b"hello world");
    /// th.join().unwrap();
    /// ```
    #[inline]
    pub fn freeze(self) -> Bytes {
        Bytes { inner: self.inner }
    }

    /// Splits the bytes into two at the given index.
    ///
    /// Afterwards `self` contains elements `[0, at)`, and the returned
    /// `BytesMut` contains elements `[at, capacity)`.
    ///
    /// This is an `O(1)` operation that just increases the reference count
    /// and sets a few indices.
    ///
    /// # Examples
    ///
    /// ```
    /// use bytes::BytesMut;
    ///
    /// let mut a = BytesMut::from(&b"hello world"[..]);
    /// let mut b = a.split_off(5);
    ///
    /// a[0] = b'j';
    /// b[0] = b'!';
    ///
    /// assert_eq!(&a[..], b"jello");
    /// assert_eq!(&b[..], b"!world");
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if `at > capacity`.
    pub fn split_off(&mut self, at: usize) -> BytesMut {
        BytesMut {
            inner: self.inner.split_off(at),
        }
    }

    /// Removes the bytes from the current view, returning them in a new
    /// `BytesMut` handle.
    ///
    /// Afterwards, `self` will be empty, but will retain any additional
    /// capacity that it had before the operation. This is identical to
    /// `self.split_to(self.len())`.
    ///
    /// This is an `O(1)` operation that just increases the reference count and
    /// sets a few indices.
    ///
    /// # Examples
    ///
    /// ```
    /// use bytes::{BytesMut, BufMut};
    ///
    /// let mut buf = BytesMut::with_capacity(1024);
    /// buf.put(&b"hello world"[..]);
    ///
    /// let other = buf.take();
    ///
    /// assert!(buf.is_empty());
    /// assert_eq!(1013, buf.capacity());
    ///
    /// assert_eq!(other, b"hello world"[..]);
    /// ```
    pub fn take(&mut self) -> BytesMut {
        let len = self.len();
        self.split_to(len)
    }

    #[deprecated(since = "0.4.1", note = "use take instead")]
    #[doc(hidden)]
    pub fn drain(&mut self) -> BytesMut {
        self.take()
    }

    /// Splits the buffer into two at the given index.
    ///
    /// Afterwards `self` contains elements `[at, len)`, and the returned `BytesMut`
    /// contains elements `[0, at)`.
    ///
    /// This is an `O(1)` operation that just increases the reference count and
    /// sets a few indices.
    ///
    /// # Examples
    ///
    /// ```
    /// use bytes::BytesMut;
    ///
    /// let mut a = BytesMut::from(&b"hello world"[..]);
    /// let mut b = a.split_to(5);
    ///
    /// a[0] = b'!';
    /// b[0] = b'j';
    ///
    /// assert_eq!(&a[..], b"!world");
    /// assert_eq!(&b[..], b"jello");
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if `at > len`.
    pub fn split_to(&mut self, at: usize) -> BytesMut {
        BytesMut {
            inner: self.inner.split_to(at),
        }
    }

    #[deprecated(since = "0.4.1", note = "use split_to instead")]
    #[doc(hidden)]
    pub fn drain_to(&mut self, at: usize) -> BytesMut {
        self.split_to(at)
    }

    /// Shortens the buffer, keeping the first `len` bytes and dropping the
    /// rest.
    ///
    /// If `len` is greater than the buffer's current length, this has no
    /// effect.
    ///
    /// The [`split_off`] method can emulate `truncate`, but this causes the
    /// excess bytes to be returned instead of dropped.
    ///
    /// # Examples
    ///
    /// ```
    /// use bytes::BytesMut;
    ///
    /// let mut buf = BytesMut::from(&b"hello world"[..]);
    /// buf.truncate(5);
    /// assert_eq!(buf, b"hello"[..]);
    /// ```
    ///
    /// [`split_off`]: #method.split_off
    pub fn truncate(&mut self, len: usize) {
        self.inner.truncate(len);
    }

    /// Shortens the buffer, dropping the first `cnt` bytes and keeping the
    /// rest.
    ///
    /// This is the same function as `Buf::advance`, and in the next breaking
    /// release of `bytes`, this implementation will be removed in favor of
    /// having `BytesMut` implement `Buf`.
    ///
    /// # Panics
    ///
    /// This function panics if `cnt` is greater than `self.len()`
    #[inline]
    pub fn advance(&mut self, cnt: usize) {
        assert!(cnt <= self.len(), "cannot advance past `remaining`");
        unsafe { self.inner.set_start(cnt); }
    }

    /// Clears the buffer, removing all data.
    ///
    /// # Examples
    ///
    /// ```
    /// use bytes::BytesMut;
    ///
    /// let mut buf = BytesMut::from(&b"hello world"[..]);
    /// buf.clear();
    /// assert!(buf.is_empty());
    /// ```
    pub fn clear(&mut self) {
        self.truncate(0);
    }

    /// Resizes the buffer so that `len` is equal to `new_len`.
    ///
    /// If `new_len` is greater than `len`, the buffer is extended by the
    /// difference with each additional byte set to `value`. If `new_len` is
    /// less than `len`, the buffer is simply truncated.
    ///
    /// # Examples
    ///
    /// ```
    /// use bytes::BytesMut;
    ///
    /// let mut buf = BytesMut::new();
    ///
    /// buf.resize(3, 0x1);
    /// assert_eq!(&buf[..], &[0x1, 0x1, 0x1]);
    ///
    /// buf.resize(2, 0x2);
    /// assert_eq!(&buf[..], &[0x1, 0x1]);
    ///
    /// buf.resize(4, 0x3);
    /// assert_eq!(&buf[..], &[0x1, 0x1, 0x3, 0x3]);
    /// ```
    pub fn resize(&mut self, new_len: usize, value: u8) {
        self.inner.resize(new_len, value);
    }

    /// Sets the length of the buffer.
    ///
    /// This will explicitly set the size of the buffer without actually
    /// modifying the data, so it is up to the caller to ensure that the data
    /// has been initialized.
    ///
    /// # Examples
    ///
    /// ```
    /// use bytes::BytesMut;
    ///
    /// let mut b = BytesMut::from(&b"hello world"[..]);
    ///
    /// unsafe {
    ///     b.set_len(5);
    /// }
    ///
    /// assert_eq!(&b[..], b"hello");
    ///
    /// unsafe {
    ///     b.set_len(11);
    /// }
    ///
    /// assert_eq!(&b[..], b"hello world");
    /// ```
    ///
    /// # Panics
    ///
    /// This method will panic if `len` is out of bounds for the underlying
    /// slice or if it comes after the `end` of the configured window.
    pub unsafe fn set_len(&mut self, len: usize) {
        self.inner.set_len(len)
    }

    /// Reserves capacity for at least `additional` more bytes to be inserted
    /// into the given `BytesMut`.
    ///
    /// More than `additional` bytes may be reserved in order to avoid frequent
    /// reallocations. A call to `reserve` may result in an allocation.
    ///
    /// Before allocating new buffer space, the function will attempt to reclaim
    /// space in the existing buffer. If the current handle references a small
    /// view in the original buffer and all other handles have been dropped,
    /// and the requested capacity is less than or equal to the existing
    /// buffer's capacity, then the current view will be copied to the front of
    /// the buffer and the handle will take ownership of the full buffer.
    ///
    /// # Examples
    ///
    /// In the following example, a new buffer is allocated.
    ///
    /// ```
    /// use bytes::BytesMut;
    ///
    /// let mut buf = BytesMut::from(&b"hello"[..]);
    /// buf.reserve(64);
    /// assert!(buf.capacity() >= 69);
    /// ```
    ///
    /// In the following example, the existing buffer is reclaimed.
    ///
    /// ```
    /// use bytes::{BytesMut, BufMut};
    ///
    /// let mut buf = BytesMut::with_capacity(128);
    /// buf.put(&[0; 64][..]);
    ///
    /// let ptr = buf.as_ptr();
    /// let other = buf.take();
    ///
    /// assert!(buf.is_empty());
    /// assert_eq!(buf.capacity(), 64);
    ///
    /// drop(other);
    /// buf.reserve(128);
    ///
    /// assert_eq!(buf.capacity(), 128);
    /// assert_eq!(buf.as_ptr(), ptr);
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if the new capacity overflows `usize`.
    pub fn reserve(&mut self, additional: usize) {
        self.inner.reserve(additional)
    }

    /// Appends given bytes to this object.
    ///
    /// If this `BytesMut` object has not enough capacity, it is resized first.
    /// So unlike `put_slice` operation, `extend_from_slice` does not panic.
    ///
    /// # Examples
    ///
    /// ```
    /// use bytes::BytesMut;
    ///
    /// let mut buf = BytesMut::with_capacity(0);
    /// buf.extend_from_slice(b"aaabbb");
    /// buf.extend_from_slice(b"cccddd");
    ///
    /// assert_eq!(b"aaabbbcccddd", &buf[..]);
    /// ```
    pub fn extend_from_slice(&mut self, extend: &[u8]) {
        self.reserve(extend.len());
        self.put_slice(extend);
    }

    /// Combine splitted BytesMut objects back as contiguous.
    ///
    /// If `BytesMut` objects were not contiguous originally, they will be extended.
    ///
    /// # Examples
    ///
    /// ```
    /// use bytes::BytesMut;
    ///
    /// let mut buf = BytesMut::with_capacity(64);
    /// buf.extend_from_slice(b"aaabbbcccddd");
    ///
    /// let splitted = buf.split_off(6);
    /// assert_eq!(b"aaabbb", &buf[..]);
    /// assert_eq!(b"cccddd", &splitted[..]);
    ///
    /// buf.unsplit(splitted);
    /// assert_eq!(b"aaabbbcccddd", &buf[..]);
    /// ```
    pub fn unsplit(&mut self, other: BytesMut) {
        let ptr;

        if other.is_empty() {
            return;
        }

        if self.is_empty() {
            *self = other;
            return;
        }

        unsafe {
            ptr = self.inner.ptr.offset(self.inner.len as isize); 
        }
        if ptr == other.inner.ptr &&
           self.inner.kind() == KIND_ARC &&
           other.inner.kind() == KIND_ARC
        {
            debug_assert_eq!(self.inner.arc.load(Acquire),
                             other.inner.arc.load(Acquire));
            // Contiguous blocks, just combine directly
            self.inner.len += other.inner.len;
            self.inner.cap += other.inner.cap;
        }
        else {
            self.extend_from_slice(&other);
        }
    }
}

impl BufMut for BytesMut {
    #[inline]
    fn remaining_mut(&self) -> usize {
        self.capacity() - self.len()
    }

    #[inline]
    unsafe fn advance_mut(&mut self, cnt: usize) {
        let new_len = self.len() + cnt;

        // This call will panic if `cnt` is too big
        self.inner.set_len(new_len);
    }

    #[inline]
    unsafe fn bytes_mut(&mut self) -> &mut [u8] {
        let len = self.len();

        // This will never panic as `len` can never become invalid
        &mut self.inner.as_raw()[len..]
    }

    #[inline]
    fn put_slice(&mut self, src: &[u8]) {
        assert!(self.remaining_mut() >= src.len());

        let len = src.len();

        unsafe {
            self.bytes_mut()[..len].copy_from_slice(src);
            self.advance_mut(len);
        }
    }

    #[inline]
    fn put_u8(&mut self, n: u8) {
        self.inner.put_u8(n);
    }

    #[inline]
    fn put_i8(&mut self, n: i8) {
        self.put_u8(n as u8);
    }
}

impl IntoBuf for BytesMut {
    type Buf = Cursor<Self>;

    fn into_buf(self) -> Self::Buf {
        Cursor::new(self)
    }
}

impl<'a> IntoBuf for &'a BytesMut {
    type Buf = Cursor<&'a BytesMut>;

    fn into_buf(self) -> Self::Buf {
        Cursor::new(self)
    }
}

impl AsRef<[u8]> for BytesMut {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        self.inner.as_ref()
    }
}

impl ops::Deref for BytesMut {
    type Target = [u8];

    #[inline]
    fn deref(&self) -> &[u8] {
        self.as_ref()
    }
}

impl AsMut<[u8]> for BytesMut {
    fn as_mut(&mut self) -> &mut [u8] {
        self.inner.as_mut()
    }
}

impl ops::DerefMut for BytesMut {
    #[inline]
    fn deref_mut(&mut self) -> &mut [u8] {
        self.inner.as_mut()
    }
}

impl From<Vec<u8>> for BytesMut {
    fn from(src: Vec<u8>) -> BytesMut {
        BytesMut {
            inner: Inner::from_vec(src),
        }
    }
}

impl From<String> for BytesMut {
    fn from(src: String) -> BytesMut {
        BytesMut::from(src.into_bytes())
    }
}

impl<'a> From<&'a [u8]> for BytesMut {
    fn from(src: &'a [u8]) -> BytesMut {
        let len = src.len();

        if len == 0 {
            BytesMut::new()
        } else if len <= INLINE_CAP {
            unsafe {
                let mut inner: Inner = mem::uninitialized();

                // Set inline mask
                inner.arc = AtomicPtr::new(KIND_INLINE as *mut Shared);
                inner.set_inline_len(len);
                inner.as_raw()[0..len].copy_from_slice(src);

                BytesMut {
                    inner: inner,
                }
            }
        } else {
            BytesMut::from(src.to_vec())
        }
    }
}

impl<'a> From<&'a str> for BytesMut {
    fn from(src: &'a str) -> BytesMut {
        BytesMut::from(src.as_bytes())
    }
}

impl From<Bytes> for BytesMut {
    fn from(src: Bytes) -> BytesMut {
        src.try_mut()
            .unwrap_or_else(|src| BytesMut::from(&src[..]))
    }
}

impl PartialEq for BytesMut {
    fn eq(&self, other: &BytesMut) -> bool {
        self.inner.as_ref() == other.inner.as_ref()
    }
}

impl PartialOrd for BytesMut {
    fn partial_cmp(&self, other: &BytesMut) -> Option<cmp::Ordering> {
        self.inner.as_ref().partial_cmp(other.inner.as_ref())
    }
}

impl Ord for BytesMut {
    fn cmp(&self, other: &BytesMut) -> cmp::Ordering {
        self.inner.as_ref().cmp(other.inner.as_ref())
    }
}

impl Eq for BytesMut {
}

impl Default for BytesMut {
    #[inline]
    fn default() -> BytesMut {
        BytesMut::new()
    }
}

impl fmt::Debug for BytesMut {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&debug::BsDebug(&self.inner.as_ref()), fmt)
    }
}

impl hash::Hash for BytesMut {
    fn hash<H>(&self, state: &mut H) where H: hash::Hasher {
        let s: &[u8] = self.as_ref();
        s.hash(state);
    }
}

impl Borrow<[u8]> for BytesMut {
    fn borrow(&self) -> &[u8] {
        self.as_ref()
    }
}

impl BorrowMut<[u8]> for BytesMut {
    fn borrow_mut(&mut self) -> &mut [u8] {
        self.as_mut()
    }
}

impl fmt::Write for BytesMut {
    #[inline]
    fn write_str(&mut self, s: &str) -> fmt::Result {
        if self.remaining_mut() >= s.len() {
            self.put_slice(s.as_bytes());
            Ok(())
        } else {
            Err(fmt::Error)
        }
    }

    #[inline]
    fn write_fmt(&mut self, args: fmt::Arguments) -> fmt::Result {
        fmt::write(self, args)
    }
}

impl Clone for BytesMut {
    fn clone(&self) -> BytesMut {
        BytesMut::from(&self[..])
    }
}

impl IntoIterator for BytesMut {
    type Item = u8;
    type IntoIter = Iter<Cursor<BytesMut>>;

    fn into_iter(self) -> Self::IntoIter {
        self.into_buf().iter()
    }
}

impl<'a> IntoIterator for &'a BytesMut {
    type Item = u8;
    type IntoIter = Iter<Cursor<&'a BytesMut>>;

    fn into_iter(self) -> Self::IntoIter {
        self.into_buf().iter()
    }
}

impl Extend<u8> for BytesMut {
    fn extend<T>(&mut self, iter: T) where T: IntoIterator<Item = u8> {
        let iter = iter.into_iter();

        let (lower, _) = iter.size_hint();
        self.reserve(lower);

        for b in iter {
            unsafe {
                self.bytes_mut()[0] = b;
                self.advance_mut(1);
            }
        }
    }
}

impl<'a> Extend<&'a u8> for BytesMut {
    fn extend<T>(&mut self, iter: T) where T: IntoIterator<Item = &'a u8> {
        self.extend(iter.into_iter().map(|b| *b))
    }
}

/*
 *
 * ===== Inner =====
 *
 */

impl Inner {
    #[inline]
    fn from_static(bytes: &'static [u8]) -> Inner {
        let ptr = bytes.as_ptr() as *mut u8;

        Inner {
            // `arc` won't ever store a pointer. Instead, use it to
            // track the fact that the `Bytes` handle is backed by a
            // static buffer.
            arc: AtomicPtr::new(KIND_STATIC as *mut Shared),
            ptr: ptr,
            len: bytes.len(),
            cap: bytes.len(),
        }
    }

    #[inline]
    fn from_vec(mut src: Vec<u8>) -> Inner {
        let len = src.len();
        let cap = src.capacity();
        let ptr = src.as_mut_ptr();

        mem::forget(src);

        let original_capacity_repr = original_capacity_to_repr(cap);
        let arc = (original_capacity_repr << ORIGINAL_CAPACITY_OFFSET) | KIND_VEC;

        Inner {
            arc: AtomicPtr::new(arc as *mut Shared),
            ptr: ptr,
            len: len,
            cap: cap,
        }
    }

    #[inline]
    fn with_capacity(capacity: usize) -> Inner {
        if capacity <= INLINE_CAP {
            unsafe {
                // Using uninitialized memory is ~30% faster
                let mut inner: Inner = mem::uninitialized();
                inner.arc = AtomicPtr::new(KIND_INLINE as *mut Shared);
                inner
            }
        } else {
            Inner::from_vec(Vec::with_capacity(capacity))
        }
    }

    /// Return a slice for the handle's view into the shared buffer
    #[inline]
    fn as_ref(&self) -> &[u8] {
        unsafe {
            if self.is_inline() {
                slice::from_raw_parts(self.inline_ptr(), self.inline_len())
            } else {
                slice::from_raw_parts(self.ptr, self.len)
            }
        }
    }

    /// Return a mutable slice for the handle's view into the shared buffer
    #[inline]
    fn as_mut(&mut self) -> &mut [u8] {
        debug_assert!(!self.is_static());

        unsafe {
            if self.is_inline() {
                slice::from_raw_parts_mut(self.inline_ptr(), self.inline_len())
            } else {
                slice::from_raw_parts_mut(self.ptr, self.len)
            }
        }
    }

    /// Return a mutable slice for the handle's view into the shared buffer
    /// including potentially uninitialized bytes.
    #[inline]
    unsafe fn as_raw(&mut self) -> &mut [u8] {
        debug_assert!(!self.is_static());

        if self.is_inline() {
            slice::from_raw_parts_mut(self.inline_ptr(), INLINE_CAP)
        } else {
            slice::from_raw_parts_mut(self.ptr, self.cap)
        }
    }

    /// Insert a byte into the next slot and advance the len by 1.
    #[inline]
    fn put_u8(&mut self, n: u8) {
        if self.is_inline() {
            let len = self.inline_len();
            assert!(len < INLINE_CAP);
            unsafe {
                *self.inline_ptr().offset(len as isize) = n;
            }
            self.set_inline_len(len + 1);
        } else {
            assert!(self.len < self.cap);
            unsafe {
                *self.ptr.offset(self.len as isize) = n;
            }
            self.len += 1;
        }
    }

    #[inline]
    fn len(&self) -> usize {
        if self.is_inline() {
            self.inline_len()
        } else {
            self.len
        }
    }

    /// Pointer to the start of the inline buffer
    #[inline]
    unsafe fn inline_ptr(&self) -> *mut u8 {
        (self as *const Inner as *mut Inner as *mut u8)
            .offset(INLINE_DATA_OFFSET)
    }

    #[inline]
    fn inline_len(&self) -> usize {
        let p: &usize = unsafe { mem::transmute(&self.arc) };
        (p & INLINE_LEN_MASK) >> INLINE_LEN_OFFSET
    }

    /// Set the length of the inline buffer. This is done by writing to the
    /// least significant byte of the `arc` field.
    #[inline]
    fn set_inline_len(&mut self, len: usize) {
        debug_assert!(len <= INLINE_CAP);
        let p = self.arc.get_mut();
        *p = ((*p as usize & !INLINE_LEN_MASK) | (len << INLINE_LEN_OFFSET)) as _;
    }

    /// slice.
    #[inline]
    unsafe fn set_len(&mut self, len: usize) {
        if self.is_inline() {
            assert!(len <= INLINE_CAP);
            self.set_inline_len(len);
        } else {
            assert!(len <= self.cap);
            self.len = len;
        }
    }

    #[inline]
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[inline]
    fn capacity(&self) -> usize {
        if self.is_inline() {
            INLINE_CAP
        } else {
            self.cap
        }
    }

    fn split_off(&mut self, at: usize) -> Inner {
        let mut other = unsafe { self.shallow_clone(true) };

        unsafe {
            other.set_start(at);
            self.set_end(at);
        }

        return other
    }

    fn split_to(&mut self, at: usize) -> Inner {
        let mut other = unsafe { self.shallow_clone(true) };

        unsafe {
            other.set_end(at);
            self.set_start(at);
        }

        return other
    }

    fn truncate(&mut self, len: usize) {
        if len <= self.len() {
            unsafe { self.set_len(len); }
        }
    }

    fn resize(&mut self, new_len: usize, value: u8) {
        let len = self.len();
        if new_len > len {
            let additional = new_len - len;
            self.reserve(additional);
            unsafe {
                let dst = self.as_raw()[len..].as_mut_ptr();
                ptr::write_bytes(dst, value, additional);
                self.set_len(new_len);
            }
        } else {
            self.truncate(new_len);
        }
    }

    unsafe fn set_start(&mut self, start: usize) {
        // Setting the start to 0 is a no-op, so return early if this is the
        // case.
        if start == 0 {
            return;
        }

        let kind = self.kind();

        // Always check `inline` first, because if the handle is using inline
        // data storage, all of the `Inner` struct fields will be gibberish.
        if kind == KIND_INLINE {
            assert!(start <= INLINE_CAP);

            let len = self.inline_len();

            if len <= start {
                self.set_inline_len(0);
            } else {
                // `set_start` is essentially shifting data off the front of the
                // view. Inlined buffers only track the length of the slice.
                // So, to update the start, the data at the new starting point
                // is copied to the beginning of the buffer.
                let new_len = len - start;

                let dst = self.inline_ptr();
                let src = (dst as *const u8).offset(start as isize);

                ptr::copy(src, dst, new_len);

                self.set_inline_len(new_len);
            }
        } else {
            assert!(start <= self.cap);

            if kind == KIND_VEC {
                // Setting the start when in vec representation is a little more
                // complicated. First, we have to track how far ahead the
                // "start" of the byte buffer from the beginning of the vec. We
                // also have to ensure that we don't exceed the maximum shift.
                let (mut pos, prev) = self.uncoordinated_get_vec_pos();
                pos += start;

                if pos <= MAX_VEC_POS {
                    self.uncoordinated_set_vec_pos(pos, prev);
                } else {
                    // The repr must be upgraded to ARC. This will never happen
                    // on 64 bit systems and will only happen on 32 bit systems
                    // when shifting past 134,217,727 bytes. As such, we don't
                    // worry too much about performance here.
                    let _ = self.shallow_clone(true);
                }
            }

            // Updating the start of the view is setting `ptr` to point to the
            // new start and updating the `len` field to reflect the new length
            // of the view.
            self.ptr = self.ptr.offset(start as isize);

            if self.len >= start {
                self.len -= start;
            } else {
                self.len = 0;
            }

            self.cap -= start;
        }
    }

    unsafe fn set_end(&mut self, end: usize) {
        debug_assert!(self.is_shared());

        // Always check `inline` first, because if the handle is using inline
        // data storage, all of the `Inner` struct fields will be gibberish.
        if self.is_inline() {
            assert!(end <= INLINE_CAP);
            let new_len = cmp::min(self.inline_len(), end);
            self.set_inline_len(new_len);
        } else {
            assert!(end <= self.cap);

            self.cap = end;
            self.len = cmp::min(self.len, end);
        }
    }

    /// Checks if it is safe to mutate the memory
    fn is_mut_safe(&mut self) -> bool {
        let kind = self.kind();

        // Always check `inline` first, because if the handle is using inline
        // data storage, all of the `Inner` struct fields will be gibberish.
        if kind == KIND_INLINE {
            // Inlined buffers can always be mutated as the data is never shared
            // across handles.
            true
        } else if kind == KIND_VEC {
            true
        } else if kind == KIND_STATIC {
            false
        } else {
            // Otherwise, the underlying buffer is potentially shared with other
            // handles, so the ref_count needs to be checked.
            unsafe { (**self.arc.get_mut()).is_unique() }
        }
    }

    /// Increments the ref count. This should only be done if it is known that
    /// it can be done safely. As such, this fn is not public, instead other
    /// fns will use this one while maintaining the guarantees.
    /// Parameter `mut_self` should only be set to `true` if caller holds
    /// `&mut self` reference.
    ///
    /// "Safely" is defined as not exposing two `BytesMut` values that point to
    /// the same byte window.
    ///
    /// This function is thread safe.
    unsafe fn shallow_clone(&self, mut_self: bool) -> Inner {
        // Always check `inline` first, because if the handle is using inline
        // data storage, all of the `Inner` struct fields will be gibberish.
        //
        // Additionally, if kind is STATIC, then Arc is *never* changed, making
        // it safe and faster to check for it now before an atomic acquire.

        if self.is_inline_or_static() {
            // In this case, a shallow_clone still involves copying the data.
            let mut inner: Inner = mem::uninitialized();
            ptr::copy_nonoverlapping(
                self,
                &mut inner,
                1,
            );
            inner
        } else {
            self.shallow_clone_sync(mut_self)
        }
    }


    #[cold]
    unsafe fn shallow_clone_sync(&self, mut_self: bool) -> Inner {
        // The function requires `&self`, this means that `shallow_clone`
        // could be called concurrently.
        //
        // The first step is to load the value of `arc`. This will determine
        // how to proceed. The `Acquire` ordering synchronizes with the
        // `compare_and_swap` that comes later in this function. The goal is
        // to ensure that if `arc` is currently set to point to a `Shared`,
        // that the current thread acquires the associated memory.
        let arc = self.arc.load(Acquire);
        let kind = arc as usize & KIND_MASK;

        if kind == KIND_ARC {
            self.shallow_clone_arc(arc)
        } else {
            assert!(kind == KIND_VEC);
            self.shallow_clone_vec(arc as usize, mut_self)
        }
    }

    unsafe fn shallow_clone_arc(&self, arc: *mut Shared) -> Inner {
        debug_assert!(arc as usize & KIND_MASK == KIND_ARC);

        let old_size = (*arc).ref_count.fetch_add(1, Relaxed);

        if old_size == usize::MAX {
            abort();
        }

        Inner {
            arc: AtomicPtr::new(arc),
            .. *self
        }
    }

    #[cold]
    unsafe fn shallow_clone_vec(&self, arc: usize, mut_self: bool) -> Inner {
        // If  the buffer is still tracked in a `Vec<u8>`. It is time to
        // promote the vec to an `Arc`. This could potentially be called
        // concurrently, so some care must be taken.

        debug_assert!(arc & KIND_MASK == KIND_VEC);

        let original_capacity_repr =
            (arc as usize & ORIGINAL_CAPACITY_MASK) >> ORIGINAL_CAPACITY_OFFSET;

        // The vec offset cannot be concurrently mutated, so there
        // should be no danger reading it.
        let off = (arc as usize) >> VEC_POS_OFFSET;

        // First, allocate a new `Shared` instance containing the
        // `Vec` fields. It's important to note that `ptr`, `len`,
        // and `cap` cannot be mutated without having `&mut self`.
        // This means that these fields will not be concurrently
        // updated and since the buffer hasn't been promoted to an
        // `Arc`, those three fields still are the components of the
        // vector.
        let shared = Box::new(Shared {
            vec: rebuild_vec(self.ptr, self.len, self.cap, off),
            original_capacity_repr: original_capacity_repr,
            // Initialize refcount to 2. One for this reference, and one
            // for the new clone that will be returned from
            // `shallow_clone`.
            ref_count: AtomicUsize::new(2),
        });

        let shared = Box::into_raw(shared);

        // The pointer should be aligned, so this assert should
        // always succeed.
        debug_assert!(0 == (shared as usize & 0b11));

        // If there are no references to self in other threads,
        // expensive atomic operations can be avoided.
        if mut_self {
            self.arc.store(shared, Relaxed);
            return Inner {
                arc: AtomicPtr::new(shared),
                .. *self
            };
        }

        // Try compare & swapping the pointer into the `arc` field.
        // `Release` is used synchronize with other threads that
        // will load the `arc` field.
        //
        // If the `compare_and_swap` fails, then the thread lost the
        // race to promote the buffer to shared. The `Acquire`
        // ordering will synchronize with the `compare_and_swap`
        // that happened in the other thread and the `Shared`
        // pointed to by `actual` will be visible.
        let actual = self.arc.compare_and_swap(arc as *mut Shared, shared, AcqRel);

        if actual as usize == arc {
            // The upgrade was successful, the new handle can be
            // returned.
            return Inner {
                arc: AtomicPtr::new(shared),
                .. *self
            };
        }

        // The upgrade failed, a concurrent clone happened. Release
        // the allocation that was made in this thread, it will not
        // be needed.
        let shared = Box::from_raw(shared);
        mem::forget(*shared);

        // Buffer already promoted to shared storage, so increment ref
        // count.
        self.shallow_clone_arc(actual)
    }

    #[inline]
    fn reserve(&mut self, additional: usize) {
        let len = self.len();
        let rem = self.capacity() - len;

        if additional <= rem {
            // The handle can already store at least `additional` more bytes, so
            // there is no further work needed to be done.
            return;
        }

        let kind = self.kind();

        // Always check `inline` first, because if the handle is using inline
        // data storage, all of the `Inner` struct fields will be gibberish.
        if kind == KIND_INLINE {
            let new_cap = len + additional;

            // Promote to a vector
            let mut v = Vec::with_capacity(new_cap);
            v.extend_from_slice(self.as_ref());

            self.ptr = v.as_mut_ptr();
            self.len = v.len();
            self.cap = v.capacity();

            // Since the minimum capacity is `INLINE_CAP`, don't bother encoding
            // the original capacity as INLINE_CAP
            self.arc = AtomicPtr::new(KIND_VEC as *mut Shared);

            mem::forget(v);
            return;
        }

        if kind == KIND_VEC {
            // If there's enough free space before the start of the buffer, then
            // just copy the data backwards and reuse the already-allocated
            // space.
            //
            // Otherwise, since backed by a vector, use `Vec::reserve`
            unsafe {
                let (off, prev) = self.uncoordinated_get_vec_pos();

                // Only reuse space if we stand to gain at least capacity/2
                // bytes of space back
                if off >= additional && off >= (self.cap / 2) {
                    // There's space - reuse it
                    //
                    // Just move the pointer back to the start after copying
                    // data back.
                    let base_ptr = self.ptr.offset(-(off as isize));
                    ptr::copy(self.ptr, base_ptr, self.len);
                    self.ptr = base_ptr;
                    self.uncoordinated_set_vec_pos(0, prev);

                    // Length stays constant, but since we moved backwards we
                    // can gain capacity back.
                    self.cap += off;
                } else {
                    // No space - allocate more
                    let mut v = rebuild_vec(self.ptr, self.len, self.cap, off);
                    v.reserve(additional);

                    // Update the info
                    self.ptr = v.as_mut_ptr().offset(off as isize);
                    self.len = v.len() - off;
                    self.cap = v.capacity() - off;

                    // Drop the vec reference
                    mem::forget(v);
                }
                return;
            }
        }

        let arc = *self.arc.get_mut();

        debug_assert!(kind == KIND_ARC);

        // Reserving involves abandoning the currently shared buffer and
        // allocating a new vector with the requested capacity.
        //
        // Compute the new capacity
        let mut new_cap = len + additional;
        let original_capacity;
        let original_capacity_repr;

        unsafe {
            original_capacity_repr = (*arc).original_capacity_repr;
            original_capacity = original_capacity_from_repr(original_capacity_repr);

            // First, try to reclaim the buffer. This is possible if the current
            // handle is the only outstanding handle pointing to the buffer.
            if (*arc).is_unique() {
                // This is the only handle to the buffer. It can be reclaimed.
                // However, before doing the work of copying data, check to make
                // sure that the vector has enough capacity.
                let v = &mut (*arc).vec;

                if v.capacity() >= new_cap {
                    // The capacity is sufficient, reclaim the buffer
                    let ptr = v.as_mut_ptr();

                    ptr::copy(self.ptr, ptr, len);

                    self.ptr = ptr;
                    self.cap = v.capacity();

                    return;
                }

                // The vector capacity is not sufficient. The reserve request is
                // asking for more than the initial buffer capacity. Allocate more
                // than requested if `new_cap` is not much bigger than the current
                // capacity.
                //
                // There are some situations, using `reserve_exact` that the
                // buffer capacity could be below `original_capacity`, so do a
                // check.
                new_cap = cmp::max(
                    cmp::max(v.capacity() << 1, new_cap),
                    original_capacity);
            } else {
                new_cap = cmp::max(new_cap, original_capacity);
            }
        }

        // Create a new vector to store the data
        let mut v = Vec::with_capacity(new_cap);

        // Copy the bytes
        v.extend_from_slice(self.as_ref());

        // Release the shared handle. This must be done *after* the bytes are
        // copied.
        release_shared(arc);

        // Update self
        self.ptr = v.as_mut_ptr();
        self.len = v.len();
        self.cap = v.capacity();

        let arc = (original_capacity_repr << ORIGINAL_CAPACITY_OFFSET) | KIND_VEC;

        self.arc = AtomicPtr::new(arc as *mut Shared);

        // Forget the vector handle
        mem::forget(v);
    }

    /// Returns true if the buffer is stored inline
    #[inline]
    fn is_inline(&self) -> bool {
        self.kind() == KIND_INLINE
    }

    #[inline]
    fn is_inline_or_static(&self) -> bool {
        // The value returned by `kind` isn't itself safe, but the value could
        // inform what operations to take, and unsafely do something without
        // synchronization.
        //
        // KIND_INLINE and KIND_STATIC will *never* change, so branches on that
        // information is safe.
        let kind = self.kind();
        kind == KIND_INLINE || kind == KIND_STATIC
    }

    /// Used for `debug_assert` statements. &mut is used to guarantee that it is
    /// safe to check VEC_KIND
    #[inline]
    fn is_shared(&mut self) -> bool {
        match self.kind() {
            KIND_VEC => false,
            _ => true,
        }
    }

    /// Used for `debug_assert` statements
    #[inline]
    fn is_static(&mut self) -> bool {
        match self.kind() {
            KIND_STATIC => true,
            _ => false,
        }
    }

    #[inline]
    fn kind(&self) -> usize {
        // This function is going to probably raise some eyebrows. The function
        // returns true if the buffer is stored inline. This is done by checking
        // the least significant bit in the `arc` field.
        //
        // Now, you may notice that `arc` is an `AtomicPtr` and this is
        // accessing it as a normal field without performing an atomic load...
        //
        // Again, the function only cares about the least significant bit, and
        // this bit is set when `Inner` is created and never changed after that.
        // All platforms have atomic "word" operations and won't randomly flip
        // bits, so even without any explicit atomic operations, reading the
        // flag will be correct.
        //
        // This is undefind behavior due to a data race, but experimental
        // evidence shows that it works in practice (discussion:
        // https://internals.rust-lang.org/t/bit-wise-reasoning-for-atomic-accesses/8853).
        //
        // This function is very critical performance wise as it is called for
        // every operation. Performing an atomic load would mess with the
        // compiler's ability to optimize. Simple benchmarks show up to a 10%
        // slowdown using a `Relaxed` atomic load on x86.

        #[cfg(target_endian = "little")]
        #[inline]
        fn imp(arc: &AtomicPtr<Shared>) -> usize {
            unsafe {
                let p: *const u8 = mem::transmute(arc);
                (*p as usize) & KIND_MASK
            }
        }

        #[cfg(target_endian = "big")]
        #[inline]
        fn imp(arc: &AtomicPtr<Shared>) -> usize {
            unsafe {
                let p: *const usize = mem::transmute(arc);
                *p & KIND_MASK
            }
        }

        imp(&self.arc)
    }

    #[inline]
    fn uncoordinated_get_vec_pos(&mut self) -> (usize, usize) {
        // Similar to above, this is a pretty crazed function. This should only
        // be called when in the KIND_VEC mode. This + the &mut self argument
        // guarantees that there is no possibility of concurrent calls to this
        // function.
        let prev = unsafe {
            let p: &AtomicPtr<Shared> = &self.arc;
            let p: *const usize = mem::transmute(p);
            *p
        };

        (prev >> VEC_POS_OFFSET, prev)
    }

    #[inline]
    fn uncoordinated_set_vec_pos(&mut self, pos: usize, prev: usize) {
        // Once more... crazy
        debug_assert!(pos <= MAX_VEC_POS);

        unsafe {
            let p: &mut AtomicPtr<Shared> = &mut self.arc;
            let p: &mut usize = mem::transmute(p);
            *p = (pos << VEC_POS_OFFSET) | (prev & NOT_VEC_POS_MASK);
        }
    }
}

fn rebuild_vec(ptr: *mut u8, mut len: usize, mut cap: usize, off: usize) -> Vec<u8> {
    unsafe {
        let ptr = ptr.offset(-(off as isize));
        len += off;
        cap += off;

        Vec::from_raw_parts(ptr, len, cap)
    }
}

impl Drop for Inner {
    fn drop(&mut self) {
        let kind = self.kind();

        if kind == KIND_VEC {
            let (off, _) = self.uncoordinated_get_vec_pos();

            // Vector storage, free the vector
            let _ = rebuild_vec(self.ptr, self.len, self.cap, off);
        } else if kind == KIND_ARC {
            release_shared(*self.arc.get_mut());
        }
    }
}

fn release_shared(ptr: *mut Shared) {
    // `Shared` storage... follow the drop steps from Arc.
    unsafe {
        if (*ptr).ref_count.fetch_sub(1, Release) != 1 {
            return;
        }

        // This fence is needed to prevent reordering of use of the data and
        // deletion of the data.  Because it is marked `Release`, the decreasing
        // of the reference count synchronizes with this `Acquire` fence. This
        // means that use of the data happens before decreasing the reference
        // count, which happens before this fence, which happens before the
        // deletion of the data.
        //
        // As explained in the [Boost documentation][1],
        //
        // > It is important to enforce any possible access to the object in one
        // > thread (through an existing reference) to *happen before* deleting
        // > the object in a different thread. This is achieved by a "release"
        // > operation after dropping a reference (any access to the object
        // > through this reference must obviously happened before), and an
        // > "acquire" operation before deleting the object.
        //
        // [1]: (www.boost.org/doc/libs/1_55_0/doc/html/atomic/usage_examples.html)
        atomic::fence(Acquire);

        // Drop the data
        Box::from_raw(ptr);
    }
}

impl Shared {
    fn is_unique(&self) -> bool {
        // The goal is to check if the current handle is the only handle
        // that currently has access to the buffer. This is done by
        // checking if the `ref_count` is currently 1.
        //
        // The `Acquire` ordering synchronizes with the `Release` as
        // part of the `fetch_sub` in `release_shared`. The `fetch_sub`
        // operation guarantees that any mutations done in other threads
        // are ordered before the `ref_count` is decremented. As such,
        // this `Acquire` will guarantee that those mutations are
        // visible to the current thread.
        self.ref_count.load(Acquire) == 1
    }
}

fn original_capacity_to_repr(cap: usize) -> usize {
    let width = PTR_WIDTH - ((cap >> MIN_ORIGINAL_CAPACITY_WIDTH).leading_zeros() as usize);
    cmp::min(width, MAX_ORIGINAL_CAPACITY_WIDTH - MIN_ORIGINAL_CAPACITY_WIDTH)
}

fn original_capacity_from_repr(repr: usize) -> usize {
    if repr == 0 {
        return 0;
    }

    1 << (repr + (MIN_ORIGINAL_CAPACITY_WIDTH - 1))
}

#[test]
fn test_original_capacity_to_repr() {
    assert_eq!(original_capacity_to_repr(0), 0);

    let max_width = 32;

    for width in 1..(max_width + 1) {
        let cap = 1 << width - 1;

        let expected = if width < MIN_ORIGINAL_CAPACITY_WIDTH {
            0
        } else if width < MAX_ORIGINAL_CAPACITY_WIDTH {
            width - MIN_ORIGINAL_CAPACITY_WIDTH
        } else {
            MAX_ORIGINAL_CAPACITY_WIDTH - MIN_ORIGINAL_CAPACITY_WIDTH
        };

        assert_eq!(original_capacity_to_repr(cap), expected);

        if width > 1 {
            assert_eq!(original_capacity_to_repr(cap + 1), expected);
        }

        //  MIN_ORIGINAL_CAPACITY_WIDTH must be bigger than 7 to pass tests below
        if width == MIN_ORIGINAL_CAPACITY_WIDTH + 1 {
            assert_eq!(original_capacity_to_repr(cap - 24), expected - 1);
            assert_eq!(original_capacity_to_repr(cap + 76), expected);
        } else if width == MIN_ORIGINAL_CAPACITY_WIDTH + 2 {
            assert_eq!(original_capacity_to_repr(cap - 1), expected - 1);
            assert_eq!(original_capacity_to_repr(cap - 48), expected - 1);
        }
    }
}

#[test]
fn test_original_capacity_from_repr() {
    assert_eq!(0, original_capacity_from_repr(0));

    let min_cap = 1 << MIN_ORIGINAL_CAPACITY_WIDTH;

    assert_eq!(min_cap, original_capacity_from_repr(1));
    assert_eq!(min_cap * 2, original_capacity_from_repr(2));
    assert_eq!(min_cap * 4, original_capacity_from_repr(3));
    assert_eq!(min_cap * 8, original_capacity_from_repr(4));
    assert_eq!(min_cap * 16, original_capacity_from_repr(5));
    assert_eq!(min_cap * 32, original_capacity_from_repr(6));
    assert_eq!(min_cap * 64, original_capacity_from_repr(7));
}

unsafe impl Send for Inner {}
unsafe impl Sync for Inner {}

/*
 *
 * ===== PartialEq / PartialOrd =====
 *
 */

impl PartialEq<[u8]> for BytesMut {
    fn eq(&self, other: &[u8]) -> bool {
        &**self == other
    }
}

impl PartialOrd<[u8]> for BytesMut {
    fn partial_cmp(&self, other: &[u8]) -> Option<cmp::Ordering> {
        (**self).partial_cmp(other)
    }
}

impl PartialEq<BytesMut> for [u8] {
    fn eq(&self, other: &BytesMut) -> bool {
        *other == *self
    }
}

impl PartialOrd<BytesMut> for [u8] {
    fn partial_cmp(&self, other: &BytesMut) -> Option<cmp::Ordering> {
        other.partial_cmp(self)
    }
}

impl PartialEq<str> for BytesMut {
    fn eq(&self, other: &str) -> bool {
        &**self == other.as_bytes()
    }
}

impl PartialOrd<str> for BytesMut {
    fn partial_cmp(&self, other: &str) -> Option<cmp::Ordering> {
        (**self).partial_cmp(other.as_bytes())
    }
}

impl PartialEq<BytesMut> for str {
    fn eq(&self, other: &BytesMut) -> bool {
        *other == *self
    }
}

impl PartialOrd<BytesMut> for str {
    fn partial_cmp(&self, other: &BytesMut) -> Option<cmp::Ordering> {
        other.partial_cmp(self)
    }
}

impl PartialEq<Vec<u8>> for BytesMut {
    fn eq(&self, other: &Vec<u8>) -> bool {
        *self == &other[..]
    }
}

impl PartialOrd<Vec<u8>> for BytesMut {
    fn partial_cmp(&self, other: &Vec<u8>) -> Option<cmp::Ordering> {
        (**self).partial_cmp(&other[..])
    }
}

impl PartialEq<BytesMut> for Vec<u8> {
    fn eq(&self, other: &BytesMut) -> bool {
        *other == *self
    }
}

impl PartialOrd<BytesMut> for Vec<u8> {
    fn partial_cmp(&self, other: &BytesMut) -> Option<cmp::Ordering> {
        other.partial_cmp(self)
    }
}

impl PartialEq<String> for BytesMut {
    fn eq(&self, other: &String) -> bool {
        *self == &other[..]
    }
}

impl PartialOrd<String> for BytesMut {
    fn partial_cmp(&self, other: &String) -> Option<cmp::Ordering> {
        (**self).partial_cmp(other.as_bytes())
    }
}

impl PartialEq<BytesMut> for String {
    fn eq(&self, other: &BytesMut) -> bool {
        *other == *self
    }
}

impl PartialOrd<BytesMut> for String {
    fn partial_cmp(&self, other: &BytesMut) -> Option<cmp::Ordering> {
        other.partial_cmp(self)
    }
}

impl<'a, T: ?Sized> PartialEq<&'a T> for BytesMut
    where BytesMut: PartialEq<T>
{
    fn eq(&self, other: &&'a T) -> bool {
        *self == **other
    }
}

impl<'a, T: ?Sized> PartialOrd<&'a T> for BytesMut
    where BytesMut: PartialOrd<T>
{
    fn partial_cmp(&self, other: &&'a T) -> Option<cmp::Ordering> {
        self.partial_cmp(*other)
    }
}

impl<'a> PartialEq<BytesMut> for &'a [u8] {
    fn eq(&self, other: &BytesMut) -> bool {
        *other == *self
    }
}

impl<'a> PartialOrd<BytesMut> for &'a [u8] {
    fn partial_cmp(&self, other: &BytesMut) -> Option<cmp::Ordering> {
        other.partial_cmp(self)
    }
}

impl<'a> PartialEq<BytesMut> for &'a str {
    fn eq(&self, other: &BytesMut) -> bool {
        *other == *self
    }
}

impl<'a> PartialOrd<BytesMut> for &'a str {
    fn partial_cmp(&self, other: &BytesMut) -> Option<cmp::Ordering> {
        other.partial_cmp(self)
    }
}

impl PartialEq<[u8]> for Bytes {
    fn eq(&self, other: &[u8]) -> bool {
        self.inner.as_ref() == other
    }
}

impl PartialOrd<[u8]> for Bytes {
    fn partial_cmp(&self, other: &[u8]) -> Option<cmp::Ordering> {
        self.inner.as_ref().partial_cmp(other)
    }
}

impl PartialEq<Bytes> for [u8] {
    fn eq(&self, other: &Bytes) -> bool {
        *other == *self
    }
}

impl PartialOrd<Bytes> for [u8] {
    fn partial_cmp(&self, other: &Bytes) -> Option<cmp::Ordering> {
        other.partial_cmp(self)
    }
}

impl PartialEq<str> for Bytes {
    fn eq(&self, other: &str) -> bool {
        self.inner.as_ref() == other.as_bytes()
    }
}

impl PartialOrd<str> for Bytes {
    fn partial_cmp(&self, other: &str) -> Option<cmp::Ordering> {
        self.inner.as_ref().partial_cmp(other.as_bytes())
    }
}

impl PartialEq<Bytes> for str {
    fn eq(&self, other: &Bytes) -> bool {
        *other == *self
    }
}

impl PartialOrd<Bytes> for str {
    fn partial_cmp(&self, other: &Bytes) -> Option<cmp::Ordering> {
        other.partial_cmp(self)
    }
}

impl PartialEq<Vec<u8>> for Bytes {
    fn eq(&self, other: &Vec<u8>) -> bool {
        *self == &other[..]
    }
}

impl PartialOrd<Vec<u8>> for Bytes {
    fn partial_cmp(&self, other: &Vec<u8>) -> Option<cmp::Ordering> {
        self.inner.as_ref().partial_cmp(&other[..])
    }
}

impl PartialEq<Bytes> for Vec<u8> {
    fn eq(&self, other: &Bytes) -> bool {
        *other == *self
    }
}

impl PartialOrd<Bytes> for Vec<u8> {
    fn partial_cmp(&self, other: &Bytes) -> Option<cmp::Ordering> {
        other.partial_cmp(self)
    }
}

impl PartialEq<String> for Bytes {
    fn eq(&self, other: &String) -> bool {
        *self == &other[..]
    }
}

impl PartialOrd<String> for Bytes {
    fn partial_cmp(&self, other: &String) -> Option<cmp::Ordering> {
        self.inner.as_ref().partial_cmp(other.as_bytes())
    }
}

impl PartialEq<Bytes> for String {
    fn eq(&self, other: &Bytes) -> bool {
        *other == *self
    }
}

impl PartialOrd<Bytes> for String {
    fn partial_cmp(&self, other: &Bytes) -> Option<cmp::Ordering> {
        other.partial_cmp(self)
    }
}

impl<'a> PartialEq<Bytes> for &'a [u8] {
    fn eq(&self, other: &Bytes) -> bool {
        *other == *self
    }
}

impl<'a> PartialOrd<Bytes> for &'a [u8] {
    fn partial_cmp(&self, other: &Bytes) -> Option<cmp::Ordering> {
        other.partial_cmp(self)
    }
}

impl<'a> PartialEq<Bytes> for &'a str {
    fn eq(&self, other: &Bytes) -> bool {
        *other == *self
    }
}

impl<'a> PartialOrd<Bytes> for &'a str {
    fn partial_cmp(&self, other: &Bytes) -> Option<cmp::Ordering> {
        other.partial_cmp(self)
    }
}

impl<'a, T: ?Sized> PartialEq<&'a T> for Bytes
    where Bytes: PartialEq<T>
{
    fn eq(&self, other: &&'a T) -> bool {
        *self == **other
    }
}

impl<'a, T: ?Sized> PartialOrd<&'a T> for Bytes
    where Bytes: PartialOrd<T>
{
    fn partial_cmp(&self, other: &&'a T) -> Option<cmp::Ordering> {
        self.partial_cmp(&**other)
    }
}

impl PartialEq<BytesMut> for Bytes
{
    fn eq(&self, other: &BytesMut) -> bool {
        &other[..] == &self[..]
    }
}

impl PartialEq<Bytes> for BytesMut
{
    fn eq(&self, other: &Bytes) -> bool {
        &other[..] == &self[..]
    }
}

// While there is `std::process:abort`, it's only available in Rust 1.17, and
// our minimum supported version is currently 1.15. So, this acts as an abort
// by triggering a double panic, which always aborts in Rust.
struct Abort;

impl Drop for Abort {
    fn drop(&mut self) {
        panic!();
    }
}

#[inline(never)]
#[cold]
fn abort() {
    let _a = Abort;
    panic!();
}
