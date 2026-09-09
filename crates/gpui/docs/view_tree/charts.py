import sys, os, csv
import matplotlib; matplotlib.use("Agg")
import matplotlib.pyplot as plt
import numpy as np
# Usage: python charts.py <output dir> [matrix.csv]  (needs matplotlib)
# Reads matrix.sh's rows: fixture,main,branch,change_pct,lo,hi,main_rss,branch_rss,main_rss_max,branch_rss_max
out = sys.argv[1]
matrix = sys.argv[2] if len(sys.argv) > 2 else os.path.join(os.path.dirname(__file__), "matrix.csv")
LABELS = {  # matrix.sh fixture filter -> chart label
 "Workbench/update/row": "Workbench/row", "Workbench/update/editor": "Workbench/editor",
 "Workbench/update/mixed": "Workbench/mixed", "Workbench/update/full": "Workbench/full",
 "^editor_render$": "editor_render", "^editor_render_with_editorconfig$": "editorconfig",
 "^open_editor_with_one_long_line$": "one long line",
 "Multi-cursor input/cursors/1000$": "multi-cursor 1000", "Multi-cursor input/cursors/10000": "multi-cursor 10000",
 "Markdown render/min_bytes/5000$": "Markdown 5000", "Markdown render/min_bytes/10000": "Markdown 10000",
 "Markdown render/min_bytes/50000": "Markdown 50000",
 "Elements/all dirty/256": "Elements/all dirty/256", "Elements/all dirty/2048": "Elements/all dirty/2048",
 "Elements/all dirty/8192": "Elements/all dirty/8192",
 "Elements/incremental/256": "Elements/incr/256", "Elements/incremental/2048": "Elements/incr/2048",
 "Elements/incremental/8192": "Elements/incr/8192",
 "Siblings/all dirty/64": "Siblings/64", "Siblings/all dirty/256": "Siblings/256", "Siblings/all dirty/1024": "Siblings/1024",
}
UNITS = {"ns": 1e-3, "µs": 1, "us": 1, "ms": 1e3, "s": 1e6}
def micros(text):
    value, unit = text.split()
    return float(value) * UNITS[unit]
D, CI, RSS = {}, {}, {}  # label -> (main, branch) µs; (lo, hi) %; (main, branch) MB after the first measurement
with open(matrix, encoding="utf-8") as rows:
    for row in csv.reader(rows):
        if len(row) < 6 or row[0] not in LABELS:
            continue
        label = LABELS[row[0]]
        D[label] = (micros(row[1]), micros(row[2]))
        CI[label] = (float(row[4]), float(row[5]))
        if len(row) >= 8 and row[6] and row[7]:
            RSS[label] = (float(row[6]), float(row[7]))
D = {label: D[label] for label in LABELS.values() if label in D}
plt.rcParams.update({"font.size": 10, "figure.dpi": 130})

# 1. Overview: % change per fixture. Siblings is left out: a percentage of a frame that is
# nothing but trivial nodes says little, so it is reported as the flat cost per node (chart 2).
names = [n for n in D if not n.startswith("Siblings")]; pct = [100*(D[n][1]-D[n][0])/D[n][0] for n in names]
lo = [max(0,pct[i]-CI[n][0]) for i,n in enumerate(names)]; hi = [max(0,CI[n][1]-pct[i]) for i,n in enumerate(names)]
colors = ["#2a9d8f" if p < 0 else "#e76f51" for p in pct]
fig, ax = plt.subplots(figsize=(9, 6.2))
y = np.arange(len(names))
ax.barh(y, pct, color=colors, xerr=[lo,hi], error_kw=dict(ecolor="#444", capsize=2, lw=0.8))
ax.set_yticks(y); ax.set_yticklabels(names); ax.invert_yaxis()
ax.axvline(0, color="k", lw=0.8)
for sep in [3.5, 8.5, 11.5]: ax.axhline(sep, color="#bbb", lw=0.6, ls="--")
ax.text(-66, 1.5, "reuse fires", va="center", color="#2a9d8f")
ax.text(-66, 6, "Zed-shaped, all dirty", va="center", color="#555")
ax.text(-66, 10, "one view, all elements dirty", va="center", color="#555")
ax.text(-66, 14.5, "per-element cost (synthetic:\none view, N trivial divs)", va="center", color="#e76f51")
ax.set_xlabel("frame time change vs main (%), Criterion median, 95% CI")
ax.set_title("GPUI view tree vs main — paired Criterion runs, same machine")
for i,p in enumerate(pct): ax.text(p + (0.5 if p>=0 else -0.5), i, f"{p:+.1f}%", va="center", ha="left" if p>=0 else "right", fontsize=8)
ax.set_xlim(-72, 12)
fig.tight_layout(); fig.savefig(f"{out}/overview.png"); plt.close(fig)

