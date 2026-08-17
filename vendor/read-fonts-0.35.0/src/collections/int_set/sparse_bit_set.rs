//! Provides serialization of [`IntSet`]'s to a highly compact bitset format as defined in the
//! IFT specification:
//!
//! <https://w3c.github.io/IFT/Overview.html#sparse-bit-set-decoding>

use std::collections::VecDeque;
use std::error::Error;
use std::fmt;

use super::bitset::U32SetBuilder;
use super::input_bit_stream::InputBitStream;
use super::output_bit_stream::OutputBitStream;
use super::IntSet;
use super::U32Set;

#[derive(Debug, PartialEq)]
pub struct DecodingError;

impl Error for DecodingError {}

impl fmt::Display for DecodingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "The input data stream was too short to be a valid sparse bit set."
        )
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub(crate) enum BranchFactor {
    Two,
    Four,
    Eight,
    ThirtyTwo,
}

impl IntSet<u32> {
    /// Populate this set with the values obtained from decoding the provided sparse bit set bytes.
    ///
    /// Sparse bit sets are a specialized, compact encoding of bit sets defined in the IFT specification:
    /// <https://w3c.github.io/IFT/Overview.html#sparse-bit-set-decoding>
    pub fn from_sparse_bit_set(data: &[u8]) -> Result<IntSet<u32>, DecodingError> {
        Self::from_sparse_bit_set_bounded(data, 0, u32::MAX).map(|(set, _)| set)
    }

    /// Populate this set with the values obtained from decoding the provided sparse bit set bytes.
    ///
    /// During decoding bias will be added to each decoded set members value. The final set will not contain
    /// any values larger than max_value: any encoded values larger than max_value after the bias is applied
    /// are ignored.
    ///
    /// Sparse bit sets are a specialized, compact encoding of bit sets defined in the IFT specification:
    /// <https://w3c.github.io/IFT/Overview.html#sparse-bit-set-decoding>
    pub fn from_sparse_bit_set_bounded(
        data: &[u8],
        bias: u32,
        max_value: u32,
    ) -> Result<(IntSet<u32>, &[u8]), DecodingError> {
        // This is a direct port of the decoding algorithm from:
        // <https://w3c.github.io/IFT/Overview.html#sparse-bit-set-decoding>
        let Some((branch_factor, height)) = InputBitStream::<0>::decode_header(data) else {
            return Err(DecodingError);
        };

        if height > branch_factor.max_height() {
            // TODO(garretrieger): the spec says nothing about this depth limit, we need to update the spec
            // to match.
            return Err(DecodingError);
        }

        let result = match branch_factor {
            BranchFactor::Two => {
                Self::decode_sparse_bit_set_nodes::<2>(data, height, bias, max_value)
            }
            BranchFactor::Four => {
                Self::decode_sparse_bit_set_nodes::<4>(data, height, bias, max_value)
            }
            BranchFactor::Eight => {
                Self::decode_sparse_bit_set_nodes::<8>(data, height, bias, max_value)
            }
            BranchFactor::ThirtyTwo => {
                Self::decode_sparse_bit_set_nodes::<32>(data, height, bias, max_value)
            }
        };

        result.map(|(bitset, data)| (IntSet::<u32>::from_bitset(bitset), data))
    }

