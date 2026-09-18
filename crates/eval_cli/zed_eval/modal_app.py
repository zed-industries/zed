from __future__ import annotations

import hashlib
import json
import os
import pathlib
import shlex
import shutil
import subprocess
import tarfile
import time
import traceback
import uuid
from typing import Any

import modal

from . import cleanup, config, harness_command, rejudge, source
from .common import load_json, safe_extract_archive, utc_now, write_json
from .source import (
    BUILD_TARGET,
    CARGO_ZIGBUILD_VERSION,
    RUST_IMAGE,
    RUST_VERSION,
    ZIG_VERSION,
    build_toolchain_info,
)

APP_NAME = os.environ.get("AGENT_EVALS_APP_NAME", "agent-evals")
VOLUME_NAME = os.environ.get("AGENT_EVALS_VOLUME", "agent-evals")
MODAL_TOKEN_SECRET_NAME = os.environ.get(
    "AGENT_EVALS_MODAL_TOKEN_SECRET", "agent-evals-modal-token"
)
# LLM-providers secret mounted into the rejudge controller so the in-controller
# judge proxy can read the judge's API key (e.g. BASETEN_API_KEY) the same way a
# trial sandbox does. Override at deploy time if the secret is named differently.
LLM_PROVIDERS_SECRET_NAME = os.environ.get(
    "AGENT_EVALS_LLM_PROVIDERS_SECRET", config.DEFAULT_LLM_PROVIDERS_SECRET_NAME
)
REPO_URL = os.environ.get(
    "AGENT_EVALS_REPO_URL", "https://github.com/zed-industries/zed.git"
)
ZIG_URL = (
    f"https://ziglang.org/download/{ZIG_VERSION}/zig-x86_64-linux-{ZIG_VERSION}.tar.xz"
)
CLEANUP_BUILD_RETENTION_DAYS = float(
    os.environ.get("AGENT_EVALS_BUILD_RETENTION_DAYS", "14")
)

app = modal.App(APP_NAME)
volume = modal.Volume.from_name(VOLUME_NAME, create_if_missing=True)


def reload_volume() -> None:
    try:
        volume.reload()
    except Exception:
        pass


build_image = (
    modal.Image.from_registry(RUST_IMAGE, add_python="3.13")
    .apt_install("cmake", "build-essential", "curl", "xz-utils", "git")
    .run_commands(
        f"rustup toolchain install {RUST_VERSION} --profile minimal"
        " --component rustfmt --component clippy"
        " --component rust-analyzer --component rust-src"
        " --target wasm32-wasip2 --target wasm32-unknown-unknown"
        " --target x86_64-unknown-linux-musl --target x86_64-unknown-linux-gnu",
        f"mkdir -p /opt/zig && curl -fsSL {ZIG_URL}"
        " | tar -xJ -C /opt/zig --strip-components=1"
        " && ln -s /opt/zig/zig /usr/local/bin/zig",
        f"cargo install --locked cargo-zigbuild --version {CARGO_ZIGBUILD_VERSION}",
    )
)

controller_image = (
    modal.Image.debian_slim(python_version="3.13")
    .apt_install("bash", "ca-certificates", "curl", "git", "tar")
    .pip_install(f"modal=={config.MODAL_VERSION}", "uv")
    .run_commands(
        f"uv tool install harbor=={config.HARBOR_VERSION}"
        f" --with 'modal=={config.MODAL_VERSION}' && "
        "ln -sf /root/.local/bin/harbor /usr/local/bin/harbor",
        # Pier is a Harbor fork that adds per-agent network allowlists, required
        # to run DeepSWE's air-gapped (`allow_internet = false`) tasks while
        # still letting eval-cli reach the model API. Kept non-fatal so a Pier
        # install problem can't break the Harbor benchmarks that share this image;
        # DeepSWE runs surface a clear "pier not found" error at runtime instead.
        "(uv tool install git+https://github.com/datacurve-ai/pier"
        f" --with 'modal=={config.MODAL_VERSION}' && "
        "ln -sf /root/.local/bin/pier /usr/local/bin/pier) || "
        "echo 'WARNING: pier install failed; DeepSWE runs will not work'",
    )
)


