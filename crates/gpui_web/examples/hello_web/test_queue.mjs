// With the hello_web gallery served by Trunk:
// node test_queue.mjs http://127.0.0.1:8080 [worker-count]
import assert from "node:assert/strict";
import fs from "node:fs";
import path from "node:path";
import { spawn } from "node:child_process";
import { fileURLToPath } from "node:url";

const origin = new URL(process.argv[2] ?? "http://127.0.0.1:8080").origin;
const workers = Number(process.argv[3] ?? 8);
assert.ok(Number.isInteger(workers) && workers > 0);
const chrome = process.env.CHROME ??
    (process.platform === "darwin"
        ? "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome"
        : "chromium");
const output = path.join(path.dirname(fileURLToPath(import.meta.url)), "target", "queue-test", String(Date.now()));
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
        }
    });
    const call = (method, params = {}) => new Promise((resolve, reject) => {
        const id = ++nextId;
        // This deadline runs outside the page, so it also catches a main-thread hang.
        const timer = setTimeout(() => reject(new Error(`Timed out: ${method}`)), 120_000);
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
            if (!crossOriginIsolated) throw new Error("Shared-memory isolation headers are required");
            const fixture = await import("/queue_test.js");
            await fixture.default();
            let trapped = false;
            try { fixture.forbidden_main_thread_wait(); }
            catch (error) { trapped = error instanceof WebAssembly.RuntimeError; }
            if (!trapped) throw new Error("Main-thread Wasm waits must trap in this browser");
            const rounds = 128, batch = 4096;
            const stress = new fixture.QueueStress(${workers}, rounds * batch);
            const yieldToBrowser = () => new Promise(resolve => setTimeout(resolve, 2));
            while (!stress.ready()) await yieldToBrowser();
            for (let round = 0; round < rounds; round++) {
                stress.send_batch(batch);
                while (stress.drain() !== (round + 1) * batch) await yieldToBrowser();
                // Let the request queue empty so workers repeatedly park and get notified.
                await yieldToBrowser();
            }
            stress.stop();
            while (!stress.stopped()) await yieldToBrowser();
            stress.free();
            return { workers: ${workers}, items: rounds * batch, rounds };
        })()`,
    });
    assert.equal(result.exceptionDetails, undefined, JSON.stringify(result.exceptionDetails));
    assert.deepEqual(errors, [], JSON.stringify(errors));
    assert.equal(result.result.value.items, 128 * 4096);
    console.log("PASS: main-thread/worker queue contention", result.result.value);
} finally {
    socket?.close();
    browser.kill();
    fs.closeSync(log);
}
