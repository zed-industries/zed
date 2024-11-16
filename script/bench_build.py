import subprocess

def run_build(crate):
    subprocess.run(["touch", f"crates/{crate}/src/{crate}.rs"])
    result = subprocess.run(["cargo", "build", "--timings"], capture_output=True, text=True)
    output = result.stderr.splitlines()[-1].split(" ")[-1]
    return float(output.rstrip('s'))

def run_bench(crate, n = 5):
    results = []
    for i in range(n):
        results.append(run_build(crate))
        print(f"Run {i+1}/{n} took {results[-1]} seconds")

    average = sum(results) / len(results)
    print(f"Average build time: {average:.2f} seconds")
    return average

run_bench("project")