    fn decode_sparse_bit_set_nodes<const BF: u8>(
        data: &[u8],
        height: u8,
        bias: u32,
        max_value: u32,
    ) -> Result<(U32Set, &[u8]), DecodingError> {
        let mut out = U32Set::empty();
        if height == 0 {
            // 1 byte was used for the header.
            return Ok((out, &data[1..]));
        }

        let mut builder = U32SetBuilder::start(&mut out);
        let mut bits = InputBitStream::<BF>::from(data);
        // TODO(garretrieger): estimate initial capacity (maximum is a function of the number of nodes in the bit stream).
        let mut queue = VecDeque::<NextNode>::new();
        queue.push_back(NextNode { start: 0, depth: 1 });

        'outer: while let Some(next) = queue.pop_front() {
            let mut bits = bits.next().ok_or(DecodingError)?;
            if bits == 0 {
                // all bits were zeroes which is a special command to completely fill in
                // all integers covered by this node.
                let exp = (height as u32) - next.depth + 1;
                let node_size = (BF as u64).pow(exp);

                let Some(start) = u32::try_from(next.start)
                    .ok()
                    .and_then(|start| start.checked_add(bias))
                    .filter(|start| *start <= max_value)
                else {
                    // start is outside the valid range of the set, so skip this range.
                    continue;
                };

                let end = u32::try_from(next.start + node_size - 1)
                    .unwrap_or(u32::MAX)
                    .saturating_add(bias)
                    .min(max_value);

                // TODO(garretrieger): implement special insert_range on the builder as well.
                builder.set.insert_range(start..=end);
                continue;
            }

            let height = height as u32;

            let exp = height - next.depth;
            let next_node_size = (BF as u64).pow(exp);
            loop {
                let bit_index = bits.trailing_zeros();
                if bit_index == 32 {
                    break;
                }

                // TODO(garretrieger): possible optimization by having two versions of this loop
                //                     as next.depth == height has the same value for each of the outer iterations.
                if next.depth == height {
                    // TODO(garretrieger): this has a few branches, is it faster to do all the math in u64
                    //                     then check only once for > max_value? Will need to check with a benchmark.
                    let Some(start) = u32::try_from(next.start)
                        .ok()
                        .and_then(|start| start.checked_add(bit_index))
                        .and_then(|start| start.checked_add(bias))
                        .filter(|start| *start <= max_value)
                    else {
                        // At the lowest depth values are encountered in order, so if this is out of range so will be
                        // all future values. We can break early.
                        break 'outer;
                    };

                    // TODO(garretrieger): further optimize by inserting entire nodes at once (as a bit field).
                    builder.insert(start);
                } else {
                    let start_delta = bit_index as u64 * next_node_size;
                    queue.push_back(NextNode {
                        start: next.start + start_delta,
                        depth: next.depth + 1,
                    });
                }

                bits &= !(1 << bit_index); // clear the bit that was just read.
            }
        }

        builder.finish();

        // If the max value was reached the loop above may have terminated early leaving some unprocessed nodes
        // in the queue. The loop can only break once we are at the lowest depth which means that each remaining queue node
        // will consume only one node from the bit stream. Advance the bit stream by the remaining number of nodes to
        // correctly count the number of bytes consumed.
        if !bits.skip_nodes(queue.len() as u32) {
            // We ran out of bits to consume before decoding would have been finished.
            return Err(DecodingError);
        }

        Ok((out, &data[bits.bytes_consumed()..]))
    }

    /// Encode this set as a sparse bit set byte encoding.
    ///
    /// Sparse bit sets are a specialized, compact encoding of bit sets defined in the IFT specification:
    /// <https://w3c.github.io/IFT/Overview.html#sparse-bit-set-decoding>
    pub fn to_sparse_bit_set(&self) -> Vec<u8> {
        // TODO(garretrieger): use the heuristic approach from the incxfer
        // implementation to guess the optimal size. Building the set 4 times
        // is costly.
        let mut candidates: Vec<Vec<u8>> = vec![];

        let Some(max_value) = self.last() else {
            return OutputBitStream::new(BranchFactor::Two, 0).into_bytes();
        };

        if BranchFactor::Two.tree_height_for(max_value) <= BranchFactor::Two.max_height() {
            candidates.push(to_sparse_bit_set_with_bf::<2>(self));
        }

        if BranchFactor::Four.tree_height_for(max_value) <= BranchFactor::Four.max_height() {
            candidates.push(to_sparse_bit_set_with_bf::<4>(self));
        }

        if BranchFactor::Eight.tree_height_for(max_value) <= BranchFactor::Eight.max_height() {
            candidates.push(to_sparse_bit_set_with_bf::<8>(self));
        }

        if BranchFactor::ThirtyTwo.tree_height_for(max_value)
            <= BranchFactor::ThirtyTwo.max_height()
        {
            candidates.push(to_sparse_bit_set_with_bf::<32>(self));
        }

        candidates.into_iter().min_by_key(|f| f.len()).unwrap()
    }
}

