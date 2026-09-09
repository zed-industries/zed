import sys, os, csv, json, re
import matplotlib; matplotlib.use("Agg")
import matplotlib.pyplot as plt
import numpy as np
# Usage: python complexity.py <output dir> [complexity.csv] [matrix.csv]  (needs matplotlib, numpy)
# Fits the frame cost model to complexity.sh's rows and writes complexity_fit.png,
# complexity_surface.png, complexity_fit_budget.png and complexity.html (sliders over the model).
#
# The model. An element is light (a quad), medium (a quad and a 3-glyph label) or heavy (a
# quad, a shadow and a 10-glyph label) — the three `P` settings of the sweep. Cost is linear
# in the number of elements of each kind, dirty (in a view that re-rendered) or clean (in a
# view that was replayed), plus a shared quadratic term in the total element count for the
# frame-wide work that grows faster than the scene (bounds tree, cache footprint):
#   main:   t = c + Σ_k n_k·m_k + q·(N/100)²          (everything is dirty on main)
#   branch: t = c + Σ_k (d_k·b_k + (n_k − d_k)·r_k) + q·(N/100)²
# Views enter only as the per-node cost the Siblings sweep measured (≈0.55 µs per dirty
# node), which is below this fit's resolution, so it is added as a constant, not fitted.
out = sys.argv[1]
here = os.path.dirname(os.path.abspath(__file__))
csv_path = sys.argv[2] if len(sys.argv) > 2 else os.path.join(here, "complexity.csv")
matrix_path = sys.argv[3] if len(sys.argv) > 3 else os.path.join(here, "matrix.csv")
UNITS = {"ns": 1e-3, "µs": 1, "us": 1, "ms": 1e3, "s": 1e6}
def micros(text):
    value, unit = text.split()
    return float(value) * UNITS[unit]
KIND = {1: 0, 4: 1, 12: 2}; KIND_NAMES = ["light", "medium", "heavy"]
PER_DIRTY_NODE = 0.55  # µs, Siblings sweep

points = []  # (V, E, P, F, main_us, branch_us)
editor_points = []  # (editors, dirty, reshape, main_us, branch_us)
# The same fixtures with every view / editor wrapped in `.cached()`: what `main`'s own opt-in
# reuse (docked panels, pane items) costs for a clean view. On the branch `.cached()` changes
# nothing, which the branch column of these rows checks.
cached_points = []
cached_editor_points = []
with open(csv_path, encoding="utf-8") as rows:
    for row in csv.reader(rows):
        if not row or len(row) < 3:
            continue
        cached = row[0].rstrip("$").endswith("-cached")
        if m := re.search(r"v(\d+)-e(\d+)-p(\d+)-f(\d+)", row[0]):
            V, E, P, F = map(int, m.groups())
            (cached_points if cached else points).append((V, E, P, F, micros(row[1]), micros(row[2])))
        elif m := re.search(r"editors/k(\d+)-d(\d+)-(cursor|scroll)", row[0]):
            K, D, how = int(m.group(1)), int(m.group(2)), m.group(3)
            (cached_editor_points if cached else editor_points).append((K, D, how == "scroll", micros(row[1]), micros(row[2])))

