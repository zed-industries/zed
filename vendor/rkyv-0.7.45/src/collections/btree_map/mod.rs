//! [`Archive`] implementation for B-tree maps.

#[cfg(feature = "validation")]
pub mod validation;

use crate::{Archive, ArchivePointee, Archived, RelPtr};
use core::{
    borrow::Borrow,
    cmp::Ordering,
    fmt,
    hash::{Hash, Hasher},
    iter::FusedIterator,
    marker::PhantomData,
    ops::Index,
    ptr::NonNull,
};
use ptr_meta::Pointee;

#[cfg_attr(feature = "strict", repr(C))]
struct InnerNodeEntry<K> {
    ptr: RelPtr<NodeHeader>,
    key: K,
}

#[cfg_attr(feature = "strict", repr(C))]
struct LeafNodeEntry<K, V> {
    key: K,
    value: V,
}

impl<'a, UK: Archive, UV: Archive> Archive for LeafNodeEntry<&'a UK, &'a UV> {
    type Archived = LeafNodeEntry<UK::Archived, UV::Archived>;
    type Resolver = (UK::Resolver, UV::Resolver);

    #[inline]
    unsafe fn resolve(&self, pos: usize, resolver: Self::Resolver, out: *mut Self::Archived) {
        let (fp, fo) = out_field!(out.key);
        self.key.resolve(pos + fp, resolver.0, fo);
        let (fp, fo) = out_field!(out.value);
        self.value.resolve(pos + fp, resolver.1, fo);
    }
}

#[cfg_attr(feature = "strict", repr(C))]
struct NodeHeader {
    meta: Archived<u16>,
    size: Archived<usize>,
    // For leaf nodes, this points to the next leaf node in order
    // For inner nodes, this points to the node in the next layer that's less than the first key in
    // this node
    ptr: RelPtr<NodeHeader>,
}

impl NodeHeader {
    #[inline]
    fn is_inner(&self) -> bool {
        split_meta(from_archived!(self.meta)).0
    }

    #[inline]
    fn is_leaf(&self) -> bool {
        !split_meta(from_archived!(self.meta)).0
    }

    #[inline]
    fn len(&self) -> usize {
        split_meta(from_archived!(self.meta)).1
    }
}

#[inline]
#[cfg(feature = "alloc")]
fn combine_meta(is_inner: bool, len: usize) -> u16 {
    if is_inner {
        0x80_00 | len as u16
    } else {
        len as u16
    }
}

#[inline]
fn split_meta(meta: u16) -> (bool, usize) {
    (meta & 0x80_00 == 0x80_00, (meta & 0x7F_FF) as usize)
}

#[cfg_attr(feature = "strict", repr(C))]
struct Node<T: ?Sized> {
    header: NodeHeader,
    tail: T,
}

impl<T> Pointee for Node<[T]> {
    type Metadata = usize;
}

impl<T> ArchivePointee for Node<[T]> {
    type ArchivedMetadata = Archived<usize>;

    #[inline]
    fn pointer_metadata(archived: &Self::ArchivedMetadata) -> <Self as Pointee>::Metadata {
        from_archived!(*archived) as usize
    }
}

type InnerNode<K> = Node<[InnerNodeEntry<K>]>;
type LeafNode<K, V> = Node<[LeafNodeEntry<K, V>]>;

struct NodeHeaderData {
    meta: u16,
    size: usize,
    pos: Option<usize>,
}

impl Archive for NodeHeaderData {
    type Archived = NodeHeader;
    type Resolver = ();

    #[inline]
    unsafe fn resolve(&self, pos: usize, _: Self::Resolver, out: *mut Self::Archived) {
        let (fp, fo) = out_field!(out.meta);
        self.meta.resolve(pos + fp, (), fo);

        let (fp, fo) = out_field!(out.size);
        self.size.resolve(pos + fp, (), fo);

        let (fp, fo) = out_field!(out.ptr);
        RelPtr::emplace(pos + fp, self.pos.unwrap_or(pos + fp), fo);
    }
}

struct InnerNodeEntryData<'a, UK> {
    key: &'a UK,
}

impl<'a, UK: Archive> Archive for InnerNodeEntryData<'a, UK> {
    type Archived = InnerNodeEntry<UK::Archived>;
    type Resolver = (usize, UK::Resolver);