/// Encode this set as a sparse bit set byte encoding with a specified branch factor.
///
/// Branch factor can be 2, 4, 8 or 32. It's a compile time constant so that optimized decoding implementations
/// can be generated by the compiler.
///
/// Sparse bit sets are a specialized, compact encoding of bit sets defined in the IFT specification:
/// <https://w3c.github.io/IFT/Overview.html#sparse-bit-set-decoding>
pub fn to_sparse_bit_set_with_bf<const BF: u8>(set: &IntSet<u32>) -> Vec<u8> {
    let branch_factor = BranchFactor::from_val(BF);
    let Some(max_value) = set.last() else {
        return OutputBitStream::new(branch_factor, 0).into_bytes();
    };
    let mut height = branch_factor.tree_height_for(max_value);
    if height > branch_factor.max_height() {
        if BF == 2 {
            // Branch factor 2 cannot encode all possible u32 values, so upgrade to a BF4 set in that case.
            return to_sparse_bit_set_with_bf::<4>(set);
        }
        // This shouldn't be reachable for any possible u32 values.
        panic!("Height value exceeds the maximum for this branch factor.");
    }
    let mut os = OutputBitStream::new(branch_factor, height);
    let mut nodes: Vec<Node> = vec![];

    // We build the nodes that will comprise the bit stream in reverse order
    // from the last value in the last layer up to the first layer. Then
    // when generating the final stream the order is reversed.
    // The reverse order construction is needed since nodes at the lower layer
    // affect the values in the parent layers.
    let mut indices = set.clone();
    let mut filled_indices = IntSet::<u32>::all();
    while height > 0 {
        (indices, filled_indices) =
            create_layer(branch_factor, indices, filled_indices, &mut nodes);
        height -= 1;
    }

    for node in nodes.iter().rev() {
        match node.node_type {
            NodeType::Standard => os.write_node(node.bits),
            NodeType::Filled => os.write_node(0),
            NodeType::Skip => {}
        };
    }

    os.into_bytes()
}

struct CreateLayerState<'a> {
    // This is the set of indices which are to be set in the layer above this one
    upper_indices: IntSet<u32>,
    // Similarly, this is the set of indices in the layer above this one which are fully filled.
    upper_filled_indices: IntSet<u32>,

    current_node: Option<Node>,
    current_node_filled_bits: u32,
    nodes: &'a mut Vec<Node>,
    child_count: u64,
    nodes_init_length: u64,
    branch_factor: BranchFactor,
}

impl CreateLayerState<'_> {
    fn commit_current_node(&mut self) {
        let Some(mut node) = self.current_node.take() else {
            // noop if there isn't a node to commit.
            return;
        };
        self.upper_indices.insert(node.parent_index);

        if self.current_node_filled_bits == self.branch_factor.u32_mask() {
            // This node is filled and can thus be represented by a node that is '0'.
            // It's index is recorded so that the parent node can also check if they are filled.
            self.upper_filled_indices.insert(node.parent_index);
            node.node_type = NodeType::Filled;

            if self.nodes_init_length >= self.child_count {
                // Since this node is filled, find all nodes which are children and set them to be skipped in
                // the encoding.
                let children_start_index = self.nodes_init_length.saturating_sub(self.child_count);
                let children_end_index = self.nodes_init_length;
                // TODO(garretrieger): this scans all nodes of the previous layer to find those which are children,
                //   but we can likely limit it to just the children of this node with some extra book keeping.
                for child in
                    &mut self.nodes[children_start_index as usize..children_end_index as usize]
                {
                    if child.parent_index >= node.parent_index * self.branch_factor.value()
                        && child.parent_index < (node.parent_index + 1) * self.branch_factor.value()
                    {
                        child.node_type = NodeType::Skip;
                    }
                }
            }
        }

        self.nodes.push(node);
        self.current_node_filled_bits = 0;
    }
}

