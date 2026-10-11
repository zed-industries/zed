#!/usr/bin/env python3
"""The harness's turn cap, OMP wrapper and model check, against a fake OMP.
No model, no real OMP. Run: python3 script/cedian-bench/test_bench.py"""
import json
import os
import shutil
import subprocess
import sys
import tempfile
import unittest

BENCH = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.dirname(os.path.dirname(BENCH))
CAP = [sys.executable, os.path.join(BENCH, "cap.py")]
RUN = os.path.join(BENCH, "run.sh")
PIN = json.load(open(os.path.join(ROOT, "vendor/omp-revision.json")))["spikeVerified"]["ompVersion"]

MODEL = "opencode-go/muse-spark-1.3-contributor"

# `config` and `models` answer the harness's pre-turn checks; BENCH_FAKE_MODEL
# and BENCH_FAKE_CATALOG let a test make either one disagree.
FAKE_OMP = """#!/bin/sh
if [ "$1" = --version ]; then echo omp/%(pin)s; exit 0; fi
if [ "$1" = config ]; then echo "{\\"value\\":{\\"default\\":\\"${BENCH_FAKE_MODEL:-%(model)s}\\"}}"; exit 0; fi
if [ "$1" = models ]; then echo "{\\"models\\":[{\\"provider\\":\\"opencode-go\\",\\"id\\":\\"muse-spark-1.3-contributor\\",\\"selector\\":\\"${BENCH_FAKE_CATALOG:-%(model)s}\\"}]}"; exit 0; fi
python3 -c 'import json,sys; print(json.dumps({"type": "args", "argv": sys.argv[1:]}))' "$@"
echo '{"type":"response","command":"get_state","data":{"model":{"provider":"p","id":"m"}}}'
echo '{"type":"agent_start"}'
echo '{"type":"agent_start"}'
""" % {"pin": PIN, "model": MODEL}


