const { Buffer } = require("buffer");
const fs = require("fs");
const path = require("path");
const { once } = require("events");

const prettierContainerPath = process.argv[2];
if (prettierContainerPath == null || prettierContainerPath.length == 0) {
  process.stderr.write(
    `Prettier path argument was not specified or empty.\nUsage: ${process.argv[0]} ${process.argv[1]} prettier/path\n`,
  );
  process.exit(1);
}
fs.stat(prettierContainerPath, (err, stats) => {
  if (err) {
    process.stderr.write(`Path '${prettierContainerPath}' does not exist\n`);
    process.exit(1);
  }

  if (!stats.isDirectory()) {
    process.stderr.write(
      `Path '${prettierContainerPath}' exists but is not a directory\n`,
    );
    process.exit(1);
  }
});
const prettierPath = path.join(prettierContainerPath, "node_modules/prettier");

class Prettier {
  constructor(path, prettier, config) {
    this.path = path;
    this.prettier = prettier;
    this.config = config;
  }
}

(async () => {
  let prettier;
  let config;
  try {
    prettier = await loadPrettier(prettierPath);
    config = (await prettier.resolveConfig(prettierPath)) || {};
  } catch (e) {
    process.stderr.write(`Failed to load prettier: ${e}\n`);
    process.exit(1);
  }
  process.stderr.write(
    `Prettier at path '${prettierPath}' loaded successfully, config: ${JSON.stringify(config)}\n`,
  );
  process.stdin.resume();
  handleBuffer(new Prettier(prettierPath, prettier, config));
})();

async function handleBuffer(prettier) {
  for await (const messageText of readStdin()) {
    let message;
    try {
      message = JSON.parse(messageText);
    } catch (e) {
      sendResponse(makeError(`Failed to parse message '${messageText}': ${e}`));
      continue;
    }
    // allow concurrent request handling by not `await`ing the message handling promise (async function)
    handleMessage(message, prettier).catch((e) => {
      const errorMessage = message;
      if ((errorMessage.params || {}).text !== undefined) {
        errorMessage.params.text = "..snip..";
      }
      sendResponse({
        id: message.id,
        ...makeError(
          `error during message '${JSON.stringify(errorMessage)}' handling: ${e}`,
        ),
      });
    });
  }
}

const headerSeparator = "\r\n";
const contentLengthHeaderName = "Content-Length";

async function* readStdin() {
  let buffer = Buffer.alloc(0);
  let streamEnded = false;
  process.stdin.on("end", () => {
    streamEnded = true;
  });
  process.stdin.on("data", (data) => {
    buffer = Buffer.concat([buffer, data]);
  });

  async function handleStreamEnded(errorMessage) {
    sendResponse(makeError(errorMessage));
    buffer = Buffer.alloc(0);
    messageLength = null;
    await once(process.stdin, "readable");
    streamEnded = false;
  }

  try {
    let headersLength = null;
    let messageLength = null;
    main_loop: while (true) {
      if (messageLength === null) {
        while (buffer.indexOf(`${headerSeparator}${headerSeparator}`) === -1) {
          if (streamEnded) {
            await handleStreamEnded(
              "Unexpected end of stream: headers not found",
            );
            continue main_loop;
          } else if (buffer.length > contentLengthHeaderName.length * 10) {
            await handleStreamEnded(
              `Unexpected stream of bytes: no headers end found after ${buffer.length} bytes of input`,
            );
            continue main_loop;
          }
          await once(process.stdin, "readable");
        }
        const headers = buffer
          .subarray(0, buffer.indexOf(`${headerSeparator}${headerSeparator}`))
          .toString("ascii");
        const contentLengthHeader = headers
          .split(headerSeparator)
          .map((header) => header.split(":"))
          .filter((header) => header[2] === undefined)
          .filter((header) => (header[1] || "").length > 0)
          .find(
            (header) => (header[0] || "").trim() === contentLengthHeaderName,
          );
        const contentLength = (contentLengthHeader || [])[1];
        if (contentLength === undefined) {
          await handleStreamEnded(
            `Missing or incorrect ${contentLengthHeaderName} header: ${headers}`,
          );
          continue main_loop;
        }
        headersLength = headers.length + headerSeparator.length * 2;
        messageLength = parseInt(contentLength, 10);
      }

      while (buffer.length < headersLength + messageLength) {
        if (streamEnded) {
          await handleStreamEnded(
            `Unexpected end of stream: buffer length ${buffer.length} does not match expected header length ${headersLength} + body length ${messageLength}`,
          );
          continue main_loop;
        }
        await once(process.stdin, "readable");
      }

      const messageEnd = headersLength + messageLength;
      const message = buffer.subarray(headersLength, messageEnd);
      buffer = buffer.subarray(messageEnd);
      headersLength = null;
      messageLength = null;
      yield message.toString("utf8");
    }
  } catch (e) {
    sendResponse(makeError(`Error reading stdin: ${e}`));
  } finally {
    process.stdin.off("data", () => {});
  }
}