/// Compute the nodes for a layer of the sparse bit set.
///
/// Computes the nodes needed for the layer which contains the indices in
/// 'iter'. The new nodes are appended to 'nodes'. 'iter' must be sorted
/// in ascending order.
///
/// Returns the set of indices for the layer above.
fn create_layer(
    branch_factor: BranchFactor,
    values: IntSet<u32>,
    filled_values: IntSet<u32>,
    nodes: &mut Vec<Node>,
) -> (IntSet<u32>, IntSet<u32>) {
    let mut state = CreateLayerState {
        upper_indices: IntSet::<u32>::empty(),
        upper_filled_indices: IntSet::<u32>::empty(),
        current_node: None,
        current_node_filled_bits: 0,
        child_count: values.len(),
        nodes_init_length: nodes.len() as u64,
        nodes,
        branch_factor,
    };

    // The nodes array is produced in reverse order and then reversed before final output.
    for v in values.iter().rev() {
        let parent_index = v / branch_factor.value();
        let prev_parent_index = state
            .current_node
            .as_ref()
            .map_or(parent_index, |node| node.parent_index);
        if prev_parent_index != parent_index {
            state.commit_current_node();
        }

        let current_node = state.current_node.get_or_insert(Node {
            bits: 0,
            parent_index,
            node_type: NodeType::Standard,
        });

        let mask = 0b1 << (v % branch_factor.value());
        current_node.bits |= mask;
        if filled_values.contains(v) {
            state.current_node_filled_bits |= mask;
        }
    }

    state.commit_current_node();
    (state.upper_indices, state.upper_filled_indices)
}

enum NodeType {
    Standard,
    Filled,
    Skip,
}

struct Node {
    bits: u32,
    parent_index: u32,
    node_type: NodeType,
}

impl BranchFactor {
    pub(crate) fn value(&self) -> u32 {
        match self {
            BranchFactor::Two => 2,
            BranchFactor::Four => 4,
            BranchFactor::Eight => 8,
            BranchFactor::ThirtyTwo => 32,
        }
    }

    /// The maximum height that can be used for a given branch factor without the risk of encountering overflows
    pub(crate) fn max_height(&self) -> u8 {
        match self {
            BranchFactor::Two => 31,
            BranchFactor::Four => 16,
            BranchFactor::Eight => 11,
            BranchFactor::ThirtyTwo => 7,
        }
    }

    fn tree_height_for(&self, max_value: u32) -> u8 {
        // height H, can represent up to (BF^height) - 1
        let mut height: u32 = 0;
        let mut max_value = max_value;
        loop {
            height += 1;
            max_value >>= self.node_size_log2();
            if max_value == 0 {
                break height as u8;
            }
        }
    }

    fn from_val(val: u8) -> BranchFactor {
        match val {
            2 => BranchFactor::Two,
            4 => BranchFactor::Four,
            8 => BranchFactor::Eight,
            32 => BranchFactor::ThirtyTwo,
            // This should never happen as this is only used internally.
            _ => panic!("Invalid branch factor."),
        }
    }

    fn node_size_log2(&self) -> u32 {
        match self {
            BranchFactor::Two => 1,
            BranchFactor::Four => 2,
            BranchFactor::Eight => 3,
            BranchFactor::ThirtyTwo => 5,
        }
    }

    pub(crate) fn byte_mask(&self) -> u32 {
        match self {
            BranchFactor::Two => 0b00000011,
            BranchFactor::Four => 0b00001111,
            BranchFactor::Eight => 0b11111111,
            BranchFactor::ThirtyTwo => 0b11111111,
        }
    }

    fn u32_mask(&self) -> u32 {
        match self {
            BranchFactor::Two => 0b00000000_00000000_00000000_00000011,
            BranchFactor::Four => 0b00000000_00000000_00000000_00001111,
            BranchFactor::Eight => 0b00000000_00000000_00000000_11111111,
            BranchFactor::ThirtyTwo => 0b11111111_11111111_11111111_11111111,
        }
    }
}

struct NextNode {
    start: u64,
    depth: u32,
}

#[cfg(test)]
#[allow(clippy::unusual_byte_groupings)]
mod test {
    use super::*;

    #[test]
    fn spec_example_2() {
        // Test of decoding the example 2 given in the specification.
        // See: <https://w3c.github.io/IFT/Overview.html#sparse-bit-set-decoding>
        let bytes = [
            0b00001110, 0b00100001, 0b00010001, 0b00000001, 0b00000100, 0b00000010, 0b00001000,
        ];

        let set = IntSet::<u32>::from_sparse_bit_set(&bytes).unwrap();
        let expected: IntSet<u32> = [2, 33, 323].iter().copied().collect();
        assert_eq!(set, expected);
    }

