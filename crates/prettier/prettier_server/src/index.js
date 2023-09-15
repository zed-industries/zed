const { Buffer } = require('buffer');
const fs = require("fs");
const path = require("path");
const { once } = require('events');

const prettierContainerPath = process.argv[2];
if (prettierContainerPath == null || prettierContainerPath.length == 0) {
    console.error(`Prettier path argument was not specified or empty.\nUsage: ${process.argv[0]} ${process.argv[1]} prettier/path`);
    process.exit(1);
}
fs.stat(prettierContainerPath, (err, stats) => {
    if (err) {
        console.error(`Path '${prettierContainerPath}' does not exist.`);
        process.exit(1);
    }

    if (!stats.isDirectory()) {
        console.log(`Path '${prettierContainerPath}' exists but is not a directory.`);
        process.exit(1);
    }
});
const prettierPath = path.join(prettierContainerPath, 'node_modules/prettier');


(async () => {
    let prettier;
    try {
        prettier = await loadPrettier(prettierPath);
    } catch (error) {
        console.error("Failed to load prettier: ", error);
        process.exit(1);
    }
    console.log("Prettier loadded successfully.");
    process.stdin.resume();
    handleBuffer(prettier);
})()

async function handleBuffer(prettier) {
    for await (let messageText of readStdin()) {
        handleData(messageText, prettier).catch(e => {
            console.error("Failed to handle formatter request", e);
        });
    }
}

async function* readStdin() {
    const bufferLengthOffset = 4;
    let buffer = Buffer.alloc(0);
    let streamEnded = false;
    process.stdin.on('end', () => {
        streamEnded = true;
    });
    process.stdin.on('data', (data) => {
        buffer = Buffer.concat([buffer, data]);
    });

    try {
        main_loop: while (true) {
            while (buffer.length < bufferLengthOffset) {
                if (streamEnded) {
                    sendResponse(makeError(`Unexpected end of stream: less than ${bufferLengthOffset} characters passed`));
                    buffer = Buffer.alloc(0);
                    streamEnded = false;
                    await once(process.stdin, 'readable');
                    continue main_loop;
                }
                await once(process.stdin, 'readable');
            }

            const length = buffer.readUInt32LE(0);

            while (buffer.length < (bufferLengthOffset + length)) {
                if (streamEnded) {
                    sendResponse(makeError(
                        `Unexpected end of stream: buffer length ${buffer.length} does not match expected length ${bufferLengthOffset} + ${length}`));
                    buffer = Buffer.alloc(0);
                    streamEnded = false;
                    await once(process.stdin, 'readable');
                    continue main_loop;
                }
                await once(process.stdin, 'readable');
            }

            const message = buffer.subarray(4, 4 + length);
            buffer = buffer.subarray(4 + length);
            yield message.toString('utf8');
        }
    } catch (e) {
        console.error(`Error reading stdin: ${e}`);
    } finally {
        process.stdin.off('data');
    }
}

async function handleData(messageText, prettier) {
    try {
        const message = JSON.parse(messageText);
        await handleMessage(prettier, message);
    } catch (e) {
        sendResponse(makeError(`Request JSON parse error: ${e}`));
        return;
    }
}

// format
// clear_cache
//
// shutdown
// error

async function handleMessage(prettier, message) {
    console.log(`message: ${message}`);
    sendResponse({ method: "hi", result: null });
}

function makeError(message) {
    return { method: "error", message };
}

function sendResponse(response) {
    const message = Buffer.from(JSON.stringify(response));
    const length = Buffer.alloc(4);
    length.writeUInt32LE(message.length);
    process.stdout.write(length);
    process.stdout.write(message);
}

function loadPrettier(prettierPath) {
    return new Promise((resolve, reject) => {
        fs.access(prettierPath, fs.constants.F_OK, (err) => {
            if (err) {
                reject(`Path '${prettierPath}' does not exist.Error: ${err}`);
            } else {
                try {
                    resolve(require(prettierPath));
                } catch (err) {
                    reject(`Error requiring prettier module from path '${prettierPath}'.Error: ${err}`);
                }
            }
        });
    });
}
