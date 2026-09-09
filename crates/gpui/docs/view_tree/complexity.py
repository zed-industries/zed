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
with open(csv_path, encoding="utf-8") as rows:
    for row in csv.reader(rows):
        m = re.search(r"v(\d+)-e(\d+)-p(\d+)-f(\d+)", row[0]) if row else None
        if m and len(row) >= 3:
            V, E, P, F = map(int, m.groups())
            points.append((V, E, P, F, micros(row[1]), micros(row[2])))

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
    for budget, name in [(4.0, "4 ms"), (8.3, "120 Hz"), (16.7, "60 Hz")]:
        if budget < max(main.max(), branch.max()) * 1.1:
            ax.axhline(budget, color="#e76f51", lw=0.8, ls="--"); ax.text(1, budget, name, va="bottom", color="#e76f51", fontsize=8)
    ax.set_xlabel("elements changing per frame (%)"); ax.set_ylabel("frame time (ms)"); ax.set_title(title, fontsize=9); ax.legend(fontsize=8, loc="upper left")
    ax.set_ylim(0, max(main.max(), branch.max()) * 1.15)
fig.tight_layout(); fig.savefig(f"{out}/complexity_surface.png"); plt.close(fig)

# 3. How many medium elements fit a 4 ms frame, against the share changing.
def fits(predict, budget_us):
    lo, hi = 0, 1 << 17
    while lo < hi:
        mid = (lo + hi + 1) // 2
        if predict(mid) <= budget_us: lo = mid
        else: hi = mid - 1
    return lo