    #[test]
    fn spec_example_3() {
        // Test of decoding the example 3 given in the specification.
        // See: <https://w3c.github.io/IFT/Overview.html#sparse-bit-set-decoding>
        let bytes = [0b00000000];

        let set = IntSet::<u32>::from_sparse_bit_set(&bytes).unwrap();
        let expected: IntSet<u32> = [].iter().copied().collect();
        assert_eq!(set, expected);
    }

    #[test]
    fn spec_example_4() {
        // Test of decoding the example 4 given in the specification.
        // See: <https://w3c.github.io/IFT/Overview.html#sparse-bit-set-decoding>
        let bytes = [0b00001101, 0b00000011, 0b00110001];

        let set = IntSet::<u32>::from_sparse_bit_set(&bytes).unwrap();

        let mut expected: IntSet<u32> = IntSet::<u32>::empty();
        expected.insert_range(0..=17);

        assert_eq!(set, expected);
    }

    #[test]
    fn invalid() {
        // Spec example 2 with one byte missing.
        let bytes = [
            0b00001110, 0b00100001, 0b00010001, 0b00000001, 0b00000100, 0b00000010,
        ];
        assert!(IntSet::<u32>::from_sparse_bit_set(&bytes).is_err());

        // Max height exceeded.
        let bytes = [
            0b0_01000_11, // BF 32, Depth 8
            0b00000000,
            0b00000000,
            0b00000000,
            0b10000000, // L1
            0b00000000,
            0b00000000,
            0b00000000,
            0b10000000, // L2
            0b00000000,
            0b00000000,
            0b00000000,
            0b10000000, // L3
            0b00000000,
            0b00000000,
            0b00000000,
            0b10000000, // L4
            0b00000000,
            0b00000000,
            0b00000000,
            0b10000000, // L5
            0b00000000,
            0b00000000,
            0b00000000,
            0b10000000, // L6
            0b00000000,
            0b00000000,
            0b00000000,
            0b00000001, // L7
            0b00000000,
            0b00000000,
            0b00000000,
            0b10000000, // L8
        ];
        assert!(IntSet::<u32>::from_sparse_bit_set(&bytes).is_err());
    }

    #[test]
    fn invalid_biased_and_bounded() {
        let bytes = [0b0_00011_01, 0b0000_0011, 0b1111_0011];

        assert!(IntSet::<u32>::from_sparse_bit_set_bounded(&bytes, 0, u32::MAX).is_err());
        assert!(IntSet::<u32>::from_sparse_bit_set_bounded(&bytes, 0, 20).is_err());
        assert!(IntSet::<u32>::from_sparse_bit_set_bounded(&bytes, 0, 19).is_err());
        assert!(IntSet::<u32>::from_sparse_bit_set_bounded(&bytes, 0, 18).is_err());
        assert!(IntSet::<u32>::from_sparse_bit_set_bounded(&bytes, 0, 15).is_err());
        assert!(IntSet::<u32>::from_sparse_bit_set_bounded(&bytes, 0, 14).is_err());

        assert!(IntSet::<u32>::from_sparse_bit_set_bounded(&bytes, 1, 20).is_err());
        assert!(IntSet::<u32>::from_sparse_bit_set_bounded(&bytes, 2, 20).is_err());
        assert!(IntSet::<u32>::from_sparse_bit_set_bounded(&bytes, 3, 20).is_err());
        assert!(IntSet::<u32>::from_sparse_bit_set_bounded(&bytes, 6, 20).is_err());
    }

    #[test]
    fn larger_than_u32() {
        // Set with values beyond u32
        let bytes = [
            0b0_00111_11, // BF 32, Depth 7
            0b00000000,
            0b00000000,
            0b00000000,
            0b10000000, // L1
            0b00000000,
            0b00000000,
            0b00000000,
            0b10000000, // L2
            0b00000000,
            0b00000000,
            0b00000000,
            0b10000000, // L3
            0b00000000,
            0b00000000,
            0b00000000,
            0b10000000, // L4
            0b00000000,
            0b00000000,
            0b00000000,
            0b10000000, // L5
            0b00000000,
            0b00000000,
            0b00000000,
            0b10000000, // L6
            0b00000000,
            0b00000000,
            0b00000000,
            0b00000001, // L7
        ];
        assert_eq!(
            IntSet::<u32>::from_sparse_bit_set(&bytes).unwrap(),
            IntSet::<u32>::empty()
        );

        // Set with filled node values beyond u32
        let bytes = [
            0b0_00111_11, // BF 32, Depth 7
            0b00000000,
            0b00000000,
            0b00000000,
            0b10000000, // L1
            0b00000000,
            0b00000000,
            0b00000000,
            0b00000000, // L2
        ];

        assert_eq!(
            IntSet::<u32>::from_sparse_bit_set(&bytes).unwrap(),
            IntSet::<u32>::empty()
        );
    }

