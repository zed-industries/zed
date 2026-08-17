#!/usr/bin/python3

"""
Benchmarking script. See --help for details. Compiles benchmark, runs and outputs csv.
"""

import argparse
import os
import subprocess
import sys
import time

_POST_COMPILE_SLEEP = 0.5

_GEMMTYPE = {
    'f32': 'SGEMM',
    'f64': 'DGEMM',
    'c32': 'CGEMM',
    'c64': 'ZGEMM',
}

_COMPILE = "cargo rustc --example benchmark --release".split()
_DEFAULT_FEATURES = "constconf cgemm".split()
_EXEC = "./target/release/examples/benchmark"


def bench_loop(args, *, file):
    extra_header = ",nc,kc,mc,threads"
    print("m,k,n,layout,type,average_ns,minimum_ns,median_ns,samples,gflops", extra_header, sep="", file=file)
    ncs = [None] if args.nc is None else args.nc
    kcs = [None] if args.kc is None else args.kc
    mcs = [None] if args.mc is None else args.mc
    for threads in args.threads:
        for ty in args.type:
            for nc in ncs:
                for kc in kcs:
                    for mc in mcs:
                        bench_iteration(args.size, ty, nc, kc, mc, threads=threads, file=file, sleep=args.sleep)


def bench_iteration(sizes, ty, nc, kc, mc, *, threads, file, sleep):
    features = list(_DEFAULT_FEATURES)
    if threads > 0:
        features.append("threading")
    compile_argv = list(_COMPILE)
    compile_argv.append("--features=" + ",".join(features))
    file.flush()
    env = os.environ.copy()
    for value, name in zip([nc, kc, mc], ["nc", "kc", "mc"]):
        if value is not None:
            env["MATMUL_" + _GEMMTYPE[ty] + "_" + name.upper()] = str(value)

    print("Running", " ".join(compile_argv), file=sys.stderr)
    subprocess.run(compile_argv, env=env)

    time.sleep(_POST_COMPILE_SLEEP)

    exec_env = os.environ.copy()
    exec_env["MATMUL_NUM_THREADS"] = str(threads)
    extra_column = ",".join((str(value) if value is not None else "") for value in [nc, kc, mc, threads])
    for size in sizes:
        argv = [_EXEC]
        argv.extend(["--type", ty])
        argv.extend(["--csv", "--layout", "fcc", "--extra-column", extra_column])
        argv.extend([str(size)] * 3)

        print("Running", " ".join(argv), file=sys.stderr)
        subprocess.run(argv, env=exec_env, stdout=file)
        time.sleep(sleep)


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--type", "-t", type=str, default=["f64"], choices=["f64", "f32", "c64", "c32"], nargs="+")
    parser.add_argument("--size", "-s", type=str, nargs="+", required=True,
                        help="Sizes or size ranges like 16 or 16:64:8")
    parser.add_argument("--nc", type=str, nargs="+", help="Sizes or size ranges like 16 or 16:64:8")
    parser.add_argument("--kc", type=str, nargs="+", help="Sizes or size ranges like 16 or 16:64:8")
    parser.add_argument("--mc", type=str, nargs="+", help="Sizes or size ranges like 16 or 16:64:8")
    parser.add_argument("--threads", type=int, default=[0], nargs="+",
                        help="Thread use. 0: not enabled; 1: enabled but one thread; n: enabled with n threads.")
    parser.add_argument("--sleep", type=int, default=1, help="Time to wait between every run")
    parser.add_argument("--output", type=str, default=None, help="Output file (csv format)")
    args = parser.parse_args()

    if len(args.type) > 1 and any([args.nc, args.kc, args.mc]):
        print("Warning: Combining type loop with nc, mc, kc might not make sense", file=sys.stderr)

    # postprocess nc, kc, mc to parse x:y:z into ranges
    for var in ['size', 'nc', 'kc', 'mc']:
        arg_values = getattr(args, var)
        if arg_values is not None:
            new_arg_values = []
            for arg_value in arg_values:
                if ":" in arg_value:
                    parts = list(int(v) for v in arg_value.split(":"))
                    values = list(range(*parts))
                    new_arg_values.extend(values)
                else:
                    new_arg_values.append(arg_value)
            setattr(args, var, new_arg_values)

    if args.output is not None:
        with open(args.output, "w") as f:
            bench_loop(args, file=f)
    else:
        bench_loop(args, file=sys.stdout)


if __name__ == "__main__":
    main()