fig, ax = plt.subplots(figsize=(6, 3.6))
F = np.linspace(0, 1, 51)
for budget, style in [(4000, "-"), (8300, "--")]:
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
<label>divs (a quad) <input id=L type=range min=0 max=4000 step=10 value=200> <span id=Lv></span></label>
<label>buttons (quad + 3-glyph label) <input id=M type=range min=0 max=1500 step=10 value=300> <span id=Mv></span></label>
<label>cards (quad + shadow + 10-glyph label) <input id=H type=range min=0 max=400 step=5 value=20> <span id=Hv></span></label>
<label>views <input id=V type=range min=1 max=128 value=20> <span id=Vv></span></label>
<label>elements changing per frame <input id=F type=range min=0 max=100 value=10> <span id=Fv></span>%%</label>
<table><tr><th></th><th>main</th><th>branch</th></tr>
<tr><td>frame time</td><td class=m id=tm></td><td class=n id=tb></td></tr>
<tr><td>scale of this scene that fits 4 ms (× the element counts above)</td><td class=m id=sm></td><td class=n id=sb></td></tr>
<tr><td>… i.e. total elements</td><td class=m id=em></td><td class=n id=eb></td></tr>
<tr><td>share changing up to which the frame fits 4 ms</td><td class=m id=fm></td><td class=n id=fb></td></tr></table>
<canvas id=c width=900 height=320></canvas>
<h3>Measured: the Zed-shaped <code>Workbench</code> fixtures</h3>
<table><tr><th>fixture</th><th>main</th><th>branch</th><th>change</th></tr>%(measured)s</table>
<p><small>Model fitted to <code>complexity.csv</code> (%(npoints)d points; mean error %(errm).0f%% / %(errb).0f%%); see <code>view_tree.md</code>, "Scene complexity". Coefficients (µs): main — %(cm)s; branch — %(cb)s.</small></p>
<script>
const M=%(coef_main)s, B=%(coef_branch)s, NODE=%(node)s;
function tmain(n){const N=n[0]+n[1]+n[2];return M[0]+M[1]*n[0]+M[2]*n[1]+M[3]*n[2]+M[4]*(N/100)**2;}
function tbranch(n,f,V){const N=n[0]+n[1]+n[2];let t=B[0]+B[7]*(N/100)**2+NODE*(f==0?0:Math.max(1,Math.ceil(f*V)));
for(let k=0;k<3;k++){const d=n[k]*f;t+=B[1+k]*d+B[4+k]*(n[k]-d);}return t;}
function scale(f,budget){let lo=0,hi=1024;for(let i=0;i<40;i++){const mid=(lo+hi)/2;if(f(mid)<=budget*1000)lo=mid;else hi=mid;}return lo;}
const $=id=>document.getElementById(id);
function fmt(us){return us>=1000?(us/1000).toFixed(2)+' ms':us.toFixed(0)+' µs';}
const PRESETS={
 'Zed-shaped chrome: 200 divs + 300 buttons + 20 cards, 20 views, 10%% changing':[200,300,20,20,10],
 'typing in one of 3 panes: same scene, 3%%':[200,300,20,20,3],
 'dashboard: 3000 div tiles, one animating (1%%)':[3000,0,0,40,1],
 'list: 300 card rows scrolling (all changing)':[0,0,300,1,100],
 'everything dirty (resize / focus change)':[200,300,20,20,100],
 'sweep-sized: 1500 buttons, 25%%':[0,1500,0,16,25],
};
const BUDGETS=[[4,'4 ms'],[8.3,'120 Hz']];
function label(g,text,x,y,color){g.font='12px system-ui';g.lineWidth=3;g.strokeStyle='rgba(255,255,255,.9)';g.strokeText(text,x,y);g.fillStyle=color;g.fillText(text,x,y);}
function draw(){const n=[+$('L').value,+$('M').value,+$('H').value],V=+$('V').value,F=+$('F').value,budget=4,f=F/100;
$('Lv').textContent=n[0];$('Mv').textContent=n[1];$('Hv').textContent=n[2];$('Vv').textContent=V;$('Fv').textContent=F;
const tm=tmain(n),tb=tbranch(n,f,V);$('tm').textContent=fmt(tm);$('tb').textContent=fmt(tb);
const N=n[0]+n[1]+n[2];const sm=scale(s=>tmain(n.map(x=>x*s)),budget),sb=scale(s=>tbranch(n.map(x=>x*s),f,V),budget);
$('sm').textContent=sm.toFixed(2)+'×';$('sb').textContent=sb.toFixed(2)+'×';$('em').textContent=Math.round(N*sm);$('eb').textContent=Math.round(N*sb);
// Where the branch curve crosses the budget (it is linear in f between 0 and 100%%).
const t0=tbranch(n,0,V),t1=tbranch(n,1,V);const fBudget=t1===t0?null:(4000-t0)/(t1-t0);
$('fm').textContent=tm<=4000?'always':'never';$('fb').textContent=fBudget===null?(t0<=4000?'always':'never'):fBudget>=1?'always':fBudget<=0?'never':Math.round(fBudget*100)+'%%';
// Break-even: the share of change above which the branch costs what main does.
const fEven=t1===t0?null:(tm-t0)/(t1-t0);
const cv=$('c'),g=cv.getContext('2d');g.clearRect(0,0,cv.width,cv.height);const pad=44,W=cv.width-2*pad-40,H=cv.height-2*pad;
const ymax=Math.max(tm,tb,4000)*1.15;const X=p=>pad+W*p/100,Y=us=>pad+H*(1-us/ymax);
g.fillStyle='rgba(150,150,150,.5)';g.fillRect(X(0),Y(tm),W,Y(0)-Y(tm));
g.beginPath();g.moveTo(X(0),Y(0));for(let p=0;p<=100;p++)g.lineTo(X(p),Y(tbranch(n,p/100,V)));g.lineTo(X(100),Y(0));g.closePath();g.fillStyle='rgba(42,157,143,.7)';g.fill();
g.lineWidth=1;g.strokeStyle='#e76f51';g.setLineDash([4,4]);for(const [ms,name] of BUDGETS){if(ms*1000<ymax){g.beginPath();g.moveTo(X(0),Y(ms*1000));g.lineTo(X(100),Y(ms*1000));g.stroke();label(g,name,X(100)+6,Y(ms*1000)+4,'#e76f51');}}g.setLineDash([]);
g.lineWidth=1;g.strokeStyle='#e76f51';g.beginPath();g.moveTo(X(F),Y(0));g.lineTo(X(F),Y(ymax/1.15));g.stroke();
label(g,'0%%',X(0)-8,Y(0)+16,'#333');label(g,'100%%',X(100)-14,Y(0)+16,'#333');label(g,'elements changing per frame',X(50)-80,Y(0)+32,'#333');label(g,fmt(ymax/1.15),2,Y(ymax/1.15)+4,'#333');label(g,'0',24,Y(0)+4,'#333');
if(F>6&&F<94)label(g,F+'%%',X(F)-10,Y(0)+16,'#e76f51');
const faster=tm/tb;const gain=faster>=1?faster.toFixed(1)+'× faster ('+Math.round(100*(1-tb/tm))+'%%)':(tb/tm).toFixed(2)+'× slower';
const branchText='branch '+fmt(tb)+' — '+gain;const bw=g.measureText(branchText).width+8;
const bx=F>55?X(F)-bw:X(F)+6;const by=(Y(tb)-Y(tm))<22?Y(tb)+18:Y(tb)-6;
label(g,'main '+fmt(tm),X(2),Y(tm)-6,'#555');label(g,branchText,bx,by,'#1b6f65');
if(fBudget!==null&&fBudget>0&&fBudget<1){g.fillStyle='#e76f51';g.beginPath();g.arc(X(fBudget*100),Y(4000),4,0,7);g.fill();label(g,'fits 4 ms up to '+Math.round(fBudget*100)+'%%',Math.min(X(fBudget*100)+6,X(100)-130),Y(4000)-6,'#e76f51');}
if(fEven!==null&&fEven>0&&fEven<1){g.fillStyle='#555';g.beginPath();g.arc(X(fEven*100),Y(tm),4,0,7);g.fill();const t='break-even '+Math.round(fEven*100)+'%%';const tw=g.measureText(t).width;label(g,t,Math.min(X(fEven*100)-tw/2,X(100)-tw),Y(tm)-22,'#555');}else if(fEven!==null&&fEven>=1){const t='never slower than main below 100%%';label(g,t,X(100)-g.measureText(t).width-4,Y(tm)-6,'#555');}}
for(const id of ['L','M','H','V','F'])$(id).addEventListener('input',draw);
for(const [name,v] of Object.entries(PRESETS)){const b=document.createElement('button');b.textContent=name;b.onclick=()=>{[$('L').value,$('M').value,$('H').value,$('V').value,$('F').value]=v;draw();};$('presets').appendChild(b);}
draw();
</script>
""" % dict(npoints=len(points), errm=100 * abs(err_main).mean(), errb=100 * abs(err_branch).mean(), maxm=100 * abs(err_main).max(), maxb=100 * abs(err_branch).max(),
           cm=describe(names_main, coef_main), cb=describe(names_branch, coef_branch),
           coef_main=json.dumps([float(v) for v in coef_main]), coef_branch=json.dumps([float(v) for v in coef_branch]), node=PER_DIRTY_NODE,
           measured="".join(f"<tr><td>{f}</td><td class=m>{m}</td><td class=n>{b}</td><td>{ch}%</td></tr>" for f, m, b, ch in measured) or "<tr><td colspan=4>matrix.csv not found</td></tr>")
open(f"{out}/complexity.html", "w", encoding="utf-8").write(html)
print("ok")