def provision_benchmark_dataset(run_request: dict[str, Any], log: Any) -> None:
    """Clone a path-backed benchmark dataset (SWE-Atlas tw, DeepSWE) into the
    location the harness command expects. Registry datasets need no provisioning;
    the harness pulls them from its hub."""
    benchmark = run_request["benchmark"]
    dataset = benchmark.get("dataset") or {}
    kind = dataset.get("kind")
    if kind not in ("path", "pier_path"):
        return

    repo_url = dataset.get("repo_url")
    repo_ref = dataset.get("repo_ref") or "main"
    if not repo_url:
        raise ValueError(f"benchmark {benchmark['id']} path dataset requires repo_url")

    clone_dir = pathlib.Path(harness_command.dataset_clone_dir(benchmark))
    if clone_dir.exists():
        shutil.rmtree(clone_dir)
    clone_dir.mkdir(parents=True, exist_ok=True)
    log(f"Fetching {benchmark['id']} dataset {repo_url}@{repo_ref}")
    subprocess.run(["git", "init", "-q", str(clone_dir)], check=True)
    subprocess.run(
        ["git", "fetch", "--depth", "1", repo_url, repo_ref],
        cwd=clone_dir,
        check=True,
    )
    subprocess.run(["git", "checkout", "-q", "FETCH_HEAD"], cwd=clone_dir, check=True)
    data_dir = pathlib.Path(harness_command.dataset_path(benchmark))
    if not data_dir.exists():
        raise FileNotFoundError(f"benchmark dataset directory not found: {data_dir}")


def output_of(command: list[str]) -> str:
    try:
        return subprocess.run(
            command,
            check=True,
            capture_output=True,
            text=True,
        ).stdout.strip()
    except subprocess.CalledProcessError as error:
        return f"unknown ({error})"


