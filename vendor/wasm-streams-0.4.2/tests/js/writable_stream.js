export function new_noop_writable_stream() {
    return new WritableStream();
}

const TYPE_WRITE = 0;
const TYPE_CLOSE = 1;
const TYPE_ABORT = 2;

export function new_recording_writable_stream() {
    const events = [];
    const stream = new WritableStream({
        write(chunk) {
            events.push({type: TYPE_WRITE, chunk});
        },
        close() {
            events.push({type: TYPE_CLOSE});
        },
        abort(reason) {
            events.push({type: TYPE_ABORT, reason});
        }
    });
    return {stream, events};
}