async function handleMessage(message, prettier) {
  const { method, id, params } = message;
  if (method === undefined) {
    throw new Error(`Message method is undefined: ${JSON.stringify(message)}`);
  } else if (method == "initialized") {
    return;
  }

  if (id === undefined) {
    throw new Error(`Message id is undefined: ${JSON.stringify(message)}`);
  }

  if (method === "prettier/format") {
    if (params === undefined || params.text === undefined) {
      throw new Error(
        `Message params.text is undefined: ${JSON.stringify(message)}`,
      );
    }
    if (params.options === undefined) {
      throw new Error(
        `Message params.options is undefined: ${JSON.stringify(message)}`,
      );
    }

    let resolvedConfig = {};
    if (params.options.filepath) {
      resolvedConfig =
        (await prettier.prettier.resolveConfig(params.options.filepath)) || {};
    }

    // Marking the params.options.filepath as undefined makes
    // prettier.format() work even if no filepath is set.
    if (params.options.filepath === null) {
      params.options.filepath = undefined;
    }

    const plugins =
      Array.isArray(resolvedConfig?.plugins) &&
      resolvedConfig.plugins.length > 0
        ? resolvedConfig.plugins
        : params.options.plugins;

    const options = {
      ...(params.options.prettierOptions || prettier.config),
      ...resolvedConfig,
      plugins,
      parser: params.options.parser,
      filepath: params.options.filepath,
    };
    process.stderr.write(
      `Resolved config: ${JSON.stringify(resolvedConfig)}, will format file '${
        params.options.filepath || ""
      }' with options: ${JSON.stringify(options)}\n`,
    );
    const formattedText = await prettier.prettier.format(params.text, options);
    sendResponse({ id, result: { text: formattedText } });
  } else if (method === "prettier/clear_cache") {
    prettier.prettier.clearConfigCache();
    prettier.config =
      (await prettier.prettier.resolveConfig(prettier.path)) || {};
    sendResponse({ id, result: null });
  } else if (method === "initialize") {
    sendResponse({
      id,
      result: {
        capabilities: {},
      },
    });
  } else {
    throw new Error(`Unknown method: ${method}`);
  }
}

function makeError(message) {
  return {
    error: {
      code: -32600, // invalid request code
      message,
    },
  };
}

function sendResponse(response) {
  const responsePayloadString = JSON.stringify({
    jsonrpc: "2.0",
    ...response,
  });
  const headers = `${contentLengthHeaderName}: ${Buffer.byteLength(
    responsePayloadString,
  )}${headerSeparator}${headerSeparator}`;
  process.stdout.write(headers + responsePayloadString);
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
          reject(
            `Error requiring prettier module from path '${prettierPath}'.Error: ${err}`,
          );
        }
      }
    });
  });
}