@app.function(
    image=build_image,
    # Right-sized from the original 16 cpu / 32 GB after observing peak usage of
    # ~10 cores and ~10 GB. ephemeral_disk stays at Modal's 512 GiB floor (the
    # minimum allowed for a function), so it isn't a tunable here.
    cpu=12,
    memory=24576,
    ephemeral_disk=524288,
    timeout=7200,
    volumes={"/data": volume},
)
def build_eval_cli(build_request: dict[str, Any]) -> dict[str, Any]:
    def run(command: str, **kwargs: Any) -> None:
        print(f"+ {command}", flush=True)
        subprocess.run(command, shell=True, check=True, **kwargs)

    reload_volume()

    build_id = build_request["build_id"]
    source_info = build_request.get("source") or {"type": "git_patch"}
    base_sha = source_info.get("base_sha") or build_request["base_sha"]
    patch = build_request.get("patch") or ""
    build_dir = pathlib.Path("/data/builds") / build_id
    ready_path = build_dir / "READY"
    binary_path = build_dir / "eval-cli"
    build_info_path = build_dir / "build-info.json"
    # The lease lives outside build_dir so it never interferes with the atomic
    # move of the finished build directory below.
    building_path = pathlib.Path("/data/build-locks") / f"{build_id}.json"
    lease_ttl = int(build_request.get("build_wait_timeout_secs") or 7200)
    owner = uuid.uuid4().hex

    def reuse_existing() -> dict[str, Any]:
        build_info = load_json(build_info_path) or {}
        build_info.setdefault("build_id", build_id)
        build_info["reused"] = True
        volume.commit()
        print(f"Reusing existing build {build_id}", flush=True)
        return build_info

    if ready_path.exists() and binary_path.exists():
        return reuse_existing()
    if build_dir.exists():
        raise RuntimeError(
            f"build directory already exists but is not ready; refusing to overwrite {build_id}"
        )

    # Single-flight lease: if another invocation is already compiling this exact
    # build, wait for it to finish rather than running a second multi-minute
    # compile. The lease is best-effort (the volume has no atomic compare-and-swap);
    # the atomic move below still guarantees correctness if two builds slip through.
    deadline = time.time() + lease_ttl
    while True:
        reload_volume()
        if ready_path.exists() and binary_path.exists():
            return reuse_existing()
        if build_dir.exists():
            raise RuntimeError(
                f"build directory already exists but is not ready; refusing to overwrite {build_id}"
            )
        lease = load_json(building_path)
        now = time.time()
        held_by_other = (
            isinstance(lease, dict)
            and lease.get("owner") != owner
            and (now - float(lease.get("epoch") or 0)) < lease_ttl
        )
        if held_by_other and time.time() < deadline:
            print(
                f"Build {build_id} is being compiled elsewhere; waiting",
                flush=True,
            )
            time.sleep(15)
            continue
        break
    # Claim the lease (write_json creates /data/build-locks). build_dir is
    # intentionally NOT created here so the atomic move below still works.
    write_json(building_path, {"owner": owner, "epoch": time.time(), "at": utc_now()})
    volume.commit()

    rustc_version = output_of(["rustc", "--version"])
    zig_version = output_of(["zig", "version"])
    cargo_zigbuild_version = output_of(["cargo-zigbuild", "--version"])

    workdir = pathlib.Path("/build/zed")
    if workdir.exists():
        shutil.rmtree(workdir)
    workdir.mkdir(parents=True, exist_ok=True)
    os.chdir(workdir)

    repo_url = source_info.get("repo_url") or REPO_URL
    run("git init -q .")
    run(f"git fetch --depth 1 {shlex.quote(repo_url)} {shlex.quote(base_sha)}")
    run("git checkout -q FETCH_HEAD")

    if patch.strip():
        patch_file = pathlib.Path("/build/source.patch")
        patch_file.write_text(patch)
        run(f"git apply --stat {shlex.quote(str(patch_file))}")
        run(f"git apply {shlex.quote(str(patch_file))}")

    patch_sha256 = source_info.get("patch_sha256")

    run(
        "cargo zigbuild --release --package eval_cli --target x86_64-unknown-linux-musl"
    )

    built = workdir / "target/x86_64-unknown-linux-musl/release/eval-cli"
    run(f"strip {built}")
    binary_bytes = built.read_bytes()
    binary_sha256 = hashlib.sha256(binary_bytes).hexdigest()

    temporary_dir = pathlib.Path("/data/tmp/builds") / f"{build_id}-{uuid.uuid4().hex}"
    temporary_dir.mkdir(parents=True, exist_ok=True)
    shutil.copy(built, temporary_dir / "eval-cli")
    (temporary_dir / "eval-cli").chmod(0o755)
    if patch.strip():
        (temporary_dir / "source.patch").write_text(patch)
    write_json(temporary_dir / "source-info.json", source_info)

    build_info = {
        "build_id": build_id,
        "base_sha": base_sha,
        "patch_sha256": patch_sha256,
        "rustc_version": rustc_version,
        "zig_version": zig_version,
        "cargo_zigbuild_version": cargo_zigbuild_version,
        "built_at_utc": utc_now(),
        "binary_size_bytes": len(binary_bytes),
        "binary_sha256": binary_sha256,
        "target": BUILD_TARGET,
        "rust_image": RUST_IMAGE,
        "toolchain": build_toolchain_info(),
        "source": source_info,
    }
    write_json(temporary_dir / "build-info.json", build_info)
    (temporary_dir / "READY").write_text(utc_now() + "\n")

    if not build_dir.exists():
        build_dir.parent.mkdir(parents=True, exist_ok=True)
        shutil.move(str(temporary_dir), str(build_dir))
    elif ready_path.exists() and binary_path.exists():
        shutil.rmtree(temporary_dir, ignore_errors=True)
        build_info = load_json(build_info_path) or build_info
        build_info["reused_after_race"] = True
    else:
        shutil.rmtree(temporary_dir, ignore_errors=True)
        raise RuntimeError(
            f"build directory already exists but is not ready; refusing to overwrite {build_id}"
        )

    # Release the single-flight lease now that READY exists.
    try:
        building_path.unlink()
    except OSError:
        pass
    volume.commit()
    print(f"Committed build {build_id} to volume '{VOLUME_NAME}'", flush=True)
    return build_info


@app.function(
    image=controller_image,
    cpu=1,
    memory=512,
    timeout=300,
    volumes={"/data": volume},
)
def list_builds(limit: int = 50) -> list[dict[str, Any]]:
    reload_volume()
    builds_dir = pathlib.Path("/data/builds")
    rows = []
    for build_dir in builds_dir.iterdir() if builds_dir.exists() else []:
        if not build_dir.is_dir():
            continue
        build_info = load_json(build_dir / "build-info.json") or {}
        ready = (build_dir / "READY").exists() and (build_dir / "eval-cli").exists()
        rows.append(
            {
                "build_id": build_dir.name,
                "ready": ready,
                "base_sha": build_info.get("base_sha"),
                "patch_sha256": build_info.get("patch_sha256"),
                "built_at_utc": build_info.get("built_at_utc"),
                "binary_sha256": build_info.get("binary_sha256"),
                "source": build_info.get("source"),
            }
        )
    rows.sort(key=lambda row: row.get("built_at_utc") or "", reverse=True)
    return rows[:limit]