def counts(V, E, P, F):
    dirty_views = 0 if F == 0 else max(1, -(-F * V // 100))
    dirty, clean = [0, 0, 0], [0, 0, 0]
    dirty[KIND[P]] = dirty_views * E; clean[KIND[P]] = (V - dirty_views) * E
    return dirty, clean, V * E

def weighted_fit(X, y):
    # Relative error matters on the page, so weight rows by 1/t.
    w = 1 / y
    coef, *_ = np.linalg.lstsq(X * w[:, None], y * w, rcond=None)
    pred = X @ coef
    return coef, pred, (pred - y) / y

y_main = np.array([p[4] for p in points]); y_branch = np.array([p[5] for p in points])
X_main = np.array([[1] + [d + c for d, c in zip(*counts(*p[:4])[:2])] + [(counts(*p[:4])[2] / 100) ** 2] for p in points])
X_branch = np.array([[1] + counts(*p[:4])[0] + counts(*p[:4])[1] + [(counts(*p[:4])[2] / 100) ** 2] for p in points])
coef_main, pred_main, err_main = weighted_fit(X_main, y_main)
coef_branch, pred_branch, err_branch = weighted_fit(X_branch, y_branch)
names_main = ["c", "light", "medium", "heavy", "q"]
names_branch = ["c", "dirty light", "dirty medium", "dirty heavy", "clean light", "clean medium", "clean heavy", "q"]
def describe(names, coef):
    return ", ".join(f"{n} {v:.2f}" for n, v in zip(names, coef))
print("main  :", describe(names_main, coef_main), "µs; |err| mean %.1f%% max %.1f%%" % (100 * abs(err_main).mean(), 100 * abs(err_main).max()))
print("branch:", describe(names_branch, coef_branch), "µs; |err| mean %.1f%% max %.1f%%" % (100 * abs(err_branch).mean(), 100 * abs(err_branch).max()))

# Editors: one view each, painting ~40 lines of code directly. Fitted on the points where a
# dirty editor scrolled (its visible lines reshaped) or nothing was dirty; a cursor move
# turned out to cost far more than a scroll and to grow faster than the number of editors
# moving, which is editor behaviour rather than rendering, so those points are reported but
# not fitted.
#   main:   t = c + K·e + D·x        (every editor re-renders; x is the reshaping extra)
#   branch: t = c + (K − D)·e_clean + D·e_dirty
editor_fit = None
fit_points = [p for p in editor_points if p[1] == 0 or p[2]]
if len(fit_points) >= 4:
    Xm = np.array([[1, K, D] for K, D, *_ in fit_points]); Xb = np.array([[1, K - D, D] for K, D, *_ in fit_points])
    ym = np.array([p[3] for p in fit_points]); yb = np.array([p[4] for p in fit_points])
    em, _, erm = weighted_fit(Xm, ym); eb, _, erb = weighted_fit(Xb, yb)
    editor_fit = dict(main_each=em[1], main_reshape=em[2], branch_clean=eb[1], branch_dirty=eb[2])
    print("editors: main %.0f µs each, +%.0f when scrolled; branch clean %.0f, dirty (scrolled) %.0f; |err| mean %.1f%% / %.1f%%"
          % (em[1], em[2], eb[1], eb[2], 100 * abs(erm).mean(), 100 * abs(erb).mean()))
    for K, D, reshape, m, b in editor_points:
        if not reshape and D > 0:
            print(f"  cursor moves (not fitted): k{K}-d{D}: main {m:.0f} µs, branch {b:.0f} µs")

# main with `.cached()`. A notified cached view re-renders at main's uncached per-element cost
# plus a surcharge u for the caching bookkeeping (at f = 100% the cached scene is ~30% dearer
# than the uncached one); a clean one is replayed at r_k per element; and the frame-wide
# superlinear term has its own coefficient q', with main's constant c kept:
#   t = c + Σ_k d_k·(m_k + u) + Σ_k (n_k − d_k)·r_k + q'·(N/100)²
# Likewise an idle cached editor costs e_replay against the uncached fit's e (+ x when scrolled)
# for the D changed ones.
main_cache = None
if len(cached_points) >= 5:
    X, y = [], []
    for p in cached_points:
        dirty, clean, N = counts(*p[:4])
        X.append(clean + [sum(dirty), (N / 100) ** 2])
        y.append(p[4] - coef_main[0] - sum(coef_main[1 + k] * d for k, d in enumerate(dirty)))
    rc, _, err_cached = weighted_fit(np.array(X, float), np.array(y))
    branch_cached_delta = [(p[5] - next((q[5] for q in points if q[:4] == p[:4]), p[5])) / p[5] for p in cached_points]
    main_cache = dict(replay=[max(0.0, float(v)) for v in rc[:3]], dirty_extra=float(rc[3]), q=float(rc[4]), editor_replay=None)
    print("main cached: replay per clean light %.2f, medium %.2f, heavy %.2f µs; dirty cached element +%.2f; q' %.2f; |err| mean %.1f%% max %.1f%%; branch with .cached() differs from uncached by %.1f%% mean"
          % (*rc[:3], rc[3], rc[4], 100 * abs(err_cached).mean(), 100 * abs(err_cached).max(), 100 * np.mean(np.abs(branch_cached_delta))))
    if editor_fit and len(cached_editor_points) >= 2:
        Xe = np.array([[1, K - D] for K, D, *_ in cached_editor_points])
        ye = np.array([m - D * (editor_fit["main_each"] + (editor_fit["main_reshape"] if reshape else 0)) for K, D, reshape, m, _ in cached_editor_points])
        ec, _, erc = weighted_fit(Xe, ye)
        main_cache["editor_replay"] = float(ec[1])
        print("main cached editors: idle editor replayed for %.0f µs (c' %.0f); |err| mean %.1f%%; branch idle editor %.0f µs"
              % (ec[1], ec[0], 100 * abs(erc).mean(), editor_fit["branch_clean"]))
        for K, D, reshape, m, b in cached_editor_points:
            print(f"  k{K}-d{D}-{'scroll' if reshape else 'cursor'}-cached: main {m:.0f} µs, branch {b:.0f} µs")

def predict_main(n):  # n = [light, medium, heavy]
    N = sum(n)
    return coef_main[0] + sum(coef_main[1 + k] * n[k] for k in range(3)) + coef_main[4] * (N / 100) ** 2
def predict_branch(n, fraction, dirty_views=0):
    N = sum(n)
    t = coef_branch[0] + coef_branch[7] * (N / 100) ** 2 + PER_DIRTY_NODE * dirty_views
    for k in range(3):
        d = n[k] * fraction
        t += coef_branch[1 + k] * d + coef_branch[4 + k] * (n[k] - d)
    return t

plt.rcParams.update({"font.size": 9, "figure.dpi": 130})

# 1. Fit quality: measured vs predicted per point.
labels = [f"v{V}-e{E}-p{P}-f{F}" for V, E, P, F, *_ in points]
fig, ax = plt.subplots(figsize=(10, 5))
x = np.arange(len(points)); w = 0.2
ax.bar(x - 1.5 * w, y_main / 1000, w, color="#999", label="main measured")
ax.bar(x - 0.5 * w, pred_main / 1000, w, color="#ccc", label="main model (|err| mean %.0f%%, max %.0f%%)" % (100 * abs(err_main).mean(), 100 * abs(err_main).max()))
ax.bar(x + 0.5 * w, y_branch / 1000, w, color="#2a9d8f", label="branch measured")
ax.bar(x + 1.5 * w, pred_branch / 1000, w, color="#9fd8d0", label="branch model (|err| mean %.0f%%, max %.0f%%)" % (100 * abs(err_branch).mean(), 100 * abs(err_branch).max()))
ax.set_xticks(x); ax.set_xticklabels(labels, rotation=45, ha="right"); ax.set_ylabel("frame time (ms)"); ax.legend(fontsize=8)
ax.set_title("Complexity sweep: measured frame time and the fitted cost model (v views · e elements per view · p primitives per element · f% of views changing)")
fig.tight_layout(); fig.savefig(f"{out}/complexity_fit.png"); plt.close(fig)

# 2. Frame time against the share of elements changing, two scene sizes. main's surface is
#    flat: everything is redrawn whatever changed; the branch meets it at 100%.
fig, axes = plt.subplots(1, 2, figsize=(10, 4))
for ax, (n, title) in zip(axes, [([0, 2048, 0], "2048 medium elements (16 views × 128; ~8k primitives)"),
                                 ([0, 8192, 0], "8192 medium elements (16 views × 512; ~33k primitives)")]):
    F = np.linspace(0, 1, 101)
    branch = np.array([predict_branch(n, f, dirty_views=16 * f) for f in F]) / 1000
    main = np.full_like(F, predict_main(n) / 1000)
    ax.fill_between(F * 100, 0, main, color="#bbb", alpha=0.6, label="main: everything redrawn")
    ax.fill_between(F * 100, 0, branch, color="#2a9d8f", alpha=0.7, label="branch: clean views replayed")
    for budget, name in [(8.3, "120 Hz"), (16.7, "60 Hz")]:
        if budget < max(main.max(), branch.max()) * 1.1:
            ax.axhline(budget, color="#e76f51", lw=0.8, ls="--"); ax.text(1, budget, name, va="bottom", color="#e76f51", fontsize=8)
    ax.set_xlabel("elements changing per frame (%)"); ax.set_ylabel("frame time (ms)"); ax.set_title(title, fontsize=9); ax.legend(fontsize=8, loc="upper left")
    ax.set_ylim(0, max(main.max(), branch.max()) * 1.15)
fig.tight_layout(); fig.savefig(f"{out}/complexity_surface.png"); plt.close(fig)

# 3. How many medium elements fit a 120 Hz frame, against the share changing.
def fits(predict, budget_us):
    lo, hi = 0, 1 << 17
    while lo < hi:
        mid = (lo + hi + 1) // 2
        if predict(mid) <= budget_us: lo = mid
        else: hi = mid - 1
    return lo
fig, ax = plt.subplots(figsize=(6, 3.6))
F = np.linspace(0, 1, 51)
for budget, style in [(8300, "-"), (16700, "--")]:
    ax.plot(F * 100, [fits(lambda n: predict_branch([0, n, 0], f, dirty_views=16 * f), budget) for f in F], "#2a9d8f", ls=style, label=f"branch, {budget/1000:g} ms budget")
    ax.plot(F * 100, [fits(lambda n: predict_main([0, n, 0]), budget)] * len(F), "#999", ls=style, label=f"main, {budget/1000:g} ms budget")
ax.set_xlabel("elements changing per frame (%)"); ax.set_ylabel("medium elements that fit"); ax.set_yscale("log"); ax.legend(fontsize=8)
ax.set_title("Scene size a frame budget affords (medium elements: quad + 3 glyphs)", fontsize=9)
fig.tight_layout(); fig.savefig(f"{out}/complexity_fit_budget.png"); plt.close(fig)

# Measured Zed-shaped fixtures from matrix.csv, for the page's table.
measured = []
if os.path.exists(matrix_path):
    with open(matrix_path, encoding="utf-8") as rows:
        for row in csv.reader(rows):
            if row and row[0].startswith("Workbench/") and len(row) >= 4:
                measured.append((row[0], row[1], row[2], row[3]))

html = """<!doctype html><meta charset="utf-8"><title>GPUI view tree: frame cost model</title>
<style>body{font:14px system-ui;margin:2em;max-width:940px;color:#222}label{display:block;margin:.35em 0}input[type=range]{width:260px;vertical-align:middle}
table{border-collapse:collapse;margin:1em 0}td,th{padding:.3em .8em;border-bottom:1px solid #ddd;text-align:right}th:first-child,td:first-child{text-align:left}
canvas{border:1px solid #ddd;margin-top:1em}.n{color:#2a9d8f}.m{color:#777}small{color:#666}button{margin:.2em .3em .2em 0}
.controls{display:flex;gap:1.5em;align-items:flex-start}.controls>div{flex:none}.controls fieldset{flex:1;min-width:260px;margin:0;padding:.4em 1em .6em;border:1px solid #ddd}
.controls fieldset input[type=range]{width:170px}.controls fieldset p{margin:.3em 0}</style>
<h2>GPUI view tree — frame cost model</h2>
<p>Presets: <span id=presets></span></p>
<style>button.active{background:#2a9d8f;color:#fff;border-color:#2a9d8f}</style>
<p><small>The frame budget at 120 Hz is 8.3 ms, shared by every window and by everything else on the main thread; the 4 ms line is the half of it a draw should stay under.</small></p>
<div class=controls><div>
<label>divs (a quad) <input id=L type=range min=0 max=4000 step=10 value=200> <span id=Lv></span></label>
<label>buttons (quad + 3-glyph label) <input id=M type=range min=0 max=1500 step=10 value=300> <span id=Mv></span></label>
<label>cards (quad + shadow + 10-glyph label) <input id=H type=range min=0 max=400 step=5 value=20> <span id=Hv></span></label>
<label>editors (~40 lines of code each) <input id=K type=range min=0 max=8 value=3> <span id=Kv></span></label>
<label>… of which changing <input id=D type=range min=0 max=8 value=1> <span id=Dv></span></label>
<label style="margin-left:1.2em"><input id=R type=checkbox> … showing new text (lines reshaped, e.g. scrolling)</label>
<label>views <input id=V type=range min=1 max=128 value=20> <span id=Vv></span></label>
<label>elements changing per frame <input id=F type=range min=0 max=100 value=10> <span id=Fv></span>%%</label>
</div>
<fieldset><legend><label style="display:inline"><input id=MC type=checkbox> <code>main</code> caches editors and panels, as Zed ships it</label></legend>
<label>cached panels<br><input id=PN type=range min=1 max=12 value=3> <span id=PNv></span></label>
<label>share of divs / buttons / cards inside them<br><input id=PS type=range min=0 max=100 value=60> <span id=PSv></span>%%</label>
<label>share of the change landing in the panel being worked in<br><input id=LOC type=range min=0 max=100 value=80> <span id=LOCv></span>%%</label>
<p id=mcnote style="color:#666"></p>
<p id=mcwarn style="color:#b00"></p></fieldset></div>
<canvas id=c width=900 height=340></canvas>
<p style="margin:.4em 0 0"><small>y axis top <input id=YS type=range min=2 max=20 step=0.1 value=4> <span id=YSv></span> ms <label style="display:inline;margin-left:1em"><input id=fit type=checkbox checked> fit the curves</label></small></p>
<table><tr><th></th><th>main</th><th>branch</th></tr>
<tr><td>frame time <small id=tmnote></small></td><td class=m id=tm></td><td class=n id=tb></td></tr>
<tr><td>scale of this scene that fits a 120 Hz frame, 8.3 ms (× the element counts above)</td><td class=m id=sm></td><td class=n id=sb></td></tr>
<tr><td>… i.e. total elements</td><td class=m id=em></td><td class=n id=eb></td></tr>
<tr><td>share changing up to which the frame fits 8.3 ms</td><td class=m id=fm></td><td class=n id=fb></td></tr></table>
<h3>The model</h3>
<p>Element kinds <i>k</i> ∈ {div, button, card} with counts <i>n<sub>k</sub></i>, <i>N</i> = Σ <i>n<sub>k</sub></i>; a share <i>f</i> of the elements and <i>D</i> of the <i>K</i> editors change per frame.</p>
<pre id=formula></pre>
<h3>Measured: the Zed-shaped <code>Workbench</code> fixtures</h3>
<table><tr><th>fixture</th><th>main</th><th>branch</th><th>change</th></tr>%(measured)s</table>
<p><small>Model fitted to <code>complexity.csv</code> (%(npoints)d element points, mean error %(errm).0f%% / %(errb).0f%%; %(neditors)d editor points); see <code>view_tree.md</code>, "Scene complexity". Coefficients (µs): main — %(cm)s; branch — %(cb)s; editors — %(editors_text)s.</small></p>
<script>
const M=%(coef_main)s, B=%(coef_branch)s, NODE=%(node)s, ED=%(editors)s;
// main's own caching (reuse_paint on cached panels and pane items), measured with the
// `-cached` fixture variants; falls back to the branch's replay costs as placeholders when
// those rows are missing from the CSV.
const MAIN_CACHE=%(main_cache)s||{measured:false, replay:[B[4],B[5],B[6]], editor_replay:ED?ED.branch_clean:0};
let K=0,D=0,R=false,preset=null,MC=false,PN=3,PS=0.6,LOC=0.8,hover=null;
// Where this frame's change lands: a share LOC of it in the panel being worked in (plus the
// uncached area), the rest spread uniformly. The working area saturates softly (tanh) rather
// than at a hard cap, so change spills into the other panels gradually as f grows instead of
// all at once when the working area is full. Each panel is clean iff none of its elements
// changed; the expected replay is the average over the active and the other P−1 panels.
function cacheStats(n,f){const N=n[0]+n[1]+n[2];const inside=n.map(x=>x*PS);const per=inside.reduce((a,b)=>a+b,0)/PN;
 const working=N>0?(per+(N-inside.reduce((a,b)=>a+b,0)))/N:1;const absorbed=working>0?working*Math.tanh(LOC*f/working):0;const fOther=Math.max(0,f-absorbed);const fActive=Math.min(1,fOther+(working>0?LOC*f/working:0));
 const pActive=Math.pow(1-fActive,per),pOther=Math.pow(1-fOther,per);const pClean=(pActive+(PN-1)*pOther)/PN;return {inside,per,pClean,pActive,pOther,fActive,fOther};}
const us=v=>v.toFixed(2);
document.getElementById('formula').textContent=
`main    t = ${us(M[0])} + ${us(M[1])}·n_div + ${us(M[2])}·n_button + ${us(M[3])}·n_card + ${us(M[4])}·(N/100)²`+
(ED?`\n          + ${ED.main_each.toFixed(0)}·K + ${ED.main_reshape.toFixed(0)}·D·[reshaped]`:'')+
`\n  with main's caching on: elements inside the P cached panels are replayed at r_main when the panel is clean,`+
`\n          and re-rendered at ${MAIN_CACHE.dirty_extra!==undefined?'+'+us(MAIN_CACHE.dirty_extra)+' over the uncached cost':'the uncached cost'} when it is not;`+
`\n          P(clean) = (1 − f′)^(elements per panel), f′ the panel's own share of the change (a share λ lands in the panel being worked in);  editors: ${MAIN_CACHE.editor_replay.toFixed(0)}·(K − D) + ${ED?ED.main_each.toFixed(0):'?'}·D`+
`\n          r_main = (${MAIN_CACHE.replay.map(us).join(', ')}) per div, button, card${MAIN_CACHE.q!==undefined?', q′ = '+us(MAIN_CACHE.q)+' inside the panels':''}${MAIN_CACHE.measured?'   (measured: the -cached fixtures)':'   ← placeholders'}`+
`\n\nbranch  t = ${us(B[0])} + (${us(B[1])}·n_div + ${us(B[2])}·n_button + ${us(B[3])}·n_card)·f        dirty: rendered`+
`\n          + (${us(B[4])}·n_div + ${us(B[5])}·n_button + ${us(B[6])}·n_card)·(1 − f)   clean: replayed`+
`\n          + ${us(B[7])}·(N/100)² + ${NODE}·⌈f·views⌉`+
(ED?`\n          + ${ED.branch_clean.toFixed(0)}·(K − D) + ${(R?ED.branch_dirty:ED.branch_clean).toFixed(0)}·D`:'')+
`\n\n(µs; least squares on relative error over the sweep; main redraws every element every frame)`;
function editorsMain(){return ED?K*ED.main_each+(R?D*ED.main_reshape:0):0;}
function editorsBranch(){return ED?(K-D)*ED.branch_clean+D*(R?ED.branch_dirty:ED.branch_clean):0;}
function tmain(n,f,mode){const N=n[0]+n[1]+n[2];let t=M[0]+M[4]*(N/100)**2;
if(MC){const {inside}=cacheStats(n,f);const pClean=mode==='best'?1:mode==='worst'?0:cacheStats(n,f).pClean;
 // The quadratic term is main's own outside the panels and the cached fit's inside them.
 if(MAIN_CACHE.q!==undefined)t+=(MAIN_CACHE.q-M[4])*PS*(N/100)**2;
 for(let k=0;k<3;k++){const clean=inside[k]*pClean,dirtyCached=inside[k]-clean;t+=M[1+k]*(n[k]-inside[k])+(M[1+k]+(MAIN_CACHE.dirty_extra||0))*dirtyCached+MAIN_CACHE.replay[k]*clean;}
 t+=ED?(K-D)*MAIN_CACHE.editor_replay+D*(ED.main_each+(R?ED.main_reshape:0)):0;}
else{t+=M[1]*n[0]+M[2]*n[1]+M[3]*n[2]+editorsMain();}
return t;}
function tbranch(n,f,V){const N=n[0]+n[1]+n[2];let t=B[0]+B[7]*(N/100)**2+NODE*(f==0?0:Math.max(1,Math.ceil(f*V)))+editorsBranch();
for(let k=0;k<3;k++){const d=n[k]*f;t+=B[1+k]*d+B[4+k]*(n[k]-d);}return t;}
function scale(f,budget){let lo=0,hi=1024;for(let i=0;i<40;i++){const mid=(lo+hi)/2;if(f(mid)<=budget*1000)lo=mid;else hi=mid;}return lo;}
const $=id=>document.getElementById(id);
function fmt(us){return us>=1000?(us/1000).toFixed(2)+' ms':us.toFixed(0)+' µs';}
// [divs, buttons, cards, views, f%%, editors, editors changing, reshaped, main caches?, cached panels, share inside them %%, share of change in the working panel %%]
const PRESETS={
 'Zed, typing':[200,300,20,20,3,3,1,false,true,3,60,100],
 'Zed, scrolling':[200,300,20,20,3,3,1,true,true,3,60,100],
 'Zed, resize':[200,300,20,20,100,3,3,true,true,3,60,0],
 'Big workspace':[600,800,60,60,5,6,1,false,true,4,60,95],
 'Busy UI':[400,1000,200,40,10,0,0,false,false,3,60,50],
 'Data table':[4000,600,0,50,5,0,0,false,false,3,60,50],
 'Dashboard':[3000,0,0,40,1,0,0,false,false,3,60,50],
 'Long list':[0,0,300,1,100,0,0,false,false,3,60,0],
};
const BUDGET=8.3;const BUDGETS=[[4,'4 ms'],[8.3,'8.3 ms (120 Hz)'],[16.7,'16.7 ms (60 Hz)']];
function label(g,text,x,y,color){g.font='12px system-ui';g.lineWidth=3;g.lineJoin='round';g.strokeStyle='rgba(255,255,255,.9)';g.strokeText(text,x,y);g.fillStyle=color;g.fillText(text,x,y);}
// Labels in the right margin sit at the height of the line they name; when two lines are close
// the labels are pushed apart (in y order) and a thin leader points back at the line.
function drawMarginLabels(g,items,x,gap){items.sort((a,b)=>a.y-b.y);for(let i=1;i<items.length;i++)items[i].y=Math.max(items[i].y,items[i-1].y+gap);
 for(const it of items){if(Math.abs(it.y-it.lineY)>1){g.save();g.setLineDash([]);g.lineWidth=1;g.strokeStyle=it.color;g.beginPath();g.moveTo(x-4,it.lineY);g.lineTo(x+2,it.y);g.stroke();g.restore();}label(g,it.text,x+4,it.y+4,it.color);}}
function draw(){const n=[+$('L').value,+$('M').value,+$('H').value],V=+$('V').value,F=+$('F').value,budget=BUDGET,f=F/100;
K=+$('K').value;$('D').max=K;D=Math.min(+$('D').value,K);$('D').value=D;R=$('R').checked;
MC=$('MC').checked;PN=+$('PN').value;PS=+$('PS').value/100;LOC=+$('LOC').value/100;$('PNv').textContent=PN;$('PSv').textContent=Math.round(PS*100);$('LOCv').textContent=Math.round(LOC*100);
{const {per,pClean,pActive,pOther,fActive,fOther}=cacheStats(n,f);const pct=p=>(100*p).toFixed(p>0&&p<0.01?2:0)+'%%';
 $('mcnote').textContent=MC?`each panel ≈ ${Math.round(per)} elements. At f = ${F}%% the panel being worked in sees ${pct(fActive)} of its elements change and stays clean with probability ${pct(pActive)}; each other panel sees ${pct(fOther)} and stays clean with probability ${pct(pOther)}. Expected ${(PN*pClean).toFixed(1)} of ${PN} panels replayed this frame.`:'';
 $('mcwarn').textContent=MC&&!MAIN_CACHE.measured?'placeholder coefficients: main’s replay cost is taken as the branch’s until the .cached() fixtures are measured':'';}
$('Lv').textContent=n[0];$('Mv').textContent=n[1];$('Hv').textContent=n[2];$('Vv').textContent=V;$('Fv').textContent=F;$('Kv').textContent=K;$('Dv').textContent=D;
const tm=tmain(n,f),tb=tbranch(n,f,V);$('tm').textContent=fmt(tm)+(MC?' (expected)':'');$('tb').textContent=fmt(tb);$('tmnote').textContent=MC?'— main’s is the expectation over which panels stayed clean; the band on the chart is its range':'';
const t0=tbranch(n,0,V),t1=tbranch(n,1,V);
const N=n[0]+n[1]+n[2];const sm=scale(s=>tmain(n.map(x=>x*s),f),budget),sb=scale(s=>tbranch(n.map(x=>x*s),f,V),budget);
$('sm').textContent=sm.toFixed(2)+'×';$('sb').textContent=sb.toFixed(2)+'×';$('em').textContent=Math.round(N*sm);$('eb').textContent=Math.round(N*sb);
// Where the branch curve crosses the budget (it is linear in f between 0 and 100%%).
const fBudget=t1===t0?null:(budget*1000-t0)/(t1-t0);
{let fm=null;for(let p=0;p<=100;p++){if(tmain(n,p/100)<=budget*1000)fm=p;else break;}$('fm').textContent=fm===null?'never':fm>=100?'always':fm+'%%';}$('fb').textContent=fBudget===null?(t0<=budget*1000?'always':'never'):fBudget>=1?'always':fBudget<=0?'never':Math.round(fBudget*100)+'%%';
// Break-even: the share of change above which the branch costs what main does.
let fEven=null;for(let p=0;p<=100;p++){if(tbranch(n,p/100,V)>=tmain(n,p/100)){fEven=p/100;break;}}if(fEven===null)fEven=1;
const cv=$('c'),g=cv.getContext('2d');g.clearRect(0,0,cv.width,cv.height);const pad=52,W=cv.width-2*pad-215,H=cv.height-2*pad;
const fitTop=Math.max(2000,Math.max(tmain(n,1),tb)*1.15);$('YS').max=Math.max(20,fitTop/1000).toFixed(1);if($('fit').checked){$('YS').value=(fitTop/1000).toFixed(1);}const ymax=(+$('YS').value)*1000;$('YSv').textContent=(+$('YS').value).toFixed(1);const X=p=>pad+W*p/100,Y=us=>pad+H*(1-us/ymax);
const mainAt=p=>tmain(n,p/100);const tmMax=Math.max(...Array.from({length:101},(_,p)=>mainAt(p)));
g.beginPath();g.moveTo(X(0),Y(0));for(let p=0;p<=100;p++)g.lineTo(X(p),Y(mainAt(p)));g.lineTo(X(100),Y(0));g.closePath();g.fillStyle='rgba(150,150,150,.5)';g.fill();
const margin=[];
if(MC){g.save();g.setLineDash([2,3]);g.lineWidth=1;g.strokeStyle='#777';for(const mode of ['best','worst']){g.beginPath();for(let p=0;p<=100;p++){const y=Y(tmain(n,p/100,mode));if(p===0)g.moveTo(X(p),y);else g.lineTo(X(p),y);}g.stroke();}g.restore();
 const best=tmain(n,f,'best'),worst=tmain(n,f,'worst');
 margin.push({lineY:Y(worst),y:Y(worst),text:'no panel clean · '+fmt(worst),color:'#666'},{lineY:Y(best),y:Y(best),text:'every panel clean · '+fmt(best),color:'#666'});}
g.beginPath();g.moveTo(X(0),Y(0));for(let p=0;p<=100;p++)g.lineTo(X(p),Y(tbranch(n,p/100,V)));g.lineTo(X(100),Y(0));g.closePath();g.fillStyle='rgba(42,157,143,.7)';g.fill();
g.lineWidth=1;g.strokeStyle='#e76f51';g.setLineDash([4,4]);for(const [ms,name] of BUDGETS){if(ms*1000<ymax){g.lineWidth=1;g.strokeStyle='#e76f51';g.setLineDash([4,4]);g.beginPath();g.moveTo(X(0),Y(ms*1000));g.lineTo(X(100),Y(ms*1000));g.stroke();const cross=t1===t0?null:(ms*1000-t0)/(t1-t0);const upTo=cross!==null&&cross>0&&cross<1?' · up to '+Math.round(cross*100)+'%%':'';margin.push({lineY:Y(ms*1000),y:Y(ms*1000),text:name+upTo,color:'#e76f51'});}}g.setLineDash([]);drawMarginLabels(g,margin,X(100)+4,15);
g.lineWidth=1;g.strokeStyle='#e76f51';g.beginPath();g.moveTo(X(F),Y(0));g.lineTo(X(F),Y(ymax/1.15));g.stroke();
if(F>6)label(g,'0%%',X(0)-8,Y(0)+16,'#333');if(F<94)label(g,'100%%',X(100)-14,Y(0)+16,'#333');label(g,'elements changing per frame',X(50)-80,Y(0)+32,'#333');{const t=fmt(ymax/1.15);label(g,t,pad-6-g.measureText(t).width,Y(ymax/1.15)+4,'#333');}label(g,'0',pad-6-g.measureText('0').width,Y(0)+4,'#333');
label(g,F+'%%',X(F)-10,Y(0)+16,'#e76f51');
const faster=tm/tb;const gain=faster>=1?faster.toFixed(1)+'× faster ('+Math.round(100*(1-tb/tm))+'%%)':(tb/tm).toFixed(2)+'× slower';
const branchText='branch '+fmt(tb)+' — '+gain;const bw=g.measureText(branchText).width+8;
const bx=F>55?X(F)-bw:X(F)+6;const by=(Y(tb)-Y(tm))<22?Y(tb)+18:Y(tb)-6;
{const worstY=MC?Y(tmain(n,f,'worst')):-99;const my=MC?(Math.abs(worstY-Y(tm))<18?Y(tm)+14:Y(tm)-6):Y(tmMax)-6;label(g,'main '+fmt(tm)+(MC?' expected':''),MC?(F>55?X(F)-150:X(F)+8):X(2),my,'#555');}label(g,branchText,bx,by,'#1b6f65');
for(const b of $('presets').children)b.classList.toggle('active',b.textContent===preset);
g.font='bold 13px system-ui';label(g,preset?preset:'custom scene',pad,pad-14,'#222');g.font='12px system-ui';
for(const [ms] of BUDGETS){const cross=t1===t0?null:(ms*1000-t0)/(t1-t0);if(cross!==null&&cross>0&&cross<1&&ms*1000<ymax){g.fillStyle='#e76f51';g.beginPath();g.arc(X(cross*100),Y(ms*1000),4,0,7);g.fill();}}
if(fEven!==null&&fEven>0&&fEven<1){g.fillStyle='#555';g.beginPath();g.arc(X(fEven*100),Y(tmain(n,fEven)),4,0,7);g.fill();const t='break-even '+Math.round(fEven*100)+'%%';const tw=g.measureText(t).width;label(g,t,Math.min(X(fEven*100)-tw/2,X(100)-tw),Y(tmain(n,fEven))-22,'#555');}else if(fEven!==null&&fEven>=1){const t='≈ main at 100%% (measured ±5%% by scene shape)';label(g,t,X(100)-g.measureText(t).width-4,Y(tmain(n,1))-6,'#555');}
// Hover readout: the values of both curves at the f under the mouse.
if(hover!==null){const hp=hover,hf=hp/100,hm=tmain(n,hf),hb=tbranch(n,hf,V);g.save();g.strokeStyle='#999';g.lineWidth=1;g.setLineDash([2,2]);g.beginPath();g.moveTo(X(hp),Y(0));g.lineTo(X(hp),pad);g.stroke();g.setLineDash([]);
 for(const [t,c] of [[hm,'#555'],[hb,'#1b6f65']]){g.fillStyle=c;g.beginPath();g.arc(X(hp),Y(Math.min(t,ymax)),3.5,0,7);g.fill();}
 const lines=[`at ${hp}%% changing`,`main    ${fmt(hm)}`+(MC?` (${fmt(tmain(n,hf,'best'))} – ${fmt(tmain(n,hf,'worst'))})`:''),`branch  ${fmt(hb)}`,hb<=hm?`${(hm/hb).toFixed(2)}× faster`:`${(hb/hm).toFixed(2)}× slower`];
 g.font='12px ui-monospace,Menlo,monospace';const bw=Math.max(...lines.map(l=>g.measureText(l).width))+16,bh=lines.length*16+10;const bx=hp>55?X(hp)-bw-8:X(hp)+8,byy=Math.max(pad,Math.min(Y(Math.min(Math.max(hm,hb),ymax))-bh/2,Y(0)-bh));
 g.fillStyle='rgba(255,255,255,.95)';g.strokeStyle='#bbb';g.beginPath();g.rect(bx,byy,bw,bh);g.fill();g.stroke();lines.forEach((l,i)=>{g.fillStyle=i===1?'#555':i===2?'#1b6f65':'#222';g.fillText(l,bx+8,byy+16+i*16);});g.restore();}}
for(const id of ['L','M','H','K','D','R','V','F'])$(id).addEventListener('input',()=>{preset=null;draw();});
for(const id of ['MC','PN','PS','LOC'])$(id).addEventListener('input',()=>{preset=null;draw();});
$('YS').addEventListener('input',()=>{$('fit').checked=false;draw();});
$('c').addEventListener('mousemove',e=>{const cv=$('c'),pad=52,W=cv.width-2*pad-215;const p=Math.round(100*(e.offsetX-pad)/W);const next=p>=0&&p<=100?p:null;if(next!==hover){hover=next;draw();}});$('c').addEventListener('mouseleave',()=>{hover=null;draw();});$('fit').addEventListener('input',draw);
for(const [name,v] of Object.entries(PRESETS)){const b=document.createElement('button');b.textContent=name;b.onclick=()=>{[$('L').value,$('M').value,$('H').value,$('V').value,$('F').value,$('K').value]=v;$('D').max=v[5];$('D').value=v[6];$('R').checked=v[7];$('MC').checked=v[8];$('PN').value=v[9];$('PS').value=v[10];$('LOC').value=v[11];preset=name;draw();};$('presets').appendChild(b);}
{const wanted=decodeURIComponent(location.hash.slice(1));const start=[...$('presets').children].find(b=>b.textContent===wanted)||$('presets').firstChild;start.click();}
</script>
""" % dict(npoints=len(points), errm=100 * abs(err_main).mean(), errb=100 * abs(err_branch).mean(), maxm=100 * abs(err_main).max(), maxb=100 * abs(err_branch).max(),
           cm=describe(names_main, coef_main), cb=describe(names_branch, coef_branch),
           coef_main=json.dumps([float(v) for v in coef_main]), coef_branch=json.dumps([float(v) for v in coef_branch]), node=PER_DIRTY_NODE,
           editors=json.dumps({k: float(v) for k, v in editor_fit.items()}) if editor_fit else "null",
           neditors=len(editor_points),
           main_cache=json.dumps(dict(measured=True, **main_cache)) if main_cache and main_cache["editor_replay"] is not None else "null",

           editors_text=("main %(main_each).0f each +%(main_reshape).0f reshaped; branch clean %(branch_clean).0f, dirty %(branch_dirty).0f" % editor_fit) if editor_fit else "not measured",
           measured="".join(f"<tr><td>{f}</td><td class=m>{m}</td><td class=n>{b}</td><td>{ch}%</td></tr>" for f, m, b, ch in measured) or "<tr><td colspan=4>matrix.csv not found</td></tr>")
open(f"{out}/complexity.html", "w", encoding="utf-8").write(html)
print("ok")
