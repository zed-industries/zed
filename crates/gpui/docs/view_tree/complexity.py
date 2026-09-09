import sys, os, csv, json, re
import matplotlib; matplotlib.use("Agg")
import matplotlib.pyplot as plt
import numpy as np
# Usage: python complexity.py <output dir> [complexity.csv]  (needs matplotlib, numpy)
# Fits the frame cost model to complexity.sh's rows and writes complexity_fit.png,
# complexity_surface.png and complexity.html (sliders over the fitted model).
out = sys.argv[1]
csv_path = sys.argv[2] if len(sys.argv) > 2 else os.path.join(os.path.dirname(__file__), "complexity.csv")
UNITS = {"ns": 1e-3, "µs": 1, "us": 1, "ms": 1e3, "s": 1e6}
def micros(text):
    value, unit = text.split()
    return float(value) * UNITS[unit]

points = []  # (V, E, P, F, main_us, branch_us)
with open(csv_path, encoding="utf-8") as rows:
    for row in csv.reader(rows):
        m = re.search(r"v(\d+)-e(\d+)-p(\d+)-f(\d+)", row[0]) if row else None
        if m and len(row) >= 3:
            V, E, P, F = map(int, m.groups())
            points.append((V, E, P, F, micros(row[1]), micros(row[2])))