    #[inline]
    unsafe fn resolve(&self, pos: usize, resolver: Self::Resolver, out: *mut Self::Archived) {
        let (fp, fo) = out_field!(out.ptr);
        RelPtr::emplace(pos + fp, resolver.0, fo);
        let (fp, fo) = out_field!(out.key);
        self.key.resolve(pos + fp, resolver.1, fo);
    }
}

enum ClassifiedNode<'a, K, V> {
    Inner(&'a InnerNode<K>),
    Leaf(&'a LeafNode<K, V>),
}

impl NodeHeader {
    #[inline]
    fn classify<K, V>(&self) -> ClassifiedNode<'_, K, V> {
        if self.is_inner() {
            ClassifiedNode::Inner(self.classify_inner())
        } else {
            ClassifiedNode::Leaf(self.classify_leaf())
        }
    }

    #[inline]
    fn classify_inner_ptr<K>(&self) -> *const InnerNode<K> {
        ptr_meta::from_raw_parts(self as *const Self as *const (), self.len())
    }

    #[inline]
    fn classify_inner<K>(&self) -> &'_ InnerNode<K> {
        debug_assert!(self.is_inner());
        unsafe { &*self.classify_inner_ptr() }
    }

    #[inline]
    fn classify_leaf_ptr<K, V>(&self) -> *const LeafNode<K, V> {
        ptr_meta::from_raw_parts(self as *const Self as *const (), self.len())
    }

    #[inline]
    fn classify_leaf<K, V>(&self) -> &'_ LeafNode<K, V> {
        debug_assert!(self.is_leaf());
        unsafe { &*self.classify_leaf_ptr() }
    }
}

/// An archived [`BTreeMap`](std::collections::BTreeMap).
#[cfg_attr(feature = "strict", repr(C))]
pub struct ArchivedBTreeMap<K, V> {
    len: Archived<usize>,
    root: RelPtr<NodeHeader>,
    _phantom: PhantomData<(K, V)>,
}

/// The resolver for an [`ArchivedBTreeMap`].
pub struct BTreeMapResolver {
    root_pos: usize,
}

/// The minimum number of entries to place in a leaf node.
///
/// This value must be greater than 0
pub const MIN_ENTRIES_PER_LEAF_NODE: usize = 1;

/// The minimum number of entries to place in an inner node.
///
/// This value must be greater than 1
pub const MIN_ENTRIES_PER_INNER_NODE: usize = 2;

impl<K, V> ArchivedBTreeMap<K, V> {
    #[inline]
    fn root(&self) -> Option<ClassifiedNode<K, V>> {
        if self.is_empty() {
            None
        } else {
            let root = unsafe { &*self.root.as_ptr() };
            Some(root.classify())
        }
    }

    #[inline]
    fn first(&self) -> NonNull<NodeHeader> {
        if let Some(mut node) = self.root() {
            while let ClassifiedNode::Inner(inner) = node {
                let next = unsafe { &*inner.header.ptr.as_ptr() };
                node = next.classify();
            }
            match node {
                ClassifiedNode::Leaf(leaf) => unsafe {
                    let node = (leaf as *const LeafNode<K, V> as *mut LeafNode<K, V>).cast();
                    NonNull::new_unchecked(node)
                },
                ClassifiedNode::Inner(_) => unsafe { core::hint::unreachable_unchecked() },
            }
        } else {
            NonNull::dangling()
        }
    }

    /// Returns `true` if the map contains a value for the specified key.
    ///
    /// The key may be any borrowed form of the map's key type, but the ordering on the borrowed
    /// form _must_ match the ordering on the key type.
    #[inline]
    pub fn contains_key<Q: Ord + ?Sized>(&self, key: &Q) -> bool
    where
        K: Borrow<Q> + Ord,
    {
        self.get_key_value(key).is_some()
    }

    /// Returns a reference to the value corresponding to the key.
    ///
    /// The key may be any borrowed form of the map’s key type, but the ordering on the borrowed
    /// form must match the ordering on the key type.
    #[inline]
    pub fn get<Q: Ord + ?Sized>(&self, key: &Q) -> Option<&V>
    where
        K: Borrow<Q> + Ord,
    {
        self.get_key_value(key).map(|(_, v)| v)
    }