    #[test]
    fn from_sparse_bit_set_bounded_with_remaining_data() {
        let bytes = [0b00001101, 0b00000011, 0b00110001, 0b10101010];
        let mut expected: IntSet<u32> = IntSet::<u32>::empty();
        expected.insert_range(0..=17);

        assert_eq!(
            IntSet::<u32>::from_sparse_bit_set_bounded(&bytes, 0, 19).unwrap(),
            (expected.clone(), &bytes[3..]),
        );
    }

    #[test]
    fn from_sparse_bit_set_biased_and_bounded() {
        let bytes = [0b0_00011_01, 0b0000_0011, 0b1111_0011, 0b0000_0001];
        let mut expected: IntSet<u32> = IntSet::<u32>::empty();
        expected.insert_range(0..=20);

        assert_eq!(
            IntSet::<u32>::from_sparse_bit_set_bounded(&bytes, 0, 20).unwrap(),
            (expected.clone(), &bytes[4..])
        );

        let mut expected: IntSet<u32> = IntSet::<u32>::empty();
        expected.insert_range(0..=19);
        assert_eq!(
            IntSet::<u32>::from_sparse_bit_set_bounded(&bytes, 0, 19).unwrap(),
            (expected.clone(), &bytes[4..])
        );

        let mut expected: IntSet<u32> = IntSet::<u32>::empty();
        expected.insert_range(1..=20);
        assert_eq!(
            IntSet::<u32>::from_sparse_bit_set_bounded(&bytes, 1, 20).unwrap(),
            (expected.clone(), &bytes[4..])
        );

        let mut expected: IntSet<u32> = IntSet::<u32>::empty();
        expected.insert_range(1..=18);
        assert_eq!(
            IntSet::<u32>::from_sparse_bit_set_bounded(&bytes, 1, 18).unwrap(),
            (expected.clone(), &bytes[4..])
        );

        let mut expected: IntSet<u32> = IntSet::<u32>::empty();
        expected.insert_range(0..=14);
        assert_eq!(
            IntSet::<u32>::from_sparse_bit_set_bounded(&bytes, 0, 14).unwrap(),
            (expected.clone(), &bytes[4..])
        );

        let mut expected: IntSet<u32> = IntSet::<u32>::empty();
        expected.insert_range(6..=20);
        assert_eq!(
            IntSet::<u32>::from_sparse_bit_set_bounded(&bytes, 6, 20).unwrap(),
            (expected.clone(), &bytes[4..])
        );

        let mut expected: IntSet<u32> = IntSet::<u32>::empty();
        expected.insert(0);
        assert_eq!(
            IntSet::<u32>::from_sparse_bit_set_bounded(&bytes, 0, 0).unwrap(),
            (expected.clone(), &bytes[4..])
        );

        assert_eq!(
            IntSet::<u32>::from_sparse_bit_set_bounded(&bytes, 1, 0).unwrap(),
            (IntSet::<u32>::empty().clone(), &bytes[4..])
        );

        let bytes = [0b00000000];
        let set = IntSet::<u32>::from_sparse_bit_set_bounded(&bytes, 5, 0)
            .unwrap()
            .0;
        assert_eq!(set, IntSet::<u32>::empty());
    }