def run_row(
    namespace_dir: pathlib.Path, experiment_dir: pathlib.Path, run_dir: pathlib.Path
) -> dict[str, Any]:
    state = load_json(run_dir / "state.json") or {}
    request = load_json(run_dir / "request.json") or {}
    metadata = load_json(run_dir / "run-metadata.json") or {}
    summary = load_json(run_dir / "summary.json") or {}
    benchmark = request.get("benchmark") or metadata.get("benchmark") or {}
    benchmark_id = benchmark.get("id") if isinstance(benchmark, dict) else None
    suite_id = request.get("suite_id") or metadata.get("suite_id")
    return {
        "namespace": namespace_dir.name,
        "experiment_name": experiment_dir.name,
        "run_id": run_dir.name,
        "status": state.get("status"),
        "updated_at": state.get("updated_at"),
        "created_at": metadata.get("created_at")
        or state.get("created_at")
        or request.get("created_at"),
        "agent_model": metadata.get("agent_model"),
        "judge_preset": metadata.get("judge_preset"),
        "judge_model": metadata.get("judge_model"),
        "build_id": metadata.get("build_id") or state.get("build_id"),
        "trial_count": summary.get("trial_count"),
        "has_archive": summary.get("has_archive"),
        "suite_id": suite_id,
        "part": request.get("suite_part"),
        "benchmark": benchmark_id,
    }


def scan_runs(
    *,
    namespace: str | None = None,
    experiment_name: str | None = None,
    suite_id: str | None = None,
) -> list[dict[str, Any]]:
    runs_root = pathlib.Path("/data/runs")
    rows = []
    if not runs_root.exists():
        return []
    namespaces = (
        [runs_root / namespace] if namespace else [path for path in runs_root.iterdir()]
    )
    for namespace_dir in namespaces:
        if not namespace_dir.is_dir():
            continue
        experiments = (
            [namespace_dir / experiment_name]
            if experiment_name
            else [path for path in namespace_dir.iterdir()]
        )
        for experiment_dir in experiments:
            if not experiment_dir.is_dir():
                continue
            for run_dir in experiment_dir.iterdir():
                if not run_dir.is_dir():
                    continue
                row = run_row(namespace_dir, experiment_dir, run_dir)
                if suite_id and row.get("suite_id") != suite_id:
                    continue
                rows.append(row)
    rows.sort(
        key=lambda row: row.get("updated_at") or row.get("created_at") or "",
        reverse=True,
    )
    return rows


@app.function(
    image=controller_image,
    cpu=1,
    memory=512,
    timeout=300,
    volumes={"/data": volume},
)
def list_runs(
    namespace: str | None = None,
    experiment_name: str | None = None,
    limit: int = 50,
) -> list[dict[str, Any]]:
    reload_volume()
    rows = scan_runs(namespace=namespace, experiment_name=experiment_name)
    return rows[:limit]


def _run_cleanup(
    *,
    dry_run: bool,
    build_retention_days: float | None = None,
) -> dict[str, Any]:
    reload_volume()
    result = cleanup.prune_artifacts(
        pathlib.Path("/data"),
        dry_run=dry_run,
        build_retention_days=(
            build_retention_days
            if build_retention_days is not None
            else CLEANUP_BUILD_RETENTION_DAYS
        ),
    )
    if not dry_run:
        volume.commit()
    print(json.dumps(result, indent=2), flush=True)
    return result


@app.function(
    image=controller_image,
    cpu=1,
    memory=2048,
    timeout=1800,
    volumes={"/data": volume},
    schedule=modal.Period(days=1),
)
def cleanup_scheduled() -> dict[str, Any]:
    """Daily prune of stale build artifacts. Never touches eval results."""
    return _run_cleanup(dry_run=False)


@app.function(
    image=controller_image,
    cpu=1,
    memory=2048,
    timeout=1800,
    volumes={"/data": volume},
)
def cleanup_artifacts(request: dict[str, Any]) -> dict[str, Any]:
    """On-demand prune (the `zed-eval cleanup` command), supporting --dry-run
    and retention overrides."""
    return _run_cleanup(
        dry_run=bool(request.get("dry_run")),
        build_retention_days=request.get("build_retention_days"),
    )