    /// Returns the key-value pair corresponding to the supplied key.
    ///
    /// The supplied key may be any borrowed form of the map’s key type, but the ordering on the
    /// borrowed form must match the ordering on the key type.
    pub fn get_key_value<Q: Ord + ?Sized>(&self, k: &Q) -> Option<(&K, &V)>
    where
        K: Borrow<Q> + Ord,
    {
        if let Some(mut current) = self.root() {
            loop {
                match current {
                    ClassifiedNode::Inner(node) => {
                        // Binary search for the next node layer
                        let next = match node
                            .tail
                            .binary_search_by(|probe| probe.key.borrow().cmp(k))
                        {
                            Ok(i) => unsafe { &*node.tail[i].ptr.as_ptr() },
                            Err(i) => {
                                if i == 0 {
                                    unsafe { &*node.header.ptr.as_ptr() }
                                } else {
                                    unsafe { &*node.tail[i - 1].ptr.as_ptr() }
                                }
                            }
                        };
                        current = next.classify();
                    }
                    ClassifiedNode::Leaf(node) => {
                        // Binary search for the value
                        if let Ok(i) = node
                            .tail
                            .binary_search_by(|probe| probe.key.borrow().cmp(k))
                        {
                            let entry = &node.tail[i];
                            break Some((&entry.key, &entry.value));
                        } else {
                            break None;
                        }
                    }
                }
            }
        } else {
            None
        }
    }

    /// Returns `true` if the map contains no elements.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Gets an iterator over the entries of the map, sorted by key.
    #[inline]
    pub fn iter(&self) -> Iter<'_, K, V> {
        Iter {
            inner: RawIter::new(self.first(), 0, self.len()),
        }
    }

    /// Gets an iterator over the keys of the map, in sorted order.
    #[inline]
    pub fn keys(&self) -> Keys<'_, K, V> {
        Keys {
            inner: RawIter::new(self.first(), 0, self.len()),
        }
    }

    /// Returns the number of items in the archived B-tree map.
    #[inline]
    pub fn len(&self) -> usize {
        from_archived!(self.len) as usize
    }

    /// Gets an iterator over the values of the map, in order by key.
    #[inline]
    pub fn values(&self) -> Values<'_, K, V> {
        Values {
            inner: RawIter::new(self.first(), 0, self.len()),
        }
    }

    /// Resolves a B-tree map from its length.
    ///
    /// # Safety
    ///
    /// - `len` must be the number of elements that were serialized
    /// - `pos` must be the position of `out` within the archive
    /// - `resolver` must be the result of serializing a B-tree map
    #[inline]
    pub unsafe fn resolve_from_len(
        len: usize,
        pos: usize,
        resolver: BTreeMapResolver,
        out: *mut Self,
    ) {
        let (fp, fo) = out_field!(out.len);
        len.resolve(pos + fp, (), fo);

        let (fp, fo) = out_field!(out.root);
        RelPtr::emplace(pos + fp, resolver.root_pos, fo);
    }
}

