//! Internal details.
//!
//! While the other parts of documentation are useful to users of the crate, this part is probably
//! helpful only if you want to look into the code or are curious about how it works internally.
//!
//! Also note that any of these details may change in future versions and are not part of the
//! stability guarantees. Don't rely on anything here.
//!
//! # Storing the [`Arc`].
//!
//! The [`Arc`] can be turned into a raw pointer and back. This is abstracted by the [`RefCnt`]
//! trait and it is technically possible to implement it for custom types (this crate also
//! implements it for [`Rc`] and [`Weak`], though the actual usefulness of these is a bit
//! questionable).
//!
//! The raw pointer is stored inside an [`AtomicPtr`].
//!
//! # Protection of reference counts
//!
//! The first idea would be to just use [`AtomicPtr`] with whatever the [`Arc::into_raw`] returns.
//! Then replacing it would be fine (there's no need to update ref counts). The load needs to
//! increment the reference count ‒ one still stays inside and another is returned to the caller.
//! This is done by re-creating the Arc from the raw pointer and then cloning it, throwing one
//! instance away (without destroying it).
//!
//! This approach has a problem. There's a short time between we read the raw pointer and increment
//! the count. If some other thread replaces the stored Arc and throws it away, the ref count could
//! drop to 0, get destroyed and we would be trying to bump ref counts in a ghost, which would be
//! totally broken.
//!
//! To prevent this, we actually use two approaches in a hybrid manner.
//!
//! The first one is based on hazard pointers idea, but slightly modified. There's a global
//! repository of pointers that owe a reference. When someone swaps a pointer, it walks this list
//! and pays all the debts (and takes them out of the repository).
//!
//! For simplicity and performance, storing into the repository is fallible. If storing into the
//! repository fails (because the thread used up all its own slots, or because the pointer got
//! replaced in just the wrong moment and it can't confirm the reservation), unlike the full
//! hazard-pointers approach, we don't retry, but fall back onto secondary strategy.
//!
//! The secondary strategy is similar, but a bit more complex (and therefore slower, that's why it
//! is only a fallback). We first publish an intent to read a pointer (and where we are reading it
//! from). Then we actually do so and publish the debt, like previously.
//!
//! The writer pays the debts as usual. But also, if it sees the intent to read the value, it helps
//! along, reads it, bumps the reference and passes it to the reader. Therefore, if the reader
//! fails to do the protection itself, because it got interrupted by a writer, it finds a
//! ready-made replacement value it can just use and doesn't have to retry. Also, the writer
//! doesn't have to wait for the reader in any way, because it can just solve its problem and move
//! on.
//!
//! # Unsafety
//!
//! All the uses of the unsafe keyword is just to turn the raw pointer back to Arc. It originated
//! from an Arc in the first place, so the only thing to ensure is it is still valid. That means its
//! ref count never dropped to 0.
//!
//! At the beginning, there's ref count of 1 stored in the raw pointer (and maybe some others
//! elsewhere, but we can't rely on these). This 1 stays there for the whole time the pointer is
//! stored there. When the arc is replaced, this 1 is returned to the caller, so we just have to
//! make sure no more readers access it by that time.
//!
//! # Leases and debts
//!
//! Instead of incrementing the reference count, the pointer reference can be owed. In such case, it
//! is recorded into a global storage. As each thread has its own storage (the global storage is
//! composed of multiple thread storages), the readers don't contend. When the pointer is no longer
//! in use, the debt is erased.
//!
//! The writer pays all the existing debts, therefore the reader have the full Arc with ref count at
//! that time. The reader is made aware the debt was paid and decrements the reference count.
//!
//! # Memory orders
//!
//! ## Synchronizing the data pointed to by the pointer.
//!
//! We have AcqRel (well, SeqCst, but that's included) on the swap and Acquire on the loads. In case
//! of the double read around the debt allocation, we do that on the *second*, because of ABA.
//! That's also why that SeqCst on the allocation of debt itself is not enough.
//! the *latest* decrement. By making both the increment and decrement AcqRel, we effectively chain
//! the edges together.
//!
//! # Memory orders around debts
//!
//! The linked list of debt nodes only grows. The shape of the list (existence of nodes) is
//! synchronized through Release on creation and Acquire on load on the head pointer.
//!
//! The debts work similar to locks ‒ Acquire and Release make all the pointer manipulation at the
//! interval where it is written down. However, we use the SeqCst on the allocation of the debt
//! because when we see an empty slot, we need to make sure that it happened after we have
//! overwritten the pointer.
//!
//! In case the writer pays the debt, it sees the new enough data (for the same reasons the stale
//! empties are not seen). The reference count on the Arc is AcqRel and makes sure it is not
//! destroyed too soon. The writer traverses all the slots, therefore they don't need to synchronize
//! with each other.
//!
//! Further details are inside the internal `debt` module.
//!
//! [`RefCnt`]: crate::RefCnt
//! [`Arc`]: std::sync::Arc
//! [`Arc::into_raw`]: std::sync::Arc::into_raw
//! [`Rc`]: std::rc::Rc
//! [`Weak`]: std::sync::Weak
//! [`AtomicPtr`]: std::sync::atomic::AtomicPtr