@app.function(
    image=controller_image,
    cpu=1,
    memory=512,
    timeout=300,
    volumes={"/data": volume},
)
def read_run_provenance(
    namespace: str, experiment_name: str, run_id: str
) -> dict[str, Any]:
    """Provenance a baseline record needs for one run: the launch request
    (model/judge/resources/build_id), the run summary (status), and the build's
    build-info (base_sha/patch_sha256/source label)."""
    reload_volume()
    run_dir = pathlib.Path("/data/runs") / namespace / experiment_name / run_id
    request = load_json(run_dir / "request.json") or {}
    summary = load_json(run_dir / "summary.json") or {}
    metadata = load_json(run_dir / "run-metadata.json") or {}
    build_id = request.get("build_id") or metadata.get("build_id")
    build_info: dict[str, Any] = {}
    if build_id:
        build_info = (
            load_json(pathlib.Path("/data/builds") / build_id / "build-info.json") or {}
        )
    return {
        "request": request,
        "summary": summary,
        "metadata": metadata,
        "build_info": build_info,
    }


@app.function(
    image=controller_image,
    cpu=1,
    memory=512,
    timeout=300,
    volumes={"/data": volume},
)
def record_baseline(record: dict[str, Any]) -> dict[str, Any]:
    """Write a baseline-of-record keyed by (experiment, model). Supersedes any
    existing current baseline (archived under history/) and refreshes the
    discoverable index."""
    reload_volume()
    experiment_slug = source.sanitize_namespace(record["experiment"])
    model_slug = source.sanitize_namespace(record.get("model_slug") or record["model"])
    base_dir = pathlib.Path("/data/baselines") / experiment_slug / model_slug
    current_path = base_dir / "current.json"

    existing = load_json(current_path)
    if existing:
        previous_sha = source.sanitize_namespace(
            str(existing.get("base_sha") or "unknown")
        )
        write_json(base_dir / "history" / f"{previous_sha}.json", existing)

    payload = {**record, "updated_at": utc_now()}
    write_json(current_path, payload)

    index_path = pathlib.Path("/data/baselines/index.json")
    index = load_json(index_path) or {}
    entries = index.get("baselines")
    if not isinstance(entries, list):
        entries = []
    entries = [
        entry
        for entry in entries
        if not (
            entry.get("experiment") == record["experiment"]
            and entry.get("model") == record["model"]
        )
    ]
    entries.append(
        {
            "experiment": record["experiment"],
            "model": record["model"],
            "base_sha": record.get("base_sha"),
            "base_ref": record.get("base_ref"),
            "on_main": record.get("on_main"),
            "clean": record.get("clean"),
            "judge": record.get("judge"),
            "run_id": (record.get("run") or {}).get("run_id"),
            "recorded_at": record.get("recorded_at"),
            "path": f"baselines/{experiment_slug}/{model_slug}/current.json",
        }
    )
    entries.sort(
        key=lambda entry: (entry.get("experiment") or "", entry.get("model") or "")
    )
    write_json(index_path, {"baselines": entries})
    volume.commit()
    return payload


@app.function(
    image=controller_image,
    cpu=1,
    memory=512,
    timeout=300,
    volumes={"/data": volume},
)
def read_baselines(
    experiment: str | None = None,
    model_slug: str | None = None,
    include_history: bool = False,
) -> dict[str, Any]:
    """List all current baselines (no args) or show one (experiment+model_slug),
    optionally with superseded history."""
    reload_volume()
    root = pathlib.Path("/data/baselines")
    if experiment and model_slug:
        base_dir = (
            root
            / source.sanitize_namespace(experiment)
            / source.sanitize_namespace(model_slug)
        )
        result: dict[str, Any] = {"current": load_json(base_dir / "current.json")}
        if include_history:
            history = []
            history_dir = base_dir / "history"
            if history_dir.is_dir():
                for path in sorted(history_dir.glob("*.json")):
                    entry = load_json(path)
                    if entry:
                        history.append(entry)
            result["history"] = history
        return result
    return load_json(root / "index.json") or {"baselines": []}


@app.function(
    image=controller_image,
    cpu=1,
    memory=512,
    timeout=300,
    volumes={"/data": volume},
)
def suite_status(namespace: str, suite_id: str) -> list[dict[str, Any]]:
    reload_volume()
    rows = scan_runs(namespace=namespace, suite_id=suite_id)
    if not rows:
        raise FileNotFoundError(f"suite not found: {namespace}/{suite_id}")
    rows.sort(key=lambda row: row.get("created_at") or row.get("run_id") or "")
    return rows


def write_run_inputs(run_dir: pathlib.Path, run_request: dict[str, Any]) -> None:
    write_json(run_dir / "request.json", run_request)
    write_json(run_dir / "run-metadata.json", harness_command.run_metadata(run_request))
    task_names = run_request.get("task_names") or []
    (run_dir / "selected-tasks.txt").write_text(
        "\n".join(task_names) + ("\n" if task_names else "")
    )


