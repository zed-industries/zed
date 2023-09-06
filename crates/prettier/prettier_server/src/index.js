const { Buffer } = require('buffer');

let buffer = Buffer.alloc(0);
process.stdin.resume();
process.stdin.on('data', (data) => {
    buffer = Buffer.concat([buffer, data]);
    handleData();
});
process.stdin.on('end', () => {
    handleData();
});

function handleData() {
    if (buffer.length < 4) {
        return;
    }

    const length = buffer.readUInt32LE(0);
    console.log(length);
    console.log(buffer.toString());
    if (buffer.length < 4 + length) {
        return;
    }

    const bytes = buffer.subarray(4, 4 + length);
    buffer = buffer.subarray(4 + length);

    try {
        const message = JSON.parse(bytes);
        handleMessage(message);
    } catch (_) {
        sendResponse(makeError("Request JSON parse error"));
        return;
    }
}

// format
// clear_cache
// shutdown
// error

function handleMessage(message) {
    console.log(message);
    sendResponse({ method: "hi", result: null });
}

function makeError(message) {
    return { method: "error", message };
}

function sendResponse(response) {
    let message = Buffer.from(JSON.stringify(response));
    let length = Buffer.alloc(4);
    length.writeUInt32LE(message.length);
    process.stdout.write(length);
    process.stdout.write(message);
}
