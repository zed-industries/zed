export function new_noop_transform_stream() {
    return new TransformStream();
}

export function new_uppercase_transform_stream() {
    return new TransformStream({
        transform(chunk, controller) {
            controller.enqueue(chunk.toUpperCase());
        }
    });
}