# 2. Siblings: fixed per-node cost
n = np.array([64,256,1024]); m = np.array([D[f"Siblings/{k}"][0] for k in n]); b = np.array([D[f"Siblings/{k}"][1] for k in n])
fig, (ax1, ax2) = plt.subplots(1,2, figsize=(10,4))
ax1.plot(n, m, "o-", label="main"); ax1.plot(n, b, "o-", label="branch"); ax1.set_xscale("log", base=2); ax1.set_yscale("log")
ax1.set_xlabel("dirty views (nodes) per frame"); ax1.set_ylabel("frame time (µs)"); ax1.legend(); ax1.set_title("Siblings/all dirty: N trivial views, all notified")
per = (b-m)/n
ax2.bar([str(k) for k in n], per, color="#e76f51"); ax2.set_ylabel("µs per dirty node per frame"); ax2.set_title("engine tax per dirty node (branch − main) / N")
for i,v in enumerate(per): ax2.text(i, v+0.01, f"{v:.2f}", ha="center")
ax2.set_ylim(0, 0.9)
fig.tight_layout(); fig.savefig(f"{out}/per_node.png"); plt.close(fig)

# 3. Elements: proportional per-element cost
n = np.array([256,2048,8192])
fig, (ax1, ax2) = plt.subplots(1,2, figsize=(10,4))
for key,label,c in [("all dirty","all dirty (layout tree cleared)","#e76f51"),("incr","incremental (one clean sibling)","#f4a261")]:
    m = np.array([D[f"Elements/{key}/{k}"][0] for k in n]); b = np.array([D[f"Elements/{key}/{k}"][1] for k in n])
    ax1.plot(n, m, "o--", color="#777", label=f"main, {label}" if key=="all dirty" else None); ax1.plot(n, b, "o-", color=c, label=f"branch, {label}")
    ax2.plot(n, (b-m)/n, "o-", color=c, label=label)
ax1.set_xscale("log", base=2); ax1.set_yscale("log"); ax1.set_xlabel("elements in the one dirty view"); ax1.set_ylabel("frame time (µs)"); ax1.legend(fontsize=8); ax1.set_title("Elements: one view, N id'd divs with a glyph")
ax2.set_xscale("log", base=2); ax2.set_xlabel("elements"); ax2.set_ylabel("µs per element per frame"); ax2.set_ylim(0,0.3); ax2.legend(fontsize=8); ax2.set_title("engine tax per element (branch − main) / N")
ax2.axhline(2.9, color="none")
fig.tight_layout(); fig.savefig(f"{out}/per_element.png"); plt.close(fig)

# 4. Model vs Zed-shaped fixtures
fig, ax = plt.subplots(figsize=(8,4))
labels = ["Workbench/full\n(48 rows, 4 panels,\neditor; all dirty)", "editor_render", "Markdown\n10000", "Elements/all dirty\n2048"]
vals = [100*(D[k][1]-D[k][0])/D[k][0] for k in ["Workbench/full","editor_render","Markdown 10000","Elements/all dirty/2048"]]
ax.bar(labels, vals, color=["#2a9d8f","#999","#e9c46a","#e76f51"])
ax.axhline(0, color="k", lw=0.8); ax.set_ylabel("% vs main, everything dirty")
for i,v in enumerate(vals): ax.text(i, v + (0.5 if v>=0 else -1.5), f"{v:+.1f}%", ha="center")
ax.set_title("All-dirty tax by shape: ≈≈0.55 µs × nodes + ≈0.08 µs × elements − retention wins", fontsize=10)
fig.tight_layout(); fig.savefig(f"{out}/tax_by_shape.png"); plt.close(fig)

# 5. Process resident memory after the first measurement
if RSS:
    names = [n for n in D if n in RSS]
    fig, ax = plt.subplots(figsize=(9, 7))
    y = np.arange(len(names)); h = 0.38
    ax.barh(y - h/2, [RSS[n][0] for n in names], h, color="#999", label="main")
    ax.barh(y + h/2, [RSS[n][1] for n in names], h, color="#2a9d8f", label="branch")
    for i, n in enumerate(names):
        ax.text(max(RSS[n]) + 1, i, f"{RSS[n][1]-RSS[n][0]:+.1f} MB", va="center", fontsize=8)
    ax.set_yticks(y); ax.set_yticklabels(names); ax.invert_yaxis(); ax.legend(loc="lower right")
    ax.set_xlabel("bench process resident set size after the first measurement (MB)")
    ax.set_title("Process memory, main vs branch, same fixture in the same harness")
    fig.tight_layout(); fig.savefig(f"{out}/memory.png"); plt.close(fig)
print("ok")