#[cfg(feature = "alloc")]
const _: () = {
    use crate::{ser::Serializer, Serialize};
    #[cfg(not(feature = "std"))]
    use alloc::vec::Vec;
    use core::mem;

    impl<K, V> ArchivedBTreeMap<K, V> {
        /// Serializes an ordered iterator of key-value pairs as a B-tree map.
        ///
        /// # Safety
        ///
        /// - Keys returned by the iterator must be unique
        /// - Keys must be in reverse sorted order from last to first
        pub unsafe fn serialize_from_reverse_iter<'a, UK, UV, S, I>(
            mut iter: I,
            serializer: &mut S,
        ) -> Result<BTreeMapResolver, S::Error>
        where
            UK: 'a + Serialize<S, Archived = K>,
            UV: 'a + Serialize<S, Archived = V>,
            S: Serializer + ?Sized,
            I: ExactSizeIterator<Item = (&'a UK, &'a UV)>,
        {
            if iter.len() == 0 {
                Ok(BTreeMapResolver { root_pos: 0 })
            } else {
                // The memory span of a single node should not exceed 4kb to keep everything within
                // the distance of a single IO page
                const MAX_NODE_SIZE: usize = 4096;

                // The nodes that must go in the next level in reverse order (key, node_pos)
                let mut next_level = Vec::new();
                let mut resolvers = Vec::new();

                while let Some((key, value)) = iter.next() {
                    // Start a new block
                    let block_start_pos = serializer.pos();

                    // Serialize the last entry
                    resolvers.push((
                        key,
                        value,
                        key.serialize(serializer)?,
                        value.serialize(serializer)?,
                    ));

                    loop {
                        // This is an estimate of the block size
                        // It's not exact because there may be padding to align the node and entries
                        // slice
                        let estimated_block_size = serializer.pos() - block_start_pos
                            + mem::size_of::<NodeHeader>()
                            + resolvers.len() * mem::size_of::<LeafNodeEntry<K, V>>();

                        // If we've reached or exceeded the maximum node size and have put enough
                        // entries in this node, then break
                        if estimated_block_size >= MAX_NODE_SIZE
                            && resolvers.len() >= MIN_ENTRIES_PER_LEAF_NODE
                        {
                            break;
                        }

                        if let Some((key, value)) = iter.next() {
                            // Serialize the next entry
                            resolvers.push((
                                key,
                                value,
                                key.serialize(serializer)?,
                                value.serialize(serializer)?,
                            ));
                        } else {
                            break;
                        }
                    }

                    // Finish the current node
                    serializer.align(usize::max(
                        mem::align_of::<NodeHeader>(),
                        mem::align_of::<LeafNodeEntry<K, V>>(),
                    ))?;
                    let raw_node = NodeHeaderData {
                        meta: combine_meta(false, resolvers.len()),
                        size: serializer.pos() - block_start_pos,
                        // The last element of next_level is the next block we're linked to
                        pos: next_level.last().map(|&(_, pos)| pos),
                    };

                    // Add the first key and node position to the next level
                    next_level.push((
                        resolvers.last().unwrap().0,
                        serializer.resolve_aligned(&raw_node, ())?,
                    ));

                    serializer.align_for::<LeafNodeEntry<K, V>>()?;
                    for (key, value, key_resolver, value_resolver) in resolvers.drain(..).rev() {
                        serializer.resolve_aligned(
                            &LeafNodeEntry { key, value },
                            (key_resolver, value_resolver),
                        )?;
                    }
                }

                // Subsequent levels are populated by serializing node keys from the previous level
                // When there's only one node left, that's our root
                let mut current_level = Vec::new();
                let mut resolvers = Vec::new();
                while next_level.len() > 1 {
                    // Our previous next_level becomes our current level, and current_level is
                    // guaranteed to be empty at this point
                    mem::swap(&mut current_level, &mut next_level);

                    let mut iter = current_level.drain(..);
                    while iter.len() > 1 {
                        // Start a new inner block
                        let block_start_pos = serializer.pos();

                        // When we break, we're guaranteed to have at least one node left
                        while iter.len() > 1 {
                            let (key, pos) = iter.next().unwrap();

                            // Serialize the next entry
                            resolvers.push((key, pos, key.serialize(serializer)?));

                            // Estimate the block size
                            let estimated_block_size = serializer.pos() - block_start_pos
                                + mem::size_of::<NodeHeader>()
                                + resolvers.len() * mem::size_of::<InnerNodeEntry<K>>();

                            // If we've reached or exceeded the maximum node size and have put enough
                            // keys in this node, then break
                            if estimated_block_size >= MAX_NODE_SIZE
                                && resolvers.len() >= MIN_ENTRIES_PER_INNER_NODE
                            {
                                break;
                            }
                        }

                        // Three cases here:
                        // 1 entry left: use it as the last key
                        // 2 entries left: serialize the next one and use the last as last to avoid
                        //   putting only one entry in the final block
                        // 3+ entries left: use next as last, next block will contain at least two
                        //   entries

                        if iter.len() == 2 {
                            let (key, pos) = iter.next().unwrap();

                            // Serialize the next entry
                            resolvers.push((key, pos, key.serialize(serializer)?));
                        }

                        // The next item is the first node
                        let (first_key, first_pos) = iter.next().unwrap();

                        // Finish the current node
                        serializer.align(usize::max(
                            mem::align_of::<NodeHeaderData>(),
                            mem::align_of::<InnerNodeEntry<K>>(),
                        ))?;
                        let node_header = NodeHeaderData {
                            meta: combine_meta(true, resolvers.len()),
                            size: serializer.pos() - block_start_pos,
                            // The pos of the first key is used to make the pointer for inner nodes
                            pos: Some(first_pos),
                        };

                        // Add the second key and node position to the next level
                        next_level.push((first_key, serializer.resolve_aligned(&node_header, ())?));

                        serializer.align_for::<InnerNodeEntry<K>>()?;
                        for (key, pos, resolver) in resolvers.drain(..).rev() {
                            let inner_node_data = InnerNodeEntryData::<UK> { key };
                            serializer.resolve_aligned(&inner_node_data, (pos, resolver))?;
                        }
                    }

                    debug_assert!(iter.len() == 0);
                }

                // The root is only node in the final level
                Ok(BTreeMapResolver {
                    root_pos: next_level[0].1,
                })
            }
        }
    }
};

