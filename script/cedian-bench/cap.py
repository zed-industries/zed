#!/usr/bin/env python3
"""The live pass's turn cap and what OMP reported (ADR-0059 decision 4).

  cap.py admit <ledger> <max> <step> <estimate> <reviewer 0|1>
      reserve the step's worst case (estimate + 1, + 1 more when its turn may
      spawn a reviewer) or exit 3 when that would pass <max>.
  cap.py book <ledger> <step> <reserved> <actual>
      release the reservation and book the turns the step used; exit 3 when
      the pass has reached <max> (read back from the ledger's header).
  cap.py used <ledger>
  cap.py count <tap>     model turns (`agent_start`) across every OMP process
                         a step started (one tap file per process).
  cap.py models <tap>    the models the step's first OMP process reported.

The ledger is append-only and every read-modify-write holds flock on it, so
two invocations cannot both be admitted into the last turns.
"""
import fcntl
import json
import os
import sys

CEILING = 45
MARGIN = 1
TEMP = ("/tmp/", "/private/tmp/", "/var/folders/", "/private/var/folders/")


def die(msg, code=2):
    print(msg, file=sys.stderr)
    sys.exit(code)


class Ledger:
    def __init__(self, path):
        real = os.path.realpath(path)
        if real.startswith(TEMP):
            die(f"refused: the turn ledger {path} is in a temp dir a reboot clears")
        os.makedirs(os.path.dirname(real), exist_ok=True)
        self.f = open(real, "a+")
        fcntl.flock(self.f, fcntl.LOCK_EX)

    def lines(self):
        self.f.seek(0)
        return [l.split() for l in self.f if l.strip()]

    def used(self):
        return sum(int(l[1]) for l in self.lines() if l[0] != "max")

    def cap(self):
        caps = [int(l[2]) for l in self.lines() if l[0] == "max"]
        return caps[0] if caps else None

    def append(self, *rows):
        for row in rows:
            self.f.write(" ".join(str(x) for x in row) + "\n")
        self.f.flush()
        os.fsync(self.f.fileno())


def admit(path, cap, step, estimate, reviewer):
    cap, estimate, reviewer = int(cap), int(estimate), int(reviewer)
    if cap > CEILING:
        die(f"refused: MAX_TURNS={cap}; the cap is at most {CEILING}")
    ledger = Ledger(path)
    first = ledger.cap()
    if first is None:
        ledger.append(("max", 0, cap))
    elif first != cap:
        die(f"refused: this pass's ledger was opened with MAX_TURNS={first}, not {cap}")
    reserve = estimate + MARGIN + reviewer
    used = ledger.used()
    if used + reserve > cap:
        print(f"turn cap: {used} of {cap} used; {step} may use {reserve}; stopping before it")
        sys.exit(3)
    ledger.append((step, reserve, "reserved"))
    print(reserve)


def book(path, step, reserved, actual):
    ledger = Ledger(path)
    ledger.append((step, -int(reserved), "released"), (step, int(actual), "spent"))
    used, cap = ledger.used(), ledger.cap()
    print(f"{step}: {actual} model turn(s); pass total {used} of {cap}")
    if cap is not None and used >= cap:
        print(f"turn cap reached: {used} of {cap}; the pass stops here")
        sys.exit(3)


def frames(tap):
    """Each OMP process's tapped stdout, oldest process first."""
    if not os.path.isdir(tap):
        return []
    files = [os.path.join(tap, n) for n in os.listdir(tap) if n.endswith(".jsonl")]
    files.sort(key=lambda p: (getattr(os.stat(p), "st_birthtime", 0), p))
    out = []
    for p in files:
        rows = []
        for line in open(p, errors="replace"):
            try:
                rows.append(json.loads(line))
            except ValueError:
                pass
        out.append(rows)
    return out


def count(tap):
    return sum(1 for rows in frames(tap) for r in rows if isinstance(r, dict) and r.get("type") == "agent_start")


def observed(value, out):
    """As cedian_fake_omp::recording::observed_models: get_state's model and
    every message naming its provider and model, as provider/id."""
    if isinstance(value, dict):
        m = value.get("model")
        if isinstance(m, dict) and isinstance(m.get("provider"), str) and isinstance(m.get("id"), str):
            name = f"{m['provider']}/{m['id']}"
            if name not in out:
                out.append(name)
        if isinstance(value.get("provider"), str) and isinstance(m, str):
            name = f"{value['provider']}/{m}"
            if name not in out:
                out.append(name)
        for v in value.values():
            observed(v, out)
    elif isinstance(value, list):
        for v in value:
            observed(v, out)
    return out


def models(tap):
    procs = frames(tap)
    return observed(procs[0], []) if procs else []


def main(argv):
    cmd, args = argv[1], argv[2:]
    if cmd == "admit":
        admit(*args)
    elif cmd == "book":
        book(*args)
    elif cmd == "used":
        print(Ledger(args[0]).used())
    elif cmd == "count":
        print(count(args[0]))
    elif cmd == "models":
        print(" ".join(models(args[0])))
    else:
        die(f"unknown command {cmd}")


if __name__ == "__main__":
    main(sys.argv)
