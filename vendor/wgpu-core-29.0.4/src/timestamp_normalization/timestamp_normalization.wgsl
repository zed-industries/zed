// Must have "common.wgsl" preprocessed before this file's contents.
//
// To compile this locally, you can run:
// ```
// cat common.wgsl timestamp_normalization.wgsl | cargo run -p naga-cli -- --stdin-file-path timestamp_normalization.wgsl
// ```

// For an explanation of the timestamp normalization process, see
// the `mod.rs` file in this folder.

// These is the timestamp period turned into a fraction
// with an integer numerator and denominator. The denominator
// is a power of two, so the division can be done with a shift.
override TIMESTAMP_PERIOD_MULTIPLY: u32 = 1;
override TIMESTAMP_PERIOD_SHIFT: u32 = 0;

@group(0) @binding(0)
var<storage, read_write> timestamps: array<Uint64>;

struct ImmediateData {
    timestamp_offset: u32,
    timestamp_count: u32,
}

var<immediate> im: ImmediateData;

@compute @workgroup_size(64)
fn main(@builtin(global_invocation_id) id: vec3u) {
    if id.x >= im.timestamp_count {
        return;
    }

    let index = id.x + im.timestamp_offset;

    let input_value = timestamps[index];

    let tmp1 = u64_mul_u32(input_value, TIMESTAMP_PERIOD_MULTIPLY);
    let tmp2 = shift_right_96(tmp1, TIMESTAMP_PERIOD_SHIFT);

    timestamps[index] = truncate_u96_to_u64(tmp2);
}