    #[test]
    fn test_tree_height_for() {
        assert_eq!(BranchFactor::Two.tree_height_for(0), 1);
        assert_eq!(BranchFactor::Two.tree_height_for(1), 1);
        assert_eq!(BranchFactor::Two.tree_height_for(2), 2);
        assert_eq!(BranchFactor::Two.tree_height_for(117), 7);

        assert_eq!(BranchFactor::Four.tree_height_for(0), 1);
        assert_eq!(BranchFactor::Four.tree_height_for(3), 1);
        assert_eq!(BranchFactor::Four.tree_height_for(4), 2);
        assert_eq!(BranchFactor::Four.tree_height_for(63), 3);
        assert_eq!(BranchFactor::Four.tree_height_for(64), 4);

        assert_eq!(BranchFactor::Eight.tree_height_for(0), 1);
        assert_eq!(BranchFactor::Eight.tree_height_for(7), 1);
        assert_eq!(BranchFactor::Eight.tree_height_for(8), 2);
        assert_eq!(BranchFactor::Eight.tree_height_for(32767), 5);
        assert_eq!(BranchFactor::Eight.tree_height_for(32768), 6);

        assert_eq!(BranchFactor::ThirtyTwo.tree_height_for(0), 1);
        assert_eq!(BranchFactor::ThirtyTwo.tree_height_for(31), 1);
        assert_eq!(BranchFactor::ThirtyTwo.tree_height_for(32), 2);
        assert_eq!(BranchFactor::ThirtyTwo.tree_height_for(1_048_575), 4);
        assert_eq!(BranchFactor::ThirtyTwo.tree_height_for(1_048_576), 5);
    }

    #[test]
    fn generate_spec_example_2() {
        // Test of reproducing the encoding of example 2 given
        // in the specification. See:
        // <https://w3c.github.io/IFT/Overview.html#sparse-bit-set-decoding>

        let actual_bytes = to_sparse_bit_set_with_bf::<8>(&[2, 33, 323].iter().copied().collect());
        let expected_bytes = [
            0b00001110, 0b00100001, 0b00010001, 0b00000001, 0b00000100, 0b00000010, 0b00001000,
        ];

        assert_eq!(actual_bytes, expected_bytes);
    }

    #[test]
    fn generate_spec_example_3() {
        // Test of reproducing the encoding of example 3 given
        // in the specification. See:
        // <https://w3c.github.io/IFT/Overview.html#sparse-bit-set-decoding>

        let actual_bytes = to_sparse_bit_set_with_bf::<2>(&IntSet::<u32>::empty());
        let expected_bytes = [0b00000000];

        assert_eq!(actual_bytes, expected_bytes);
    }

    #[test]
    fn generate_spec_example_4() {
        // Test of reproducing the encoding of example 3 given
        // in the specification. See:
        // <https://w3c.github.io/IFT/Overview.html#sparse-bit-set-decoding>

        let actual_bytes = to_sparse_bit_set_with_bf::<4>(&(0..=17).collect());
        let expected_bytes = [0b00001101, 0b0000_0011, 0b0011_0001];

        assert_eq!(actual_bytes, expected_bytes);
    }

    #[test]
    fn encode_one_level() {
        let actual_bytes = to_sparse_bit_set_with_bf::<8>(&[2, 6].iter().copied().collect());
        let expected_bytes = [0b0_00001_10, 0b01000100];
        assert_eq!(actual_bytes, expected_bytes);
    }

    #[test]
    fn encode_one_level_filled() {
        let actual_bytes = to_sparse_bit_set_with_bf::<8>(&(0..=7).collect());
        let expected_bytes = [0b0_00001_10, 0b00000000];
        assert_eq!(actual_bytes, expected_bytes);
    }

    #[test]
    fn encode_two_level_filled() {
        let actual_bytes = to_sparse_bit_set_with_bf::<8>(&(3..=21).collect());
        let expected_bytes = [0b0_00010_10, 0b00000111, 0b11111000, 0b00000000, 0b00111111];
        assert_eq!(actual_bytes, expected_bytes);
    }

    #[test]
    fn encode_two_level_not_filled() {
        let actual_bytes = to_sparse_bit_set_with_bf::<4>(&[0, 4, 8, 12].iter().copied().collect());
        let expected_bytes = [0b0_00010_01, 0b0001_1111, 0b0001_0001, 0b0000_0001];
        assert_eq!(actual_bytes, expected_bytes);
    }

