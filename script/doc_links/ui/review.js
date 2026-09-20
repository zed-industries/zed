const data = window.REVIEW_DATA;
const storageKey = `zed-doc-links:${data.report_hash}`;
const decisions = data.decisions;
const pages = data.pages;
let labels = loadLabels();
let filtered = [];
let index = 0;
let history = [];

function escapeHtml(value) {
  return String(value ?? "")
    .replaceAll("&", "&amp;")
    .replaceAll("<", "&lt;")
    .replaceAll(">", "&gt;")
    .replaceAll('"', "&quot;")
    .replaceAll("'", "&#039;");
}

function safeHref(href) {
  return /^(https?:\/\/|\.\.?\/|#)/.test(href) ? href : "#";
}

function renderInline(value) {
  let text = escapeHtml(value);
  text = text.replace(/`([^`]+)`/g, "<code>$1</code>");
  text = text.replace(/\*\*([^*]+)\*\*/g, "<strong>$1</strong>");
  return text.replace(/\[([^\]]+)\]\(([^)]+)\)/g, (_, label, href) => `<a href="${safeHref(href)}">${label}</a>`);
}

function renderMarkdown(value) {
  const output = [];
  let code = [];
  let fence = null;
  let listOpen = false;
  for (const raw of String(value ?? "").split("\n")) {
    const fenceMatch = raw.match(/^\s*(`{3,}|~{3,})/);
    if (fenceMatch) {
      if (!fence) {
        fence = fenceMatch[1];
      } else if (fence[0] === fenceMatch[1][0] && fenceMatch[1].length >= fence.length) {
        output.push(`<pre><code>${escapeHtml(code.join("\n"))}</code></pre>`);
        code = [];
        fence = null;
      } else {
        code.push(raw);
      }
      continue;
    }
    if (fence) {
      code.push(raw);
      continue;
    }
    const heading = raw.match(/^(#{1,4})\s+(.+)$/);
    const list = raw.match(/^\s*[-*+]\s+(.+)$/);
    if (!list && listOpen) {
      output.push("</ul>");
      listOpen = false;
    }
    if (heading) {
      const level = Math.min(heading[1].length + 1, 5);
      output.push(`<h${level}>${renderInline(heading[2])}</h${level}>`);
    } else if (list) {
      if (!listOpen) {
        output.push("<ul>");
        listOpen = true;
      }
      output.push(`<li>${renderInline(list[1])}</li>`);
    } else if (raw.startsWith(">")) {
      output.push(`<blockquote>${renderInline(raw.replace(/^>\s?/, ""))}</blockquote>`);
    } else if (raw.trim().startsWith("|")) {
      output.push(`<pre>${escapeHtml(raw)}</pre>`);
    } else if (raw.trim()) {
      output.push(`<p>${renderInline(raw)}</p>`);
    }
  }
  if (listOpen) output.push("</ul>");
  if (fence) output.push(`<pre><code>${escapeHtml(code.join("\n"))}</code></pre>`);
  return output.join("");
}

function loadLabels() {
  try {
    return JSON.parse(localStorage.getItem(storageKey) || "{}");
  } catch (_) {
    return {};
  }
}

function saveLabels() {
  try {
    localStorage.setItem(storageKey, JSON.stringify(labels));
  } catch (_) {}
}

function currentLabel(id) {
  return labels[id] || { label: null, notes: "" };
}

function applyFilters(preserveId = null) {
  const queue = document.getElementById("queue").value;
  const status = document.getElementById("status").value;
  const query = document.getElementById("search").value.toLowerCase().trim();
  filtered = decisions.filter((decision) => {
    if (queue !== "all" && decision.queue !== queue) return false;
    const label = currentLabel(decision.id).label;
    if (status === "unlabeled" && label) return false;
    if (status !== "all" && status !== "unlabeled" && label !== status) return false;
    return !query || `${decision.id} ${decision.source_path} ${decision.target_path}`.toLowerCase().includes(query);
  });
  if (preserveId) {
    const found = filtered.findIndex((item) => item.id === preserveId);
    index = found >= 0 ? found : Math.min(index, Math.max(0, filtered.length - 1));
  } else {
    index = Math.min(index, Math.max(0, filtered.length - 1));
  }
  render();
}

function sourceContext(decision) {
  const source = pages[decision.source_path].markdown;
  if (!decision.anchor) return "No exact existing anchor was selected.";
  const anchor = decision.anchor;
  const block = source.slice(anchor.block_start, anchor.block_end);
  const localStart = anchor.start - anchor.block_start;
  const localEnd = anchor.end - anchor.block_start;
  const marker = "__ZED_ANCHOR_MARK__";
  const marked = `${block.slice(0, localStart)}${marker}${block.slice(localEnd)}`;
  return renderMarkdown(marked).replace(marker, `<mark>${escapeHtml(anchor.text)}</mark>`);
}

