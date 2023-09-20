const { Buffer } = require('buffer');
const fs = require("fs");
const path = require("path");
const { once } = require('events');

const prettierContainerPath = process.argv[2];
if (prettierContainerPath == null || prettierContainerPath.length == 0) {
    sendResponse(makeError(`Prettier path argument was not specified or empty.\nUsage: ${process.argv[0]} ${process.argv[1]} prettier/path`));
    process.exit(1);
}
fs.stat(prettierContainerPath, (err, stats) => {
    if (err) {
        sendResponse(makeError(`Path '${prettierContainerPath}' does not exist.`));
        process.exit(1);
    }

    if (!stats.isDirectory()) {
        sendResponse(makeError(`Path '${prettierContainerPath}' exists but is not a directory.`));
        process.exit(1);
    }
});
const prettierPath = path.join(prettierContainerPath, 'node_modules/prettier');


(async () => {
    let prettier;
    try {
        prettier = await loadPrettier(prettierPath);
    } catch (e) {
        sendResponse(makeError(`Failed to load prettier: ${e}`));
        process.exit(1);
    }
    sendResponse(makeError("Prettier loadded successfully."));
    process.stdin.resume();
    handleBuffer(prettier);
})()

async function handleBuffer(prettier) {
    for await (let messageText of readStdin()) {
        let message;
        try {
            message = JSON.parse(messageText);
        } catch (e) {
            sendResponse(makeError(`Failed to parse message '${messageText}': ${e}`));
            continue;
        }
        handleMessage(message, prettier).catch(e => {
            sendResponse({ id: message.id, ...makeError(`error during message handling: ${e}`) });
        });
    }
}

const headerSeparator = "\r\n";
const contentLengthHeaderName = 'Content-Length';

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
        let headersLength = null;
        let messageLength = null;
        main_loop: while (true) {
            if (messageLength === null) {
                while (buffer.indexOf(`${headerSeparator}${headerSeparator}`) === -1) {
                    if (streamEnded) {
                        await handleStreamEnded('Unexpected end of stream: headers not found');
                        continue main_loop;
                    } else if (buffer.length > contentLengthHeaderName.length * 10) {
                        await handleStreamEnded(`Unexpected stream of bytes: no headers end found after ${buffer.length} bytes of input`);
                        continue main_loop;
                    }
                    await once(process.stdin, 'readable');
                }
                const headers = buffer.subarray(0, buffer.indexOf(`${headerSeparator}${headerSeparator}`)).toString('ascii');
                const contentLengthHeader = headers.split(headerSeparator).map(header => header.split(':'))
                    .filter(header => header[2] === undefined)
                    .filter(header => (header[1] || '').length > 0)
                    .find(header => header[0].trim() === contentLengthHeaderName);
                if (contentLengthHeader === undefined) {
                    await handleStreamEnded(`Missing or incorrect ${contentLengthHeaderName} header: ${headers}`);
                    continue main_loop;
                }
                headersLength = headers.length + headerSeparator.length * 2;
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
        sendResponse(makeError(`Error reading stdin: ${e}`));
    } finally {
        process.stdin.off('data', () => { });
    }
}

// ?
// shutdown
// error
async function handleMessage(message, prettier) {
    const { method, id, params } = message;
    if (method === undefined) {
        throw new Error(`Message method is undefined: ${JSON.stringify(message)}`);
    }
    if (id === undefined) {
        throw new Error(`Message id is undefined: ${JSON.stringify(message)}`);
    }

    if (method === 'prettier/format') {
        if (params === undefined || params.text === undefined) {
            throw new Error(`Message params.text is undefined: ${JSON.stringify(message)}`);
        }
        if (params.options === undefined) {
            throw new Error(`Message params.options is undefined: ${JSON.stringify(message)}`);
        }
        const formattedText = await prettier.format(params.text, params.options);
        sendResponse({ id, result: { text: formattedText } });
    } else if (method === 'prettier/clear_cache') {
        prettier.clearConfigCache();
        sendResponse({ id, result: null });
    } else if (method === 'initialize') {
        sendResponse({
            id,
            result: {
                "capabilities": {}
            }
        });
    } else {
        throw new Error(`Unknown method: ${method}`);
    }
}

function makeError(message) {
    return {
        error: {
            "code": -32600, // invalid request code
            message,
        }
    };
}

function sendResponse(response) {
    let responsePayloadString = JSON.stringify({
        jsonrpc: "2.0",
        ...response
    });
    let headers = `${contentLengthHeaderName}: ${Buffer.byteLength(responsePayloadString)}${headerSeparator}${headerSeparator}`;
    let dataToSend = headers + responsePayloadString;
    process.stdout.write(dataToSend);
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
