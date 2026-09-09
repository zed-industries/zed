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
with open(csv_path, encoding="utf-8") as rows:
    for row in csv.reader(rows):
        if not row or len(row) < 3:
            continue
        if m := re.search(r"v(\d+)-e(\d+)-p(\d+)-f(\d+)", row[0]):
            V, E, P, F = map(int, m.groups())
            points.append((V, E, P, F, micros(row[1]), micros(row[2])))
        elif m := re.search(r"editors/k(\d+)-d(\d+)-(cursor|scroll)", row[0]):
            K, D, how = int(m.group(1)), int(m.group(2)), m.group(3)
            editor_points.append((K, D, how == "scroll", micros(row[1]), micros(row[2])))

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
<style>body{font:14px system-ui;margin:2em;max-width:940px;color:#222}label{display:block;margin:.35em 0}input[type=range]{width:300px;vertical-align:middle}
table{border-collapse:collapse;margin:1em 0}td,th{padding:.3em .8em;border-bottom:1px solid #ddd;text-align:right}th:first-child,td:first-child{text-align:left}
canvas{border:1px solid #ddd;margin-top:1em}.n{color:#2a9d8f}.m{color:#777}small{color:#666}button{margin:.2em .3em .2em 0}</style>
<h2>GPUI view tree — frame cost model</h2>
<p>Presets: <span id=presets></span></p>
<style>button.active{background:#2a9d8f;color:#fff;border-color:#2a9d8f}</style>
<p><small>The frame budget at 120 Hz is 8.3 ms, shared by every window and by everything else on the main thread; the 4 ms line is the half of it a draw should stay under.</small></p>
<label>divs (a quad) <input id=L type=range min=0 max=4000 step=10 value=200> <span id=Lv></span></label>
<label>buttons (quad + 3-glyph label) <input id=M type=range min=0 max=1500 step=10 value=300> <span id=Mv></span></label>
<label>cards (quad + shadow + 10-glyph label) <input id=H type=range min=0 max=400 step=5 value=20> <span id=Hv></span></label>
<label>editors (~40 lines of code each) <input id=K type=range min=0 max=8 value=3> <span id=Kv></span></label>
<label>… of which changing <input id=D type=range min=0 max=8 value=1> <span id=Dv></span> <label style="display:inline;margin-left:1em"><input id=R type=checkbox> showing new text (lines reshaped, e.g. scrolling)</label></label>
<label>views <input id=V type=range min=1 max=128 value=20> <span id=Vv></span></label>
<label>elements changing per frame <input id=F type=range min=0 max=100 value=10> <span id=Fv></span>%%</label>
<fieldset style="margin:1em 0;padding:.6em 1em;border:1px solid #ddd"><legend><label style="display:inline"><input id=MC type=checkbox> <code>main</code> caches editors and panels, as Zed ships it</label></legend>
<label>cached panels <input id=PN type=range min=1 max=12 value=3> <span id=PNv></span></label>
<label>share of divs / buttons / cards inside them <input id=PS type=range min=0 max=100 value=60> <span id=PSv></span>%%</label>
<p id=mcnote style="margin:.3em 0;color:#666"></p>
<p id=mcwarn style="margin:.3em 0;color:#b00"></p></fieldset>
<table><tr><th></th><th>main</th><th>branch</th></tr>
<tr><td>frame time <small id=tmnote></small></td><td class=m id=tm></td><td class=n id=tb></td></tr>
<tr><td>scale of this scene that fits a 120 Hz frame, 8.3 ms (× the element counts above)</td><td class=m id=sm></td><td class=n id=sb></td></tr>
<tr><td>… i.e. total elements</td><td class=m id=em></td><td class=n id=eb></td></tr>
<tr><td>share changing up to which the frame fits 8.3 ms</td><td class=m id=fm></td><td class=n id=fb></td></tr></table>
<p style="margin:.6em 0 0">y axis top <input id=YS type=range min=2 max=20 step=0.1 value=4> <span id=YSv></span> ms <button id=fit>fit the curves</button></p>
<canvas id=c width=900 height=340></canvas>
<h3>The model</h3>
<p>Element kinds <i>k</i> ∈ {div, button, card} with counts <i>n<sub>k</sub></i>, <i>N</i> = Σ <i>n<sub>k</sub></i>; a share <i>f</i> of the elements and <i>D</i> of the <i>K</i> editors change per frame.</p>
<pre id=formula></pre>
<h3>Measured: the Zed-shaped <code>Workbench</code> fixtures</h3>
<table><tr><th>fixture</th><th>main</th><th>branch</th><th>change</th></tr>%(measured)s</table>
<p><small>Model fitted to <code>complexity.csv</code> (%(npoints)d element points, mean error %(errm).0f%% / %(errb).0f%%; %(neditors)d editor points); see <code>view_tree.md</code>, "Scene complexity". Coefficients (µs): main — %(cm)s; branch — %(cb)s; editors — %(editors_text)s.</small></p>
<script>
const M=%(coef_main)s, B=%(coef_branch)s, NODE=%(node)s, ED=%(editors)s;
// main's own caching (reuse_paint on cached panels and pane items). PLACEHOLDER coefficients
// until the .cached() fixture variants have been measured: a replayed element on main is
// taken as the branch's replay cost, an idle cached editor as the branch's clean editor.
const MAIN_CACHE={measured:false, replay:[B[4],B[5],B[6]], editor_replay:ED?ED.branch_clean:0};
let K=0,D=0,R=false,preset=null,MC=false,PN=3,PS=0.6,fitRequested=true;
function cacheStats(n,f){const inside=n.map(x=>x*PS);const per=inside.reduce((a,b)=>a+b,0)/PN;const pClean=Math.pow(1-f,per);return {inside,per,pClean};}
const us=v=>v.toFixed(2);
document.getElementById('formula').textContent=
`main    t = ${us(M[0])} + ${us(M[1])}·n_div + ${us(M[2])}·n_button + ${us(M[3])}·n_card + ${us(M[4])}·(N/100)²`+
(ED?`\n          + ${ED.main_each.toFixed(0)}·K + ${ED.main_reshape.toFixed(0)}·D·[reshaped]`:'')+
`\n  with main's caching on: elements inside the P cached panels are replayed at r_main when the panel is clean,`+
`\n          P(clean) = (1 − f)^(elements per panel);  editors: ${MAIN_CACHE.editor_replay.toFixed(0)}·(K − D) + ${ED?ED.main_each.toFixed(0):'?'}·D`+
`\n          r_main = (${MAIN_CACHE.replay.map(us).join(', ')}) per div, button, card${MAIN_CACHE.measured?'':'   ← placeholders'}`+
`\n\nbranch  t = ${us(B[0])} + (${us(B[1])}·n_div + ${us(B[2])}·n_button + ${us(B[3])}·n_card)·f        dirty: rendered`+
`\n          + (${us(B[4])}·n_div + ${us(B[5])}·n_button + ${us(B[6])}·n_card)·(1 − f)   clean: replayed`+
`\n          + ${us(B[7])}·(N/100)² + ${NODE}·⌈f·views⌉`+
(ED?`\n          + ${ED.branch_clean.toFixed(0)}·(K − D) + ${(R?ED.branch_dirty:ED.branch_clean).toFixed(0)}·D`:'')+
`\n\n(µs; least squares on relative error over the sweep; main redraws every element every frame)`;
function editorsMain(){return ED?K*ED.main_each+(R?D*ED.main_reshape:0):0;}
function editorsBranch(){return ED?(K-D)*ED.branch_clean+D*(R?ED.branch_dirty:ED.branch_clean):0;}
function tmain(n,f,mode){const N=n[0]+n[1]+n[2];let t=M[0]+M[4]*(N/100)**2;
if(MC){const {inside}=cacheStats(n,f);const pClean=mode==='best'?1:mode==='worst'?0:cacheStats(n,f).pClean;for(let k=0;k<3;k++){const clean=inside[k]*pClean;t+=M[1+k]*(n[k]-clean)+MAIN_CACHE.replay[k]*clean;}
 t+=ED?(K-D)*MAIN_CACHE.editor_replay+D*(ED.main_each+(R?ED.main_reshape:0)):0;}
else{t+=M[1]*n[0]+M[2]*n[1]+M[3]*n[2]+editorsMain();}
return t;}
function tbranch(n,f,V){const N=n[0]+n[1]+n[2];let t=B[0]+B[7]*(N/100)**2+NODE*(f==0?0:Math.max(1,Math.ceil(f*V)))+editorsBranch();
for(let k=0;k<3;k++){const d=n[k]*f;t+=B[1+k]*d+B[4+k]*(n[k]-d);}return t;}
function scale(f,budget){let lo=0,hi=1024;for(let i=0;i<40;i++){const mid=(lo+hi)/2;if(f(mid)<=budget*1000)lo=mid;else hi=mid;}return lo;}
const $=id=>document.getElementById(id);
function fmt(us){return us>=1000?(us/1000).toFixed(2)+' ms':us.toFixed(0)+' µs';}
// [divs, buttons, cards, views, f%%, editors, editors changing, reshaped, main caches?, cached panels, share inside them %%]
const PRESETS={
 'Zed, typing':[200,300,20,20,3,3,1,false,true,3,60],
 'Zed, scrolling':[200,300,20,20,3,3,1,true,true,3,60],
 'Zed, resize':[200,300,20,20,100,3,3,true,true,3,60],
 'Big workspace':[600,800,60,60,5,6,1,false,true,4,60],
 'Busy UI':[400,1000,200,40,10,0,0,false,false,3,60],
 'Data table':[4000,600,0,50,5,0,0,false,false,3,60],
 'Dashboard':[3000,0,0,40,1,0,0,false,false,3,60],
 'Long list':[0,0,300,1,100,0,0,false,false,3,60],
};
const BUDGET=8.3;const BUDGETS=[[4,'4 ms — half a 120 Hz frame'],[8.3,'8.3 ms (120 Hz)'],[16.7,'16.7 ms (60 Hz)']];
function label(g,text,x,y,color){g.font='12px system-ui';g.lineWidth=3;g.strokeStyle='rgba(255,255,255,.9)';g.strokeText(text,x,y);g.fillStyle=color;g.fillText(text,x,y);}
function draw(){const n=[+$('L').value,+$('M').value,+$('H').value],V=+$('V').value,F=+$('F').value,budget=BUDGET,f=F/100;
K=+$('K').value;$('D').max=K;D=Math.min(+$('D').value,K);$('D').value=D;R=$('R').checked;
MC=$('MC').checked;PN=+$('PN').value;PS=+$('PS').value/100;$('PNv').textContent=PN;$('PSv').textContent=Math.round(PS*100);
{const {per,pClean}=cacheStats(n,f);$('mcnote').textContent=MC?`each panel ≈ ${Math.round(per)} elements; clean with probability (1 − f)ⁿ = ${(100*pClean).toFixed(pClean<0.01?2:0)}%% at f = ${F}%%; expected ${(PN*pClean).toFixed(1)} of ${PN} panels replayed this frame`:'';
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
const cv=$('c'),g=cv.getContext('2d');g.clearRect(0,0,cv.width,cv.height);const pad=44,W=cv.width-2*pad-200,H=cv.height-2*pad;
const fitTop=Math.max(2000,Math.max(tmain(n,1),tb)*1.15);$('YS').max=Math.max(20,fitTop/1000).toFixed(1);if(fitRequested){$('YS').value=(fitTop/1000).toFixed(1);fitRequested=false;}const ymax=(+$('YS').value)*1000;$('YSv').textContent=(+$('YS').value).toFixed(1);const X=p=>pad+W*p/100,Y=us=>pad+H*(1-us/ymax);
const mainAt=p=>tmain(n,p/100);const tmMax=Math.max(...Array.from({length:101},(_,p)=>mainAt(p)));
g.beginPath();g.moveTo(X(0),Y(0));for(let p=0;p<=100;p++)g.lineTo(X(p),Y(mainAt(p)));g.lineTo(X(100),Y(0));g.closePath();g.fillStyle='rgba(150,150,150,.5)';g.fill();
if(MC){g.save();g.setLineDash([2,3]);g.lineWidth=1;g.strokeStyle='#777';for(const mode of ['best','worst']){g.beginPath();for(let p=0;p<=100;p++){const y=Y(tmain(n,p/100,mode));if(p===0)g.moveTo(X(p),y);else g.lineTo(X(p),y);}g.stroke();}g.restore();
 const best=tmain(n,f,'best'),worst=tmain(n,f,'worst');const lx=F>55?X(F)-190:X(F)+8;
 label(g,'every panel clean: '+fmt(best),lx,Y(best)+14,'#666');label(g,'no panel clean: '+fmt(worst),lx,Y(worst)-6,'#666');}
g.beginPath();g.moveTo(X(0),Y(0));for(let p=0;p<=100;p++)g.lineTo(X(p),Y(tbranch(n,p/100,V)));g.lineTo(X(100),Y(0));g.closePath();g.fillStyle='rgba(42,157,143,.7)';g.fill();
g.lineWidth=1;g.strokeStyle='#e76f51';g.setLineDash([4,4]);for(const [ms,name] of BUDGETS){if(ms*1000<ymax){g.lineWidth=1;g.strokeStyle='#e76f51';g.setLineDash([4,4]);g.beginPath();g.moveTo(X(0),Y(ms*1000));g.lineTo(X(100),Y(ms*1000));g.stroke();const cross=t1===t0?null:(ms*1000-t0)/(t1-t0);const upTo=cross!==null&&cross>0&&cross<1?' · up to '+Math.round(cross*100)+'%%':'';label(g,name+upTo,X(100)+6,Y(ms*1000)+4,'#e76f51');}}g.setLineDash([]);
g.lineWidth=1;g.strokeStyle='#e76f51';g.beginPath();g.moveTo(X(F),Y(0));g.lineTo(X(F),Y(ymax/1.15));g.stroke();
if(F>6)label(g,'0%%',X(0)-8,Y(0)+16,'#333');if(F<94)label(g,'100%%',X(100)-14,Y(0)+16,'#333');label(g,'elements changing per frame',X(50)-80,Y(0)+32,'#333');label(g,fmt(ymax/1.15),2,Y(ymax/1.15)+4,'#333');label(g,'0',24,Y(0)+4,'#333');
label(g,F+'%%',X(F)-10,Y(0)+16,'#e76f51');
const faster=tm/tb;const gain=faster>=1?faster.toFixed(1)+'× faster ('+Math.round(100*(1-tb/tm))+'%%)':(tb/tm).toFixed(2)+'× slower';
const branchText='branch '+fmt(tb)+' — '+gain;const bw=g.measureText(branchText).width+8;
const bx=F>55?X(F)-bw:X(F)+6;const by=(Y(tb)-Y(tm))<22?Y(tb)+18:Y(tb)-6;
{const worstY=MC?Y(tmain(n,f,'worst')):-99;const my=MC?(Math.abs(worstY-Y(tm))<18?Y(tm)+14:Y(tm)-6):Y(tmMax)-6;label(g,'main '+fmt(tm)+(MC?' expected':''),MC?(F>55?X(F)-150:X(F)+8):X(2),my,'#555');}label(g,branchText,bx,by,'#1b6f65');
for(const b of $('presets').children)b.classList.toggle('active',b.textContent===preset);
g.font='bold 13px system-ui';label(g,preset?preset:'custom scene',pad,pad-14,'#222');g.font='12px system-ui';
for(const [ms] of BUDGETS){const cross=t1===t0?null:(ms*1000-t0)/(t1-t0);if(cross!==null&&cross>0&&cross<1&&ms*1000<ymax){g.fillStyle='#e76f51';g.beginPath();g.arc(X(cross*100),Y(ms*1000),4,0,7);g.fill();}}
if(fEven!==null&&fEven>0&&fEven<1){g.fillStyle='#555';g.beginPath();g.arc(X(fEven*100),Y(tmain(n,fEven)),4,0,7);g.fill();const t='break-even '+Math.round(fEven*100)+'%%';const tw=g.measureText(t).width;label(g,t,Math.min(X(fEven*100)-tw/2,X(100)-tw),Y(tmain(n,fEven))-22,'#555');}else if(fEven!==null&&fEven>=1){const t='≈ main at 100%% (measured ±5%% by scene shape)';label(g,t,X(100)-g.measureText(t).width-4,Y(tmain(n,1))-6,'#555');}}
for(const id of ['L','M','H','K','D','R','V','F'])$(id).addEventListener('input',()=>{preset=null;draw();});
for(const id of ['MC','PN','PS'])$(id).addEventListener('input',()=>{preset=null;draw();});
$('YS').addEventListener('input',draw);$('fit').addEventListener('click',()=>{fitRequested=true;draw();});
for(const [name,v] of Object.entries(PRESETS)){const b=document.createElement('button');b.textContent=name;b.onclick=()=>{[$('L').value,$('M').value,$('H').value,$('V').value,$('F').value,$('K').value]=v;$('D').max=v[5];$('D').value=v[6];$('R').checked=v[7];$('MC').checked=v[8];$('PN').value=v[9];$('PS').value=v[10];preset=name;fitRequested=true;draw();};$('presets').appendChild(b);}
$('presets').firstChild.click();
</script>
""" % dict(npoints=len(points), errm=100 * abs(err_main).mean(), errb=100 * abs(err_branch).mean(), maxm=100 * abs(err_main).max(), maxb=100 * abs(err_branch).max(),
           cm=describe(names_main, coef_main), cb=describe(names_branch, coef_branch),
           coef_main=json.dumps([float(v) for v in coef_main]), coef_branch=json.dumps([float(v) for v in coef_branch]), node=PER_DIRTY_NODE,
           editors=json.dumps({k: float(v) for k, v in editor_fit.items()}) if editor_fit else "null",
           neditors=len(editor_points),
           editors_text=("main %(main_each).0f each +%(main_reshape).0f reshaped; branch clean %(branch_clean).0f, dirty %(branch_dirty).0f" % editor_fit) if editor_fit else "not measured",
           measured="".join(f"<tr><td>{f}</td><td class=m>{m}</td><td class=n>{b}</td><td>{ch}%</td></tr>" for f, m, b, ch in measured) or "<tr><td colspan=4>matrix.csv not found</td></tr>")
open(f"{out}/complexity.html", "w", encoding="utf-8").write(html)
print("ok")
