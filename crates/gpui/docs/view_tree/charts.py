import sys, os
import matplotlib; matplotlib.use("Agg")
import matplotlib.pyplot as plt
import numpy as np
out = sys.argv[1]
# Usage: python charts.py <output dir>  (needs matplotlib)
# fixture: (main, branch) in µs; Criterion medians from matrix.csv (main 5a9b9558db vs 46c3ffa866)
D = {
 "Workbench/row":      (768.75, 323.76),
 "Workbench/editor":   (1543.8, 1112.0),
 "Workbench/mixed":    (1204.2, 796.31),
 "Workbench/full":     (1601.4, 1480.7),
 "editor_render":      (693.58, 697.08),
 "editorconfig":       (1564.9, 1590.5),
 "one long line":      (832.31, 812.67),
 "multi-cursor 1000":  (71074, 71119),
 "multi-cursor 10000": (659460, 665250),
 "Markdown 5000":      (1152.8, 1178.4),
 "Markdown 10000":     (1605.5, 1657.2),
 "Markdown 50000":     (6461.1, 6698.1),
 "Elements/all dirty/256":   (743.45, 770.35),
 "Elements/all dirty/2048":  (6712.6, 6969.7),
 "Elements/all dirty/8192":  (23658, 24752),
 "Elements/incr/256":   (761.65, 782.53),
 "Elements/incr/2048":  (6811.0, 7191.0),  # mean of three runs: +7.6, +6.4, +3.6
 "Elements/incr/8192":  (24079, 24852),
 "Siblings/64":   (220.23, 254.80),
 "Siblings/256":  (760.36, 889.40),
 "Siblings/1024": (3196.7, 3890.8),
}
CI = {  # (lo, hi) % from criterion
 "Workbench/row": (-59.5,-57.8), "Workbench/editor": (-30.2,-28.4), "Workbench/mixed": (-34.9,-33.2), "Workbench/full": (-9.7,-6.5),
 "editor_render": (-0.7,1.5), "editorconfig": (-0.0,3.5), "one long line": (-3.0,1.1), "multi-cursor 1000": (-5.5,6.6), "multi-cursor 10000": (0.6,1.2),
 "Markdown 5000": (1.2,2.5), "Markdown 10000": (1.9,4.4), "Markdown 50000": (-1.9,8.4),
 "Elements/all dirty/256": (3.4,4.3), "Elements/all dirty/2048": (3.3,5.0), "Elements/all dirty/8192": (3.4,5.7),
 "Elements/incr/256": (0.7,2.4), "Elements/incr/2048": (3.6,7.6), "Elements/incr/8192": (1.3,3.9),
 "Siblings/64": (14.8,15.9), "Siblings/256": (16.9,18.3), "Siblings/1024": (21.9,26.6),
}
plt.rcParams.update({"font.size": 10, "figure.dpi": 130})

# 1. Overview: % change per fixture
names = list(D); pct = [100*(b-m)/m for m,b in D.values()]
lo = [max(0,pct[i]-CI[n][0]) for i,n in enumerate(names)]; hi = [max(0,CI[n][1]-pct[i]) for i,n in enumerate(names)]
colors = ["#2a9d8f" if p < 0 else "#e76f51" for p in pct]
fig, ax = plt.subplots(figsize=(9, 7))
y = np.arange(len(names))
ax.barh(y, pct, color=colors, xerr=[lo,hi], error_kw=dict(ecolor="#444", capsize=2, lw=0.8))
ax.set_yticks(y); ax.set_yticklabels(names); ax.invert_yaxis()
ax.axvline(0, color="k", lw=0.8)
for sep in [3.5, 8.5, 11.5, 17.5]: ax.axhline(sep, color="#bbb", lw=0.6, ls="--")
ax.text(-58, 1.5, "reuse fires", va="center", color="#2a9d8f")
ax.text(-58, 6, "Zed-shaped, all dirty", va="center", color="#555")
ax.text(-58, 10, "one view, all elements dirty", va="center", color="#555")
ax.text(-58, 14.5, "per-element cost (synthetic)", va="center", color="#e76f51")
ax.text(-58, 19, "per-node cost (synthetic worst case)", va="center", color="#e76f51")
ax.set_xlabel("frame time change vs main (%), Criterion median, 95% CI")
ax.set_title("GPUI view tree vs main — paired Criterion runs, same machine")
for i,p in enumerate(pct): ax.text(p + (1 if p>=0 else -1), i, f"{p:+.1f}%", va="center", ha="left" if p>=0 else "right", fontsize=8)
ax.set_xlim(-62, 32)
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
labels = ["Workbench/full\n(48 rows, 4 panels,\neditor; all dirty)", "editor_render", "Markdown\n10000", "Elements/all dirty\n2048", "Siblings\n1024"]
vals = [100*(D[k][1]-D[k][0])/D[k][0] for k in ["Workbench/full","editor_render","Markdown 10000","Elements/all dirty/2048","Siblings/1024"]]
ax.bar(labels, vals, color=["#2a9d8f","#999","#e9c46a","#e76f51","#e76f51"])
ax.axhline(0, color="k", lw=0.8); ax.set_ylabel("% vs main, everything dirty")
for i,v in enumerate(vals): ax.text(i, v + (0.5 if v>=0 else -1.5), f"{v:+.1f}%", ha="center")
ax.set_title("All-dirty tax by shape: ≈0.55 µs × nodes + ≈0.12 µs × elements − retention wins", fontsize=10)
fig.tight_layout(); fig.savefig(f"{out}/tax_by_shape.png"); plt.close(fig)
print("ok")