@app.function(
    image=controller_image,
    cpu=1,
    memory=512,
    timeout=300,
    volumes={"/data": volume},
)
def create_run_record(run_request: dict[str, Any]) -> dict[str, Any]:
    namespace = run_request["namespace"]
    experiment_name = run_request["experiment_name"]
    run_id = run_request["run_id"]
    run_dir = pathlib.Path("/data/runs") / namespace / experiment_name / run_id
    state_path = run_dir / "state.json"

    reload_volume()

    if state_path.exists():
        raise FileExistsError(
            f"run record already exists: {namespace}/{experiment_name}/{run_id}"
        )

    run_dir.mkdir(parents=True, exist_ok=True)
    write_run_inputs(run_dir, run_request)
    state = {
        "run_id": run_id,
        "namespace": namespace,
        "experiment_name": experiment_name,
        "status": "pending",
        "created_at": run_request.get("created_at"),
        "updated_at": utc_now(),
        "build_id": run_request.get("build_id"),
    }
    write_json(state_path, state)
    volume.commit()
    return state


@app.function(
    image=controller_image,
    cpu=2,
    memory=4096,
    timeout=86_400,
    volumes={"/data": volume},
    secrets=[modal.Secret.from_name(MODAL_TOKEN_SECRET_NAME)],
)
def run_controller(run_request: dict[str, Any]) -> dict[str, Any]:
    namespace = run_request["namespace"]
    experiment_name = run_request["experiment_name"]
    run_id = run_request["run_id"]
    run_dir = pathlib.Path("/data/runs") / namespace / experiment_name / run_id
    run_dir.mkdir(parents=True, exist_ok=True)
    log_path = run_dir / "controller.log"
    state_path = run_dir / "state.json"
    started_at = utc_now()

    def commit() -> None:
        volume.commit()

    def log(message: str) -> None:
        line = f"[{utc_now()}] {message}"
        print(line, flush=True)
        with log_path.open("a") as log_file:
            log_file.write(line + "\n")

    def state(status: str, **extra: Any) -> None:
        payload = {
            "run_id": run_id,
            "namespace": namespace,
            "experiment_name": experiment_name,
            "status": status,
            "updated_at": utc_now(),
            "started_at": started_at,
            **extra,
        }
        write_json(state_path, payload)
        commit()

    try:
        write_run_inputs(run_dir, run_request)
        state("starting", build_id=run_request.get("build_id"))
        log(f"Starting run {namespace}/{experiment_name}/{run_id}")

        build_id = run_request.get("build_id")
        if build_id:
            build_dir = pathlib.Path("/data/builds") / build_id
            ready_path = build_dir / "READY"
            build_info_path = build_dir / "build-info.json"
            source_patch_path = build_dir / "source.patch"
            source_info_path = build_dir / "source-info.json"
            state("waiting_for_build", build_id=build_id)
            deadline = time.time() + int(
                run_request.get("build_wait_timeout_secs") or 7200
            )
            while not ready_path.exists():
                if time.time() >= deadline:
                    raise TimeoutError(
                        f"build {build_id} was not ready before the wait timeout"
                    )
                log(f"Waiting for build {build_id} to become ready")
                time.sleep(30)
                reload_volume()
            shutil.copy(build_info_path, run_dir / "build-info.json")
            if source_patch_path.exists():
                shutil.copy(source_patch_path, run_dir / "source.patch")
            if source_info_path.exists():
                shutil.copy(source_info_path, run_dir / "source-info.json")
            log(f"Using build {build_id}")

        provision_benchmark_dataset(run_request, log)

        jobs_parent = pathlib.Path("/tmp/agent-evals/harbor-jobs")
        jobs_parent.mkdir(parents=True, exist_ok=True)
        command = harness_command.build_harness_command(run_request, str(jobs_parent))
        redacted = config.redacted_command(command)
        (run_dir / "harbor-command.txt").write_text(redacted + "\n")
        state(
            "running",
            build_id=build_id,
            harness_command=redacted,
        )
        commit()
        log("Launching Harbor")
        log(redacted)

        import zed_eval

        package_file = zed_eval.__file__
        if package_file is None:
            raise RuntimeError("could not resolve zed_eval package location")
        package_parent = str(pathlib.Path(package_file).resolve().parent.parent)
        environment = os.environ.copy()
        existing_pythonpath = environment.get("PYTHONPATH")
        environment["PYTHONPATH"] = (
            package_parent
            if not existing_pythonpath
            else f"{package_parent}:{existing_pythonpath}"
        )

        process = subprocess.Popen(
            command,
            stdout=subprocess.PIPE,
            stderr=subprocess.STDOUT,
            text=True,
            env=environment,
        )
        last_commit = time.time()
        assert process.stdout is not None
        with log_path.open("a") as log_file:
            for line in process.stdout:
                print(line, end="", flush=True)
                log_file.write(line)
                if time.time() - last_commit >= 10:
                    log_file.flush()
                    commit()
                    last_commit = time.time()
        return_code = process.wait()
        commit()
        log(f"Harbor exited with status {return_code}")

        job_dir = jobs_parent / run_id
        archive_path = run_dir / "harbor-job.tar.gz"
        if job_dir.exists():
            with tarfile.open(archive_path, "w:gz") as archive:
                archive.add(job_dir, arcname=job_dir.name)
            # Count trial dirs by the presence of a trial result.json rather than
            # a name prefix: Harbor names them `task-<id>__<suffix>` while Pier
            # uses `<task-id>__<suffix>`.
            trial_count = sum(
                1
                for path in job_dir.iterdir()
                if path.is_dir() and (path / "result.json").exists()
            )
            job_result = load_json(job_dir / "result.json")
        else:
            trial_count = 0
            job_result = None
            log(f"Harbor job dir was not found at {job_dir}")

        summary = {
            "run_id": run_id,
            "namespace": namespace,
            "experiment_name": experiment_name,
            "status": "completed" if return_code == 0 else "failed",
            "harbor_return_code": return_code,
            "trial_count": trial_count,
            "has_archive": archive_path.exists(),
            "job_result": job_result,
            "started_at": started_at,
            "completed_at": utc_now(),
        }
        write_json(run_dir / "summary.json", summary)

        if return_code == 0:
            (run_dir / "READY").write_text(utc_now() + "\n")
            state("completed", summary=summary)
        else:
            (run_dir / "FAILED").write_text(utc_now() + "\n")
            state("failed", summary=summary)
        commit()
        return summary
    except Exception as error:
        formatted = traceback.format_exc()
        log(f"FAILED: {error}\n{formatted}")
        (run_dir / "FAILED").write_text(utc_now() + "\n")
        state("failed", error=str(error), traceback=formatted)
        commit()
        raise