function render() {
  const main = document.getElementById("main");
  if (!filtered.length) {
    main.innerHTML = '<div class="empty">No items match these filters.</div>';
    updateChrome();
    return;
  }
  const decision = filtered[index];
  const label = currentLabel(decision.id);
  const source = pages[decision.source_path];
  const target = pages[decision.target_path];
  const canPass = Boolean(decision.anchor) && decision.queue !== "superseded";
  const proposal = decision.anchor
    ? `<code>[${escapeHtml(decision.anchor.text)}](${escapeHtml(decision.anchor.relative_target)})</code>`
    : `<strong>${escapeHtml(decision.anchor_choice)}</strong>`;
  main.innerHTML = `
    <article class="card">
      <div class="card-header">
        <div class="route">
          <div><span class="path">Source</span><h2>${escapeHtml(decision.source_path)}</h2></div>
          <span class="arrow">→</span>
          <div><span class="path">Destination</span><h2>${escapeHtml(decision.target_path)}</h2></div>
        </div>
        <div class="badges">
          <span class="badge">Queue: <strong>${escapeHtml(decision.queue)}</strong></span>
          <span class="badge">Reason: <strong>${decision.reason_probability.toFixed(2)}</strong></span>
          <span class="badge">Destination: <strong>${decision.destination_probability.toFixed(2)}</strong></span>
          <span class="badge">Anchor: <strong>${decision.anchor_probability.toFixed(2)}</strong></span>
          <span class="badge">Anchor quality: <strong>${decision.anchor_quality_probability == null ? "n/a" : decision.anchor_quality_probability.toFixed(2)}</strong></span>
          <span class="badge">Choice: <strong>${escapeHtml(decision.anchor_choice)}</strong></span>
          ${decision.superseded_by ? `<span class="badge">Superseded by: <strong>${escapeHtml(decision.superseded_by)}</strong></span>` : ""}
        </div>
      </div>
      <div class="content">
        <section class="primary">
          <h3>Source context</h3><div class="context">${sourceContext(decision)}</div>
          <h3>Proposed change</h3><div class="proposal">${proposal}</div>
        </section>
        <aside class="sidebar">
          <h3>Review decision</h3>
          <div class="actions">
            <button class="action pass ${label.label === "pass" ? "active" : ""}" data-label="pass" ${canPass ? "" : "disabled"}>1 · Pass</button>
            <button class="action fail ${label.label === "fail" ? "active" : ""}" data-label="fail">2 · Fail</button>
            <button class="action defer ${label.label === "defer" ? "active" : ""}" data-label="defer">D · Defer</button>
          </div>
          <textarea id="notes" placeholder="Why should this pass, fail, or wait?">${escapeHtml(label.notes)}</textarea>
          <div class="item-id">ID: ${escapeHtml(decision.id)}</div>
        </aside>
      </div>
      <details><summary>Full source page</summary><div class="document">${renderMarkdown(source.markdown)}</div></details>
      <details><summary>Full destination page</summary><div class="document">${renderMarkdown(target.markdown)}</div></details>
    </article>`;
  document.querySelectorAll("[data-label]").forEach((button) => {
    button.addEventListener("click", () => setLabel(button.dataset.label));
  });
  document.getElementById("notes").addEventListener("input", debounce(saveNotes, 250));
  updateChrome();
}

function updateChrome() {
  const labeled = decisions.filter((decision) => currentLabel(decision.id).label).length;
  document.getElementById("counter").textContent = `${filtered.length ? index + 1 : 0} of ${filtered.length}`;
  document.getElementById("progress").style.width = `${decisions.length ? (labeled / decisions.length) * 100 : 0}%`;
  document.getElementById("progress-label").textContent = `${labeled} of ${decisions.length} labeled`;
  document.getElementById("previous").disabled = index <= 0;
  document.getElementById("next").disabled = index >= filtered.length - 1;
}

function setLabel(name) {
  if (!filtered.length) return;
  const decision = filtered[index];
  if (name === "pass" && !decision.anchor) return;
  const previous = { ...currentLabel(decision.id) };
  history.push({ id: decision.id, previous });
  labels[decision.id] = { label: name, notes: document.getElementById("notes").value };
  saveLabels();
  applyFilters(filtered[index + 1]?.id);
}

function saveNotes() {
  if (!filtered.length) return;
  const decision = filtered[index];
  labels[decision.id] = { ...currentLabel(decision.id), notes: document.getElementById("notes").value };
  saveLabels();
}

function undo() {
  const action = history.pop();
  if (!action) return;
  labels[action.id] = action.previous;
  saveLabels();
  applyFilters(action.id);
}

function navigate(delta) {
  index = Math.max(0, Math.min(filtered.length - 1, index + delta));
  render();
}

function exportLabels() {
  const labeledDecisions = decisions.filter((decision) => labels[decision.id]?.label);
  const labeledIds = new Set(labeledDecisions.map((decision) => decision.id));
  const exportedLabels = Object.fromEntries(
    Object.entries(labels).filter(([identifier]) => labeledIds.has(identifier)),
  );
  const body = JSON.stringify(
    {
      schema_version: data.schema_version,
      report_hash: data.report_hash,
      decisions: labeledDecisions,
      labels: exportedLabels,
    },
    null,
    2,
  );
  const link = document.createElement("a");
  link.href = URL.createObjectURL(new Blob([body], { type: "application/json" }));
  link.download = "doc-link-review.json";
  link.click();
  URL.revokeObjectURL(link.href);
}

function debounce(fn, wait) {
  let timer;
  return (...args) => {
    clearTimeout(timer);
    timer = setTimeout(() => fn(...args), wait);
  };
}

for (const id of ["queue", "status"]) {
  document.getElementById(id).addEventListener("change", () => applyFilters());
}
document.getElementById("search").addEventListener(
  "input",
  debounce(() => {
    index = 0;
    applyFilters();
  }, 150),
);
document.getElementById("previous").addEventListener("click", () => navigate(-1));
document.getElementById("next").addEventListener("click", () => navigate(1));
document.getElementById("export").addEventListener("click", exportLabels);
document.addEventListener("keydown", (event) => {
  if (event.target.matches("textarea, input, select")) return;
  if (event.key === "ArrowLeft") navigate(-1);
  if (event.key === "ArrowRight") navigate(1);
  if (event.key === "1") setLabel("pass");
  if (event.key === "2") setLabel("fail");
  if (event.key.toLowerCase() === "d") setLabel("defer");
  if (event.key.toLowerCase() === "u") undo();
});
applyFilters();