def counts(V, E, P, F):
    dirty_views = 0 if F == 0 else max(1, -(-F * V // 100))
    clean_views = V - dirty_views
    return dict(Vd=dirty_views, Ed=dirty_views * E, Pd=dirty_views * E * P,
                Vc=clean_views, Ec=clean_views * E, Pc=clean_views * E * P)

BRANCH_TERMS = ["Vd", "Ed", "Pd", "Vc", "Ec", "Pc"]
MAIN_TERMS = ["V", "E", "P"]  # everything is dirty on main
def design(terms, main):
    rows = []
    for V, E, P, F, *_ in points:
        c = counts(V, E, P, F)
        if main:
            rows.append([1, V, V * E, V * E * P])
        else:
            rows.append([1] + [c[t] for t in terms])
    return np.array(rows, dtype=float)

def fit(X, y):
    coef, *_ = np.linalg.lstsq(X, y, rcond=None)
    pred = X @ coef
    r2 = 1 - ((y - pred) ** 2).sum() / ((y - y.mean()) ** 2).sum()
    return coef, pred, r2

y_main = np.array([p[4] for p in points]); y_branch = np.array([p[5] for p in points])
coef_main, pred_main, r2_main = fit(design(MAIN_TERMS, True), y_main)
coef_branch, pred_branch, r2_branch = fit(design(BRANCH_TERMS, False), y_branch)
names_main = ["c", "a (per view)", "b (per element)", "p (per primitive)"]
names_branch = ["c", "a dirty view", "b dirty element", "p dirty primitive", "a' clean view", "h clean element", "r clean primitive"]
print("main   R^2=%.3f" % r2_main, ", ".join(f"{n}={v:.3f}" for n, v in zip(names_main, coef_main)), "µs")
print("branch R^2=%.3f" % r2_branch, ", ".join(f"{n}={v:.4f}" for n, v in zip(names_branch, coef_branch)), "µs")

def predict_main(V, E, P, F=None):
    return coef_main @ np.array([1, V, V * E, V * E * P])
def predict_branch(V, E, P, F):
    c = counts(V, E, P, F)
    return coef_branch @ np.array([1] + [c[t] for t in BRANCH_TERMS])

plt.rcParams.update({"font.size": 9, "figure.dpi": 130})

# 1. Fit quality: measured vs predicted per point.
labels = [f"v{V}-e{E}-p{P}-f{F}" for V, E, P, F, *_ in points]
fig, ax = plt.subplots(figsize=(10, 5))
x = np.arange(len(points)); w = 0.2
ax.bar(x - 1.5 * w, y_main / 1000, w, color="#999", label="main measured")
ax.bar(x - 0.5 * w, pred_main / 1000, w, color="#ccc", label=f"main model (R² {r2_main:.3f})")
ax.bar(x + 0.5 * w, y_branch / 1000, w, color="#2a9d8f", label="branch measured")
ax.bar(x + 1.5 * w, pred_branch / 1000, w, color="#9fd8d0", label=f"branch model (R² {r2_branch:.3f})")
ax.set_xticks(x); ax.set_xticklabels(labels, rotation=45, ha="right"); ax.set_ylabel("frame time (ms)"); ax.legend(fontsize=8)
ax.set_title("Complexity sweep: measured frame time and the fitted cost model")
fig.tight_layout(); fig.savefig(f"{out}/complexity_fit.png"); plt.close(fig)

# 2. Frame time against the share of views changing, for two scene sizes. main's surface is
#    the branch's with the clean terms zeroed and everything dirty, so they meet at 100%.
fig, axes = plt.subplots(1, 2, figsize=(10, 4), sharey=False)
for ax, (V, E, P, title) in zip(axes, [(16, 128, 4, "16 views × 128 elements (Zed-sized, ~8k primitives)"),
                                       (16, 625, 4, "16 views × 625 elements (10k elements, 40k primitives)")]):
    F = np.linspace(0, 100, 101)
    branch = np.array([predict_branch(V, E, P, f) for f in F]) / 1000
    main = np.full_like(F, predict_main(V, E, P) / 1000)
    ax.fill_between(F, 0, main, color="#bbb", alpha=0.6, label="main (everything redrawn)")
    ax.fill_between(F, 0, branch, color="#2a9d8f", alpha=0.7, label="branch (clean views replayed)")
    for budget, name in [(4.0, "4 ms"), (8.3, "120 Hz")]:
        ax.axhline(budget, color="#e76f51", lw=0.8, ls="--"); ax.text(1, budget, name, va="bottom", color="#e76f51", fontsize=8)
    ax.set_xlabel("views changing per frame (%)"); ax.set_ylabel("frame time (ms)"); ax.set_title(title, fontsize=9); ax.legend(fontsize=8, loc="upper left")
    ax.set_ylim(0, max(main.max(), branch.max()) * 1.15)
fig.tight_layout(); fig.savefig(f"{out}/complexity_surface.png"); plt.close(fig)

# 3. Interactive page with the coefficients baked in.
html = """<!doctype html><meta charset="utf-8"><title>GPUI view tree: frame cost model</title>
<style>body{font:14px system-ui;margin:2em;max-width:900px}label{display:block;margin:.4em 0}input[type=range]{width:320px;vertical-align:middle}
table{border-collapse:collapse;margin:1em 0}td,th{padding:.3em .8em;border-bottom:1px solid #ddd;text-align:right}th:first-child,td:first-child{text-align:left}
canvas{border:1px solid #ddd;margin-top:1em}.n{color:#2a9d8f}.m{color:#777}small{color:#666}</style>
<h2>GPUI view tree — frame cost model</h2>
<p>Least-squares fit of the <code>Complexity</code> sweep (<code>complexity.csv</code>). main: <code>t = c + a·V + b·V·E + p·V·E·P</code> (R² %(r2m).3f). branch: the same terms over the <em>dirty</em> views plus <code>a'·V + h·E + r·P</code> over the <em>clean</em> ones (R² %(r2b).3f). Dirty views per frame = ⌈F·V⌉ (at least one unless F = 0).</p>
<label>views V <input id=V type=range min=1 max=256 value=16> <span id=Vv></span></label>
<label>elements per view E <input id=E type=range min=1 max=2048 value=128> <span id=Ev></span></label>
<label>primitives per element P <input id=P type=range min=1 max=16 value=4> <span id=Pv></span></label>
<label>views changing per frame F <input id=F type=range min=0 max=100 value=25> <span id=Fv></span>%%</label>
<label>frame budget <input id=B type=range min=1 max=17 step=0.1 value=4> <span id=Bv></span> ms</label>
<table><tr><th></th><th>main</th><th>branch</th></tr>
<tr><td>frame time</td><td class=m id=tm></td><td class=n id=tb></td></tr>
<tr><td>elements per view that fit the budget (at this V, P, F)</td><td class=m id=em></td><td class=n id=eb></td></tr>
<tr><td>total elements that fit</td><td class=m id=tem></td><td class=n id=teb></td></tr></table>
<canvas id=c width=860 height=320></canvas>
<p><small>Coefficients (µs): main %(cm)s; branch %(cb)s. Curve: frame time against F for the chosen V, E, P; grey is main, green the branch; dashed line is the budget.</small></p>
<script>
const M=%(coef_main)s, B=%(coef_branch)s;
function dirty(V,F){return F==0?0:Math.max(1,Math.ceil(F*V/100));}
function tmain(V,E,P){return M[0]+M[1]*V+M[2]*V*E+M[3]*V*E*P;}
function tbranch(V,E,P,F){const d=dirty(V,F),c=V-d;return B[0]+B[1]*d+B[2]*d*E+B[3]*d*E*P+B[4]*c+B[5]*c*E+B[6]*c*E*P;}
function fit(f,V,P,F,budget){let lo=0,hi=1<<20;while(lo<hi){const mid=(lo+hi+1)>>1;if(f(V,mid,P,F)<=budget*1000)lo=mid;else hi=mid-1;}return lo;}
const $=id=>document.getElementById(id);
function fmt(us){return us>=1000?(us/1000).toFixed(2)+' ms':us.toFixed(0)+' µs';}
function draw(){const V=+$('V').value,E=+$('E').value,P=+$('P').value,F=+$('F').value,budget=+$('B').value;
$('Vv').textContent=V;$('Ev').textContent=E;$('Pv').textContent=P;$('Fv').textContent=F;$('Bv').textContent=budget;
const tm=tmain(V,E,P),tb=tbranch(V,E,P,F);$('tm').textContent=fmt(tm);$('tb').textContent=fmt(tb);
const em=fit((v,e,p)=>tmain(v,e,p),V,P,F,budget),eb=fit(tbranch,V,P,F,budget);$('em').textContent=em;$('eb').textContent=eb;$('tem').textContent=em*V;$('teb').textContent=eb*V;
const cv=$('c'),g=cv.getContext('2d');g.clearRect(0,0,cv.width,cv.height);const pad=40,W=cv.width-2*pad,H=cv.height-2*pad;
const ymax=Math.max(tm,budget*1000,tb)*1.15;const X=f=>pad+W*f/100,Y=us=>pad+H*(1-us/ymax);
g.fillStyle='rgba(150,150,150,.5)';g.fillRect(X(0),Y(tm),W,Y(0)-Y(tm));
g.beginPath();g.moveTo(X(0),Y(0));for(let f=0;f<=100;f++)g.lineTo(X(f),Y(tbranch(V,E,P,f)));g.lineTo(X(100),Y(0));g.closePath();g.fillStyle='rgba(42,157,143,.7)';g.fill();
g.strokeStyle='#e76f51';g.setLineDash([4,4]);g.beginPath();g.moveTo(X(0),Y(budget*1000));g.lineTo(X(100),Y(budget*1000));g.stroke();g.setLineDash([]);
g.strokeStyle='#e76f51';g.beginPath();g.moveTo(X(F),Y(0));g.lineTo(X(F),Y(ymax/1.15));g.stroke();
g.fillStyle='#333';g.font='12px system-ui';g.fillText('0%%',X(0)-8,Y(0)+16);g.fillText('100%% of views changing',X(100)-120,Y(0)+16);g.fillText(fmt(ymax/1.15),2,Y(ymax/1.15)+4);g.fillText('0',20,Y(0)+4);
g.fillStyle='#777';g.fillText('main '+fmt(tm),X(2),Y(tm)-4);g.fillStyle='#2a9d8f';g.fillText('branch '+fmt(tb)+' at F='+F+'%%',X(F)+4,Y(tb)-4);}
for(const id of ['V','E','P','F','B'])$(id).addEventListener('input',draw);draw();
</script>
""" % dict(r2m=r2_main, r2b=r2_branch,
           cm=", ".join(f"{n}={v:.3f}" for n, v in zip(names_main, coef_main)),
           cb=", ".join(f"{n}={v:.4f}" for n, v in zip(names_branch, coef_branch)),
           coef_main=json.dumps([float(v) for v in coef_main]), coef_branch=json.dumps([float(v) for v in coef_branch]))
open(f"{out}/complexity.html", "w", encoding="utf-8").write(html)
print("ok")
