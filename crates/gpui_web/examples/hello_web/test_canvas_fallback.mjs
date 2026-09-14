import assert from "node:assert/strict";
import fs from "node:fs";
import path from "node:path";
import { spawn } from "node:child_process";
import { fileURLToPath } from "node:url";

const origin = new URL(process.argv[2] ?? "http://127.0.0.1:8080").origin;
const chrome = process.env.CHROME ??
    (process.platform === "darwin"
        ? "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome"
        : "chromium");
const output = path.join(
    path.dirname(fileURLToPath(import.meta.url)),
    "target",
    "canvas-fallback-test",
    String(Date.now()),
);
fs.mkdirSync(output, { recursive: true });
const log = fs.openSync(path.join(output, "chrome.log"), "w");
const browser = spawn(chrome, [
    "--headless=new",
    "--no-first-run",
    "--no-default-browser-check",
    "--remote-debugging-port=0",
    `--user-data-dir=${path.join(output, "profile")}`,
    "about:blank",
], { stdio: ["ignore", log, log] });
let launchError;
browser.on("error", error => { launchError = error; });
const delay = milliseconds => new Promise(resolve => setTimeout(resolve, milliseconds));
let socket;

try {
    let port;
    for (let attempt = 0; attempt < 100; attempt++) {
        if (launchError) throw launchError;
        if (browser.exitCode !== null) throw new Error(`Chrome exited; see ${output}/chrome.log`);
        const portFile = path.join(output, "profile", "DevToolsActivePort");
        if (fs.existsSync(portFile)) {
            port = fs.readFileSync(portFile, "utf8").split("\n")[0];
            break;
        }
        await delay(100);
    }
    assert.ok(port, "Chrome did not start");
    const pages = await (await fetch(`http://127.0.0.1:${port}/json/list`)).json();
    socket = new WebSocket(pages.find(page => page.type === "page").webSocketDebuggerUrl);
    await new Promise((resolve, reject) => {
        socket.addEventListener("open", resolve, { once: true });
        socket.addEventListener("error", reject, { once: true });
    });

    let nextId = 0;
    const pending = new Map();
    const errors = [];
    socket.addEventListener("message", ({ data }) => {
        const message = JSON.parse(data);
        if (message.id) {
            const request = pending.get(message.id);
            if (!request) return;
            pending.delete(message.id);
            clearTimeout(request.timer);
            if (message.error) request.reject(new Error(JSON.stringify(message.error)));
            else request.resolve(message.result);
        } else if (message.method === "Runtime.exceptionThrown" ||
            (message.method === "Runtime.consoleAPICalled" && message.params.type === "error") ||
            (message.method === "Log.entryAdded" && message.params.entry.text.includes("willReadFrequently"))) {
            errors.push(message.params);
        }
    });
    const call = (method, params = {}) => new Promise((resolve, reject) => {
        const id = ++nextId;
        const timer = setTimeout(() => {
            pending.delete(id);
            reject(new Error(`Timed out: ${method}`));
        }, 15000);
        timer.unref();
        pending.set(id, { resolve, reject, timer });
        socket.send(JSON.stringify({ id, method, params }));
    });
    const evaluate = async expression => {
        const result = await call("Runtime.evaluate", {
            expression, returnByValue: true, awaitPromise: true,
        });
        assert.equal(result.exceptionDetails, undefined, JSON.stringify(result.exceptionDetails));
        return result.result.value;
    };
    await call("Runtime.enable");
    await call("Page.enable");
    await call("Log.enable");
    await call("Emulation.setDeviceMetricsOverride", {
        width: 1280, height: 900, deviceScaleFactor: 1, mobile: false,
    });
    await call("Page.addScriptToEvaluateOnNewDocument", { source: `
        window.canvasCalls = [];
        for (const name of ["measureText", "fillText"]) {
            const original = OffscreenCanvasRenderingContext2D.prototype[name];
            OffscreenCanvasRenderingContext2D.prototype[name] = function(...args) {
                canvasCalls.push({
                    type: name, text: args[0], font: this.font,
                    willReadFrequently: this.getContextAttributes().willReadFrequently,
                });
                return original.apply(this, args);
            };
        }
    ` });
    await call("Page.navigate", { url: `${origin}/input` });
    for (let attempt = 0; attempt < 150; attempt++) {
        if (await evaluate('!!document.querySelector("textarea")')) break;
        await delay(100);
    }
    assert.ok(await evaluate('!!document.querySelector("textarea")'), "Input example did not launch");
    await evaluate("new Promise(resolve => requestAnimationFrame(() => requestAnimationFrame(resolve)))");
    // Read GPUI's buffer through its Copy action without changing the system clipboard.
    await evaluate(`
        window.copiedText = "";
        Object.defineProperty(navigator.clipboard, "writeText", {
            value: async text => { copiedText = text; },
        });
        window.sendKey = (key, metaKey = false) => {
            for (const type of ["keydown", "keyup"]) {
                document.querySelector("textarea").dispatchEvent(
                    new KeyboardEvent(type, { key, metaKey, bubbles: true, cancelable: true }),
                );
            }
        };
    `);
    const key = (key, meta = false) => evaluate(`sendKey(${JSON.stringify(key)}, ${meta})`);
    const insert = text => call("Input.insertText", { text });
    const readText = async () => {
        await key("a", true);
        await evaluate('copiedText = ""');
        await key("c", true);
        const text = await evaluate("copiedText");
        await key("ArrowRight");
        return text;
    };
    const rasterCount = text => evaluate(
        `canvasCalls.filter(call => call.type === "fillText" && call.text === ${JSON.stringify(text)}).length`,
    );

    await insert("Hello ©");
    await delay(200);
    assert.equal(await evaluate("canvasCalls.length"), 0, "Bundled glyphs should not use Canvas");
    await key("a", true);
    await insert("😀😀😀");
    await delay(200);
    assert.equal(await rasterCount("😀"), 1, "Repeated emoji should share one atlas image");
    assert.equal(await readText(), "😀😀😀");
    await key("ArrowLeft");
    await key("ArrowRight");
    await delay(200);
    assert.equal(await rasterCount("😀"), 1, "Caret movement should not rerasterize emoji");

    const text = "A中文 が か\u3099 각 각 😀 ❤️ 👍🏽 🇯🇵 1️⃣ 👩‍💻 👨‍👩‍👧‍👦Z";
    await key("a", true);
    await insert(text);
    await delay(200);
    assert.equal(await readText(), text);
    for (const grapheme of ["❤️", "👍🏽", "🇯🇵", "1⃣", "👩‍💻", "👨‍👩‍👧‍👦"]) {
        assert.ok(await rasterCount(grapheme) > 0, `Missing whole-grapheme raster: ${grapheme}`);
    }
    for (const grapheme of ["中", "文", "が", "か\u3099", "각", "각"]) {
        assert.equal(await rasterCount(grapheme), 0, `Default policy should not rasterize CJK: ${grapheme}`);
    }
    assert.ok(
        await evaluate("canvasCalls.every(call => call.willReadFrequently === true)"),
        "Glyph canvases should be configured for frequent readback",
    );
    const screenshot = await call("Page.captureScreenshot", { format: "png" });
    fs.writeFileSync(path.join(output, "input.png"), Buffer.from(screenshot.data, "base64"));
    await key("Backspace");
    await key("Backspace");
    assert.equal(await readText(), text.slice(0, -"👨‍👩‍👧‍👦Z".length));

    await key("a", true);
    await evaluate("canvasCalls = []");
    await insert("ش");
    await delay(200);
    assert.equal(await evaluate("canvasCalls.length"), 0, "Unsupported scripts should stay native");
    assert.deepEqual(errors, [], "Browser reported errors");
    console.log(`Canvas fallback checks passed. Screenshot and browser log: ${output}`);
} finally {
    socket?.close();
    browser.kill();
    fs.closeSync(log);
}
