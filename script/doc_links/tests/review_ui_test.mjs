import assert from "node:assert/strict";
import fs from "node:fs";
import vm from "node:vm";

const element = () => ({
  value: "all",
  innerHTML: "",
  textContent: "",
  disabled: false,
  style: {},
  addEventListener() {},
  matches() {
    return false;
  },
});
const elements = new Map();
const context = {
  window: {
    REVIEW_DATA: {
      report_hash: "report",
      decisions: [],
      pages: {},
    },
    location: {},
  },
  document: {
    getElementById(id) {
      if (!elements.has(id)) elements.set(id, element());
      return elements.get(id);
    },
    querySelectorAll() {
      return [];
    },
    addEventListener() {},
    createElement() {
      return element();
    },
  },
  localStorage: {
    getItem() {
      return null;
    },
    setItem() {},
  },
  URL: {
    createObjectURL() {
      return "blob:test";
    },
    revokeObjectURL() {},
  },
  Blob: class {},
  console,
  setTimeout,
  clearTimeout,
};
vm.createContext(context);
vm.runInContext(fs.readFileSync("script/doc_links/ui/review.js", "utf8"), context);

const markdown = "- Use the command palette for **actions**.\n";
const start = markdown.indexOf("command palette");
const decision = {
  source_path: "source.md",
  anchor: {
    text: "command palette",
    start,
    end: start + "command palette".length,
    block_start: 0,
    block_end: markdown.length,
  },
};
context.window.REVIEW_DATA.pages["source.md"] = { markdown };
const rendered = vm.runInContext(`sourceContext(${JSON.stringify(decision)})`, context);
assert.equal(rendered, "<ul><li>Use the <mark>command palette</mark> for <strong>actions</strong>.</li></ul>");
console.log("review UI tests passed");
