// synchronously, using the browser, import wasm_bindgen shim JS scripts
import init, { wasm_thread_entry_point } from "WASM_BINDGEN_SHIM_URL";

// Wait for the main thread to send us the shared module/memory and work context.
// Once we've got it, initialize it all with the `wasm_bindgen` global we imported via
// `importScripts`.
self.onmessage = event => {
    const [module_or_path, memory, work] = event.data;

    init({ module_or_path, memory }).catch(err => {
        console.log(err);

        // Propagate to main `onerror`:
        setTimeout(() => {
            throw err;
        });
        // Rethrow to keep promise rejected and prevent execution of further commands:
        throw err;
    }).then(() => {
        // Enter rust code by calling entry point defined in `lib.rs`.
        // This executes closure defined by work context.
        wasm_thread_entry_point(work);

        // Once done, terminate web worker
        close();
    });
};