def rejudge_run_metadata(rejudge_request: dict[str, Any]) -> dict[str, Any]:
    judge = config.get_judge(rejudge_request["judge_preset"])
    parent = rejudge_request["parent"]
    return {
        "kind": "rejudge",
        "run_id": rejudge_request["run_id"],
        "namespace": rejudge_request["namespace"],
        "experiment_name": rejudge_request["experiment_name"],
        "source_run": parent,
        "judge_preset": rejudge_request["judge_preset"],
        "judge_model": rejudge_request.get("judge_model") or judge.model,
        "judge_upstream": judge.upstream,
        "judge_auth_env": judge.auth_env,
        "orchestration": config.orchestration_info(),
        "volume_name": rejudge_request["volume_name"],
        "api_secret_name": rejudge_request.get("api_secret_name"),
        "created_at": rejudge_request.get("created_at"),
    }


@app.function(
    image=controller_image,
    cpu=2,
    memory=4096,
    timeout=86_400,
    volumes={"/data": volume},
    secrets=[
        modal.Secret.from_name(MODAL_TOKEN_SECRET_NAME),
        modal.Secret.from_name(LLM_PROVIDERS_SECRET_NAME),
    ],
)
def rejudge_controller(rejudge_request: dict[str, Any]) -> dict[str, Any]:
    """Re-grade a finished parent run with a different judge, producing a new
    derived run. The parent is read only: its stored agent outputs are re-scored
    by the real cached verifier through the judge proxy, and only the verdicts
    change. See `rejudge.py` for the per-trial grading."""
    namespace = rejudge_request["namespace"]
    experiment_name = rejudge_request["experiment_name"]
    run_id = rejudge_request["run_id"]
    parent = rejudge_request["parent"]
    judge_preset = rejudge_request["judge_preset"]
    judge = config.get_judge(judge_preset)
    judge_model = rejudge_request.get("judge_model") or judge.model

    run_dir = pathlib.Path("/data/runs") / namespace / experiment_name / run_id
    run_dir.mkdir(parents=True, exist_ok=True)
    log_path = run_dir / "controller.log"
    state_path = run_dir / "state.json"
    started_at = utc_now()

    def log(message: str) -> None:
        line = f"[{utc_now()}] {message}"
        print(line, flush=True)
        with log_path.open("a") as log_file:
            log_file.write(line + "\n")

    def state(status: str, **extra: Any) -> None:
        write_json(
            state_path,
            {
                "run_id": run_id,
                "namespace": namespace,
                "experiment_name": experiment_name,
                "kind": "rejudge",
                "status": status,
                "updated_at": utc_now(),
                "started_at": started_at,
                **extra,
            },
        )
        volume.commit()

    try:
        write_json(run_dir / "request.json", rejudge_request)
        write_json(run_dir / "run-metadata.json", rejudge_run_metadata(rejudge_request))
        state("starting", source_run=parent)
        log(
            f"Rejudging {parent['namespace']}/{parent['experiment_name']}/"
            f"{parent['run_id']} as {namespace}/{experiment_name}/{run_id} "
            f"with judge {judge_preset} ({judge_model})"
        )

        reload_volume()
        parent_dir = (
            pathlib.Path("/data/runs")
            / parent["namespace"]
            / parent["experiment_name"]
            / parent["run_id"]
        )
        parent_archive = parent_dir / "harbor-job.tar.gz"
        if not parent_archive.exists():
            raise FileNotFoundError(
                f"parent run has no harbor-job.tar.gz: {parent_archive}. "
                "Rejudge needs a completed parent run with a stored job archive."
            )

        metadata = load_json(parent_dir / "run-metadata.json") or {}
        benchmark = metadata.get("benchmark") or {}
        dataset = benchmark.get("dataset") if isinstance(benchmark, dict) else {}
        dataset_kind = dataset.get("kind") if isinstance(dataset, dict) else None
        dataset_name = dataset.get("name") if isinstance(dataset, dict) else None
        if dataset_kind != "registry" or not isinstance(dataset_name, str):
            raise NotImplementedError(
                "rejudge currently supports registry datasets (SWE-Atlas rf/qna). "
                f"Parent run dataset_kind={dataset_kind!r} is not yet supported."
            )

        work = pathlib.Path("/tmp/agent-evals/rejudge") / run_id
        if work.exists():
            shutil.rmtree(work)
        extract_root = work / "parent"
        extract_root.mkdir(parents=True, exist_ok=True)
        log(f"Extracting parent archive {parent_archive}")
        with tarfile.open(parent_archive, "r:gz") as archive:
            safe_extract_archive(archive, extract_root)
        parent_job_dir = extract_root / parent["run_id"]
        if not parent_job_dir.is_dir():
            # Fall back to the single top-level dir if the arcname differs.
            candidates = [p for p in extract_root.iterdir() if p.is_dir()]
            if len(candidates) != 1:
                raise FileNotFoundError(
                    f"could not locate job dir inside {parent_archive}"
                )
            parent_job_dir = candidates[0]

        log(f"Downloading task packages for dataset {dataset_name}")
        subprocess.run(["harbor", "datasets", "download", dataset_name], check=True)
        tasks_root = pathlib.Path.home() / ".cache" / "harbor" / "tasks" / "packages"

        state("running", source_run=parent, judge_model=judge_model)
        out_job_dir = work / "job" / run_id
        summary = rejudge.rejudge_job(
            parent_job_dir=parent_job_dir,
            out_job_dir=out_job_dir,
            tasks_root=tasks_root,
            judge=judge,
            judge_model=judge_model,
            log=log,
        )
        volume.commit()

        archive_path = run_dir / "harbor-job.tar.gz"
        with tarfile.open(archive_path, "w:gz") as archive:
            archive.add(out_job_dir, arcname=out_job_dir.name)

        summary = {
            **summary,
            "run_id": run_id,
            "namespace": namespace,
            "experiment_name": experiment_name,
            "kind": "rejudge",
            "source_run": parent,
            "judge_preset": judge_preset,
            "status": "completed",
            "has_archive": archive_path.exists(),
            "started_at": started_at,
            "completed_at": utc_now(),
        }
        write_json(run_dir / "summary.json", summary)
        (run_dir / "READY").write_text(utc_now() + "\n")
        log(
            f"Rejudge complete: {summary['passed_count']}/"
            f"{summary['rejudged_count']} passed "
            f"({summary['failed_count']} trials could not be rejudged)"
        )
        state("completed", summary=summary)
        volume.commit()
        return summary
    except Exception as error:
        formatted = traceback.format_exc()
        log(f"FAILED: {error}\n{formatted}")
        (run_dir / "FAILED").write_text(utc_now() + "\n")
        state("failed", error=str(error), traceback=formatted)
        volume.commit()
        raise
