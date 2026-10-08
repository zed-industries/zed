// This standalone example has an ignored lockfile. Before building, match
// event-listener to the repository Cargo.lock (currently 5.4.1) so the test
// exercises the production synchronization backend:
// cargo update -p event-listener --precise 5.4.1
// cargo tree --target wasm32-unknown-unknown -i event-listener -e features
// The Wasm feature tree must not enable event-listener/std (a blocking mutex).
// Then, with the hello_web gallery served by Trunk:
// node test_http_client.mjs http://127.0.0.1:8080
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
const output = path.join(path.dirname(fileURLToPath(import.meta.url)), "target", "http-client-test", String(Date.now()));
fs.mkdirSync(output, { recursive: true });
const log = fs.openSync(path.join(output, "chrome.log"), "w");
const browser = spawn(chrome, [
    "--headless=new", "--no-first-run", "--no-default-browser-check",
    "--remote-debugging-port=0", `--user-data-dir=${path.join(output, "profile")}`,
    "about:blank",
], { stdio: ["ignore", log, log] });
const delay = milliseconds => new Promise(resolve => setTimeout(resolve, milliseconds));
let launchError;
browser.on("error", error => { launchError = error; });
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
        } else if (message.method === "Runtime.exceptionThrown") {
            errors.push(message.params);
            for (const request of pending.values()) {
                clearTimeout(request.timer);
                request.reject(new Error(JSON.stringify(message.params.exceptionDetails)));
            }
            pending.clear();
        }
    });
    const call = (method, params = {}) => new Promise((resolve, reject) => {
        const id = ++nextId;
        // This deadline is outside the page so a main-thread trap or hang cannot hide.
        const timer = setTimeout(() => {
            pending.delete(id);
            reject(new Error(`Timed out: ${method}; see ${output}/chrome.log`));
        }, 120_000);
        timer.unref();
        pending.set(id, { resolve, reject, timer });
        socket.send(JSON.stringify({ id, method, params }));
    });
    await call("Runtime.enable");
    await call("Page.enable");
    const loaded = new Promise((resolve, reject) => {
        const timer = setTimeout(() => reject(new Error("Page did not load")), 10_000);
        socket.addEventListener("message", function onLoad({ data }) {
            if (JSON.parse(data).method === "Page.loadEventFired") {
                clearTimeout(timer);
                socket.removeEventListener("message", onLoad);
                resolve();
            }
        });
    });
    await call("Page.navigate", { url: origin });
    await loaded;
    const result = await call("Runtime.evaluate", {
        awaitPromise: true, returnByValue: true,
        expression: `(async () => {
            const check = (condition, message) => { if (!condition) throw new Error(message); };
            check(crossOriginIsolated, "Shared-memory isolation headers are required");
            const fixture = await import("/http_client_test.js");
            const bindings = await fixture.default();
            check(bindings.memory.buffer instanceof SharedArrayBuffer, "Wasm memory must be shared");
            let waitForbidden = false;
            try { Atomics.wait(new Int32Array(bindings.memory.buffer), 0, 0, 0); }
            catch (error) { waitForbidden = error instanceof TypeError; }
            check(waitForbidden, "Blocking waits must be forbidden on the browser main thread");
            const yieldToBrowser = () => new Promise(resolve => setTimeout(resolve, 10));
            const states = new Map();
            const originalFetch = globalThis.fetch;
            globalThis.fetch = async request => {
                const mode = new URL(request.url).pathname.slice(1);
                if (mode === "no-body") return new Response(null);
                const state = { pulled: 0, cancelled: 0 };
                states.set(mode, state);
                return new Response(new ReadableStream({
                    pull(controller) {
                        if (mode === "pending") return new Promise(() => {});
                        if (mode === "error" && state.pulled === 16) {
                            controller.error(new Error("injected stream failure"));
                            return;
                        }
                        if (mode === "invalid") {
                            controller.enqueue("not bytes");
                            return;
                        }
                        if (state.pulled === 32768) {
                            controller.close();
                            return;
                        }
                        const index = state.pulled++;
                        // Empty chunks exercise IntoAsyncRead's skip path.
                        if (index % 7 === 0) controller.enqueue(new Uint8Array());
                        controller.enqueue(new Uint8Array([index % 251, index % 239, index % 227]));
                    },
                    cancel() { state.cancelled++; },
                }, { highWaterMark: 0 }));
            };
            try {
                for (const threaded of [false, true]) {
                    const client = new fixture.FetchTest(threaded);
                    const open = mode => client.open(location.origin + "/" + mode);
                    const noBody = await open("no-body");
                    check((await noBody.read(threaded)).length === 0, "Missing body must read as EOF");
                    noBody.free();

                    for (const mode of ["pending", "full"]) {
                        const body = await open(mode);
                        await yieldToBrowser();
                        const state = states.get(mode);
                        if (mode === "full") {
                            const pulled = state.pulled;
                            check(pulled > 0 && pulled <= 10, "Response buffering must be bounded");
                            await yieldToBrowser();
                            check(state.pulled === pulled, "Full channel must stop pulling");
                        }
                        body.free();
                        for (let attempt = 0; state.cancelled === 0 && attempt < 100; attempt++) {
                            await yieldToBrowser();
                        }
                        check(state.cancelled === 1, "Dropping the body must cancel the pending or backpressured reader");
                    }

                    for (const [mode, expected] of [
                        ["error", "injected stream failure"],
                        ["invalid", "non-byte chunk"],
                    ]) {
                        const body = await open(mode);
                        let error;
                        try { await body.read(threaded); } catch (caught) { error = String(caught); }
                        body.free();
                        check(error?.includes(expected), "Stream errors must reach the body reader: " + error);
                    }

                    const readers = threaded ? 8 : 1;
                    await Promise.all(Array.from({ length: readers }, async (_, reader) => {
                        const body = await open("stream-" + reader);
                        // Let the channel fill before worker reads begin.
                        await yieldToBrowser();
                        const bytes = await body.read(threaded);
                        body.free();
                        check(bytes.length === 32768 * 3, "All bytes must reach EOF");
                        for (let index = 0; index < 32768; index++) {
                            check(bytes[index * 3] === index % 251 &&
                                bytes[index * 3 + 1] === index % 239 &&
                                bytes[index * 3 + 2] === index % 227, "Chunks must remain ordered");
                        }
                        check(states.get("stream-" + reader).cancelled === 0, "EOF must not cancel a completed stream");
                    }));
                    client.free();
                }
            } finally {
                globalThis.fetch = originalFetch;
            }
            return { modes: ["main-thread", "workers"], chunksPerResponse: 32768, workerResponses: 8 };
        })()`,
    });
    assert.equal(result.exceptionDetails, undefined, JSON.stringify(result.exceptionDetails));
    assert.deepEqual(errors, [], JSON.stringify(errors));
    assert.equal(result.result.value.workerResponses, 8);
    console.log("PASS: Fetch body ordering, EOF, errors, backpressure, cancellation, and worker contention", result.result.value);
} finally {
    socket?.close();
    browser.kill();
    fs.closeSync(log);
}