class Bench(unittest.TestCase):
    def setUp(self):
        # Not a temp dir: the ledger refuses one (a reboot clears it).
        base = os.path.join(ROOT, "target", "cedian-bench-test")
        os.makedirs(base, exist_ok=True)
        self.dir = tempfile.mkdtemp(dir=base)
        self.ledger = os.path.join(self.dir, "pass", "turns.log")
        self.omp = os.path.join(self.dir, "omp")
        with open(self.omp, "w") as f:
            f.write(FAKE_OMP)
        os.chmod(self.omp, 0o755)

    def tearDown(self):
        shutil.rmtree(self.dir)

    def cap(self, *args):
        return subprocess.run(CAP + [str(a) for a in args], capture_output=True, text=True)

    def bench(self, *args, **env):
        e = dict(os.environ, BENCH_RESULTS=self.dir, BENCH_PASS="pass", CEDIAN_OMP_BINARY=self.omp)
        e.update(env)
        return subprocess.run([RUN, *args], capture_output=True, text=True, env=e)

    def test_the_cap_cannot_exceed_46(self):
        self.assertEqual(self.cap("admit", self.ledger, 47, "s", 1, 0).returncode, 2)
        r = self.bench("turns", "all", MAX_TURNS="47")
        self.assertEqual(r.returncode, 2, r.stdout + r.stderr)

    def test_the_model_check_needs_the_pass_model_in_omps_catalog(self):
        r = self.bench("model", "all")
        self.assertEqual(r.returncode, 0, r.stdout + r.stderr)
        self.assertIn(MODEL, r.stdout)
        r = self.bench("model", "all", BENCH_FAKE_CATALOG="opencode-go/other")
        self.assertEqual(r.returncode, 2, r.stdout)
        self.assertIn(MODEL, r.stderr)
        # The person's own default is theirs (ADR-0061): it does not decide
        # the pass, and the check says what the pass pins instead.
        r = self.bench(
            "model", "all", BENCH_FAKE_MODEL="commandcode/deepseek/deepseek-v4.1-flash"
        )
        self.assertEqual(r.returncode, 0, r.stdout + r.stderr)
        self.assertIn("commandcode/deepseek/deepseek-v4.1-flash", r.stderr)
        self.assertIn(f"the pass pins {MODEL}", r.stderr)

    def test_the_wrapper_pins_the_pass_model_for_both_mandates(self):
        both = (
            ("on", ["--model", MODEL]),
            ("off", ["--model", MODEL, "--append-system-prompt", ""]),
        )
        for mandate, tail in both:
            tap = os.path.join(self.dir, f"tap-{mandate}")
            wrapper = self.bench("wrapper", mandate, BENCH_TAP=tap).stdout.strip()
            out = subprocess.run(
                [wrapper, "--mode", "rpc"], capture_output=True, text=True, check=True
            ).stdout
            argv = json.loads(out.splitlines()[0])["argv"]
            self.assertEqual(argv[-len(tail) :], tail, argv)
        # A non-rpc call (the `--version` pin check) gets no flags.
        tap = os.path.join(self.dir, "tap-version")
        wrapper = self.bench("wrapper", "on", BENCH_TAP=tap).stdout.strip()
        out = subprocess.run(
            [wrapper, "--version"], capture_output=True, text=True, check=True
        ).stdout
        self.assertEqual(out.strip(), f"omp/{PIN}")

    def test_the_ledger_is_keyed_by_pass_outside_tmp(self):
        r = self.bench("turns", "all", BENCH_RESULTS="", HOME=self.dir)
        self.assertIn(os.path.join(self.dir, ".cache/cedian-bench/pass/turns.log"), r.stdout, r.stderr)
        self.assertEqual(self.cap("admit", "/tmp/cedian-bench-x/turns.log", 40, "s", 1, 0).returncode, 2)
        r = self.bench("turns", "all", BENCH_PASS="")
        self.assertEqual(r.returncode, 2, r.stdout)

    def test_a_step_is_admitted_only_if_its_worst_case_fits(self):
        self.cap("admit", self.ledger, 40, "seed", 36, 0)  # reserves 37
        self.cap("book", self.ledger, "seed", 37, 37)
        # estimate 1 + margin 1 + reviewer 1 = 3 fits 37 + 3 = 40
        self.assertEqual(self.cap("admit", self.ledger, 40, "b1", 1, 1).returncode, 0)
        self.cap("book", self.ledger, "b1", 3, 1)  # used 38
        self.assertEqual(self.cap("admit", self.ledger, 40, "b2", 1, 1).returncode, 3)
        self.assertEqual(self.cap("admit", self.ledger, 40, "b2", 1, 0).returncode, 0)

    def test_concurrent_invocations_cannot_both_pass(self):
        procs = [subprocess.Popen(CAP + ["admit", self.ledger, "20", f"s{i}", "1", "0"],
                                  stdout=subprocess.PIPE, stderr=subprocess.PIPE) for i in range(30)]
        admitted = sum(1 for p in procs if p.wait() == 0)
        self.assertEqual(admitted, 10)  # 10 x (1 + margin) = 20
        self.assertEqual(self.cap("used", self.ledger).stdout.strip(), "20")

    def test_a_pass_opened_with_one_cap_keeps_it(self):
        self.cap("admit", self.ledger, 10, "s", 1, 0)
        self.assertEqual(self.cap("admit", self.ledger, 40, "t", 1, 0).returncode, 2)

    def test_booking_counts_every_process_and_stops_at_the_cap(self):
        tap = os.path.join(self.dir, "tap")
        r = self.bench("wrapper", "on", BENCH_TAP=tap)
        self.assertEqual(r.returncode, 0, r.stderr)
        wrapper = r.stdout.strip()
        for _ in range(2):  # the agent, then a reviewer it spawned
            subprocess.run([wrapper, "--mode", "rpc"], capture_output=True, check=True)
        subprocess.run([wrapper, "--version"], capture_output=True, check=True)
        self.assertEqual(self.cap("count", tap).stdout.strip(), "4")
        self.cap("admit", self.ledger, 5, "b1", 1, 1)
        booked = self.cap("book", self.ledger, "b1", 3, 4)
        self.assertEqual(booked.returncode, 0, booked.stdout)
        self.assertEqual(self.cap("used", self.ledger).stdout.strip(), "4")
        self.cap("admit", self.ledger, 5, "b2", 0, 0)
        self.assertEqual(self.cap("book", self.ledger, "b2", 1, 1).returncode, 3)

    def test_the_observed_model_is_the_first_process(self):
        tap = os.path.join(self.dir, "tap")
        wrapper = self.bench("wrapper", "on", BENCH_TAP=tap).stdout.strip()
        subprocess.run([wrapper, "--mode", "rpc"], capture_output=True, check=True)
        self.assertEqual(self.cap("models", tap).stdout.split(), ["p/m"])
        self.assertEqual(self.cap("models", os.path.join(self.dir, "none")).stdout.split(), [])

    def test_mandate_off_empties_the_prompt_wherever_mode_is(self):
        tap = os.path.join(self.dir, "tap")
        wrapper = self.bench("wrapper", "off", BENCH_TAP=tap).stdout.strip()
        for argv in (["--mode", "rpc"], ["--cwd", "x", "--mode", "rpc"], ["--mode=rpc"]):
            out = subprocess.run([wrapper, *argv], capture_output=True, text=True, check=True).stdout
            args = json.loads(out.splitlines()[0])["argv"]
            self.assertEqual(args[-2:], ["--append-system-prompt", ""], argv)
        out = subprocess.run([wrapper, "--version"], capture_output=True, text=True).stdout
        self.assertEqual(out.strip(), f"omp/{PIN}")


if __name__ == "__main__":
    unittest.main()
