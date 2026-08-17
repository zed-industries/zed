onmessage = function (ev) {
    let [ia, index, value] = ev.data;
    ia = new Int32Array(ia.buffer);
    let result = Atomics.wait(ia, index, value);
    postMessage(result);
};
