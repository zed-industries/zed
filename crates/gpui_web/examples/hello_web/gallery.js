const route = window.location.pathname.replace(/\/+$/, "") || "/";
const gallery = document.getElementById("gallery");
const back = document.getElementById("back");
const status = document.getElementById("status");
const example = [...gallery.querySelectorAll("a[data-module]")].find(
    (link) => new URL(link.href).pathname === route,
);

function showError(error) {
    console.error(error);
    status.hidden = false;
    status.textContent = `Unable to start example: ${error.message ?? error}`;
    status.setAttribute("role", "alert");
}

if (route !== "/") {
    gallery.hidden = true;
    back.hidden = false;
    status.hidden = false;

    if (!example) {
        showError(new Error(`Unknown example path "${route}". Choose an example from the gallery.`));
    } else {
        const title = example.querySelector("strong").textContent;
        document.title = `${title} — GPUI Web Examples`;
        status.textContent = `Loading ${title}…`;

        try {
            if (!window.crossOriginIsolated) {
                throw new Error("Shared-memory WASM requires cross-origin isolation. Serve this page with trunk serve.");
            }

            const bindings = await import(`/${example.dataset.module}.js`);
            await bindings.default();
            window.wasmBindings = bindings;
            // GPUI chooses WebGPU or WebGL2 and reports initialization failures itself.
            status.hidden = true;
        } catch (error) {
            showError(error);
        }
    }
}
