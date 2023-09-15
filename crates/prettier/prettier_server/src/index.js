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
    let buffer = Buffer.alloc(0);
    let streamEnded = false;
    process.stdin.on('end', () => {
        streamEnded = true;
    });
    process.stdin.on('data', (data) => {
        buffer = Buffer.concat([buffer, data]);
    });

    async function handleStreamEnded(errorMessage) {
        sendResponse(makeError(errorMessage));
        buffer = Buffer.alloc(0);
        messageLength = null;
        await once(process.stdin, 'readable');
        streamEnded = false;
    }

    try {
        const headersSeparator = "\r\n\r\n";
        let contentLengthHeaderName = 'Content-Length';
        let headersLength = null;
        let messageLength = null;
        main_loop: while (true) {
            if (messageLength === null) {
                while (buffer.indexOf(headersSeparator) === -1) {
                    if (streamEnded) {
                        await handleStreamEnded('Unexpected end of stream: headers not found');
                        continue main_loop;
                    } else if (buffer.length > contentLengthHeaderName.length * 10) {
                        await handleStreamEnded(`Unexpected stream of bytes: no headers end found after ${buffer.length} bytes of input`);
                        continue main_loop;
                    }
                    await once(process.stdin, 'readable');
                }
                const headers = buffer.subarray(0, buffer.indexOf(headersSeparator)).toString('ascii');
                const contentLengthHeader = headers.split('\r\n').map(header => header.split(': '))
                    .filter(header => header[2] === undefined)
                    .filter(header => (header[1] || '').length > 0)
                    .find(header => header[0].trim() === contentLengthHeaderName);
                if (contentLengthHeader === undefined) {
                    await handleStreamEnded(`Missing or incorrect Content-Length header: ${headers}`);
                    continue main_loop;
                }
                headersLength = headers.length + headersSeparator.length;
                messageLength = parseInt(contentLengthHeader[1], 10);
            }

            while (buffer.length < (headersLength + messageLength)) {
                if (streamEnded) {
                    await handleStreamEnded(
                        `Unexpected end of stream: buffer length ${buffer.length} does not match expected header length ${headersLength} + body length ${messageLength}`);
                    continue main_loop;
                }
                await once(process.stdin, 'readable');
            }

            const messageEnd = headersLength + messageLength;
            const message = buffer.subarray(headersLength, messageEnd);
            buffer = buffer.subarray(messageEnd);
            messageLength = null;
            yield message.toString('utf8');
        }
    } catch (e) {
        console.error(`Error reading stdin: ${e}`);
    } finally {
        process.stdin.off('data', () => { });
    }
}

async function handleData(messageText, prettier) {
    try {
        const message = JSON.parse(messageText);
        await handleMessage(prettier, message);
    } catch (e) {
        sendResponse(makeError(`Request JSON parse error: ${e}`));
    }
}

// format
// clear_cache
//
// shutdown
// error

async function handleMessage(prettier, message) {
    // TODO kb handle message.method, message.params and message.id
    console.log(`message: ${JSON.stringify(message)}`);
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
