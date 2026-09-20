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
const superseded = {
  id: "loser",
  queue: "superseded",
  source_path: "source.md",
  target_path: "target.md",
  reason_probability: 0.9,
  destination_probability: 0.8,
  anchor_choice: "anchor_000",
  anchor_probability: 0.8,
  anchor_quality_probability: 0.9,
  superseded_by: "winner",
  anchor: {
    text: "command palette",
    start,
    end: start + "command palette".length,
    block_start: 0,
    block_end: markdown.length,
    relative_target: "./target.md",
  },
};
vm.runInContext(
  `decisions.push(${JSON.stringify(superseded)}); pages["target.md"] = ${JSON.stringify({ markdown: "# Target\n" })}; document.getElementById("status").value = "all"; document.getElementById("search").value = ""; applyFilters()`,
  context,
);
const card = elements.get("main").innerHTML;
assert.match(card, /Superseded by:/);
assert.match(card, /data-label="pass" disabled/);
vm.runInContext('setLabel("pass")', context);
assert.equal(vm.runInContext("labels.loser", context), undefined);

console.log("review UI tests passed");