    #[test]
    fn encode_four_level_filled() {
        let mut s = IntSet::<u32>::empty();
        s.insert_range(64..=127); // Filled node on level 3
        s.insert_range(512..=1023); // Filled node on level 2
        s.insert(4000);

        let actual_bytes = to_sparse_bit_set_with_bf::<8>(&s);
        let expected_bytes = [
            // Header
            0b0_00100_10,
            // L1
            0b10000011,
            // L2
            0b00000010,
            0b00000000,
            0b01000000,
            // L3,
            0b00000000,
            0b00010000,
            // L4
            0b00000001,
        ];
        assert_eq!(actual_bytes, expected_bytes);
    }

    #[test]
    fn encode_bf32() {
        let actual_bytes = to_sparse_bit_set_with_bf::<32>(&[2, 31, 323].iter().copied().collect());
        let expected_bytes = [
            0b0_00010_11,
            // node 0
            0b00000001,
            0b00000100,
            0b00000000,
            0b00000000,
            // node 1
            0b00000100,
            0b00000000,
            0b00000000,
            0b10000000,
            // node 2
            0b00001000,
            0b00000000,
            0b00000000,
            0b00000000,
        ];

        assert_eq!(actual_bytes, expected_bytes);
    }

    #[test]
    fn round_trip() {
        let s1: IntSet<u32> = [11, 74, 9358].iter().copied().collect();
        let mut s2: IntSet<u32> = s1.clone();
        s2.insert_range(67..=412);

        check_round_trip::<2>(&s1);
        check_round_trip::<4>(&s1);
        check_round_trip::<8>(&s1);
        check_round_trip::<32>(&s1);

        check_round_trip::<2>(&s2);
        check_round_trip::<4>(&s2);
        check_round_trip::<8>(&s2);
        check_round_trip::<32>(&s2);
    }

    fn check_round_trip<const BF: u8>(s: &IntSet<u32>) {
        let bytes = to_sparse_bit_set_with_bf::<BF>(s);
        let s_prime = IntSet::<u32>::from_sparse_bit_set(&bytes).unwrap();
        assert_eq!(*s, s_prime);
    }

    #[test]
    fn find_smallest_bf() {
        let s: IntSet<u32> = [11, 74, 9358].iter().copied().collect();
        let bytes = s.to_sparse_bit_set();
        // BF4
        assert_eq!(vec![0b0_00111_01], bytes[0..1]);

        let s: IntSet<u32> = [
            16, 0, 2, 4, 6, 8, 10, 12, 14, 16, 18, 20, 22, 24, 26, 28, 30,
        ]
        .iter()
        .copied()
        .collect();
        let bytes = s.to_sparse_bit_set();
        // BF32
        assert_eq!(vec![0b0_00001_11], bytes[0..1]);
    }

    #[test]
    fn encode_maxu32() {
        let s: IntSet<u32> = [1, u32::MAX].iter().copied().collect();

        let bytes = s.to_sparse_bit_set();
        let s_prime = IntSet::<u32>::from_sparse_bit_set(&bytes);
        assert_eq!(s, s_prime.unwrap());

        let s: IntSet<u32> = [1, u32::MAX].iter().copied().collect();
        let bytes = to_sparse_bit_set_with_bf::<2>(&s);
        let s_prime = IntSet::<u32>::from_sparse_bit_set(&bytes);
        assert_eq!(s, s_prime.unwrap());

        let s: IntSet<u32> = [1, u32::MAX].iter().copied().collect();
        let bytes = to_sparse_bit_set_with_bf::<4>(&s);
        let s_prime = IntSet::<u32>::from_sparse_bit_set(&bytes);
        assert_eq!(s, s_prime.unwrap());

        let s: IntSet<u32> = [1, u32::MAX].iter().copied().collect();
        let bytes = to_sparse_bit_set_with_bf::<8>(&s);
        let s_prime = IntSet::<u32>::from_sparse_bit_set(&bytes);
        assert_eq!(s, s_prime.unwrap());

        let s: IntSet<u32> = [1, u32::MAX].iter().copied().collect();
        let bytes = to_sparse_bit_set_with_bf::<32>(&s);
        let s_prime = IntSet::<u32>::from_sparse_bit_set(&bytes);
        assert_eq!(s, s_prime.unwrap());
    }
}