impl<K: fmt::Debug, V: fmt::Debug> fmt::Debug for ArchivedBTreeMap<K, V> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_map().entries(self.iter()).finish()
    }
}

impl<K, Q, V> Index<&'_ Q> for ArchivedBTreeMap<K, V>
where
    K: Borrow<Q> + Ord,
    Q: Ord + ?Sized,
{
    type Output = V;

    fn index(&self, key: &Q) -> &V {
        self.get(key).unwrap()
    }
}

impl<'a, K, V> IntoIterator for &'a ArchivedBTreeMap<K, V> {
    type Item = (&'a K, &'a V);
    type IntoIter = Iter<'a, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<K: Eq, V: Eq> Eq for ArchivedBTreeMap<K, V> {}

impl<K: Hash, V: Hash> Hash for ArchivedBTreeMap<K, V> {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        for pair in self.iter() {
            pair.hash(state);
        }
    }
}

impl<K: Ord, V: Ord> Ord for ArchivedBTreeMap<K, V> {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        self.iter().cmp(other.iter())
    }
}

impl<K: PartialEq, V: PartialEq> PartialEq for ArchivedBTreeMap<K, V> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        if self.len() != other.len() {
            false
        } else {
            self.iter().zip(other.iter()).all(|(a, b)| a.eq(&b))
        }
    }
}

impl<K: PartialOrd, V: PartialOrd> PartialOrd for ArchivedBTreeMap<K, V> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.iter().partial_cmp(other.iter())
    }
}

// RawIter

struct RawIter<'a, K, V> {
    leaf: NonNull<NodeHeader>,
    index: usize,
    remaining: usize,
    _phantom: PhantomData<(&'a K, &'a V)>,
}

impl<'a, K, V> RawIter<'a, K, V> {
    fn new(leaf: NonNull<NodeHeader>, index: usize, remaining: usize) -> Self {
        Self {
            leaf,
            index,
            remaining,
            _phantom: PhantomData,
        }
    }
}

impl<'a, K, V> Iterator for RawIter<'a, K, V> {
    type Item = (&'a K, &'a V);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining == 0 {
            None
        } else {
            unsafe {
                // SAFETY: self.leaf is valid when self.remaining > 0
                // SAFETY: self.leaf always points to a leaf node header
                let leaf = self.leaf.as_ref().classify_leaf::<K, V>();
                if self.index == leaf.tail.len() {
                    self.index = 0;
                    // SAFETY: when self.remaining > 0 this is guaranteed to point to a leaf node
                    self.leaf = NonNull::new_unchecked(leaf.header.ptr.as_ptr() as *mut _);
                }
                let result = &self.leaf.as_ref().classify_leaf().tail[self.index];
                self.index += 1;
                self.remaining -= 1;
                Some((&result.key, &result.value))
            }
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.remaining, Some(self.remaining))
    }
}

impl<'a, K, V> ExactSizeIterator for RawIter<'a, K, V> {}
impl<'a, K, V> FusedIterator for RawIter<'a, K, V> {}

/// An iterator over the key-value pairs of an archived B-tree map.
pub struct Iter<'a, K, V> {
    inner: RawIter<'a, K, V>,
}

impl<'a, K, V> Iterator for Iter<'a, K, V> {
    type Item = (&'a K, &'a V);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

impl<'a, K, V> ExactSizeIterator for Iter<'a, K, V> {}
impl<'a, K, V> FusedIterator for Iter<'a, K, V> {}

/// An iterator over the keys of an archived B-tree map.
pub struct Keys<'a, K, V> {
    inner: RawIter<'a, K, V>,
}

impl<'a, K, V> Iterator for Keys<'a, K, V> {
    type Item = &'a K;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|(k, _)| k)
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

impl<'a, K, V> ExactSizeIterator for Keys<'a, K, V> {}
impl<'a, K, V> FusedIterator for Keys<'a, K, V> {}

/// An iterator over the values of an archived B-tree map.
pub struct Values<'a, K, V> {
    inner: RawIter<'a, K, V>,
}

impl<'a, K, V> Iterator for Values<'a, K, V> {
    type Item = &'a V;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|(_, v)| v)
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

impl<'a, K, V> ExactSizeIterator for Values<'a, K, V> {}
impl<'a, K, V> FusedIterator for Values<'a, K, V> {}
