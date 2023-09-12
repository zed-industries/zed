const { Buffer } = require('buffer');
const fs = require("fs");
const path = require("path");

let prettierContainerPath = process.argv[2];
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
let prettierPath = path.join(prettierContainerPath, 'node_modules/prettier');

(async () => {
    let prettier;
    try {
        prettier = await loadPrettier(prettierPath);
    } catch (error) {
        console.error(error);
        process.exit(1);
    }
    console.log("Prettier loadded successfully.");
    // TODO kb do the rest here
})()

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
