export function new_noop_readable_stream() {
    return new ReadableStream();
}

export function new_noop_readable_byte_stream() {
    return new ReadableStream({
        type: 'bytes',
        start(controller) {
            this.controller = controller;
        },
        cancel() {
            const byobRequest = this.controller.byobRequest;
            if (byobRequest) {
                byobRequest.respond(0);
            }
        }
    });
}

export function new_readable_stream_from_array(chunks) {
    return new ReadableStream({
        start(controller) {
            for (let chunk of chunks) {
                controller.enqueue(chunk);
            }
            controller.close();
        }
    });
}

export function new_readable_byte_stream_from_array(chunks) {
    return new ReadableStream({
        type: 'bytes',
        start(controller) {
            this.controller = controller;
            for (let chunk of chunks) {
                controller.enqueue(chunk);
            }
            controller.close();
        },
        cancel() {
            const byobRequest = this.controller.byobRequest;
            if (byobRequest) {
                byobRequest.respond(0);
            }
        }
    });
}

export function new_readable_stream_with_rejecting_cancel() {
    return new ReadableStream({
        cancel(reason) {
            return Promise.reject('error from cancel');
        }
    });
}

export function new_readable_byte_stream_with_rejecting_cancel() {
    return new ReadableStream({
        type: 'bytes',
        cancel(reason) {
            return Promise.reject('error from cancel');
        }
    });
}

/**
 * Tests whether `reader.releaseLock()` is allowed while there are pending read requests.
 *
 * See: https://github.com/whatwg/streams/commit/d5f92d9f17306d31ba6b27424d23d58e89bf64a5
 */
export function supports_release_lock_with_pending_read() {
    try {
        const reader = new ReadableStream().getReader();
        reader.read().then(() => {}, () => {});
        reader.releaseLock();
        return true;
    } catch {
        return false;
    }
}
