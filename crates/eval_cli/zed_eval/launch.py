from __future__ import annotations

import argparse
import json
import sys
import uuid
from pathlib import Path
from typing import Any

from . import benchmarks, config, harness_command, run_index, source
from .builds import prepare_build_request, resolve_source, validate_build_id
from .common import (
    dedupe_preserving_order,
    default_namespace,
    deployed_function,
    modal_call_id,
    parse_parts,
    print_json,
    utc_now,
    utc_timestamp,
)
from .volume import build_ready_on_volume

# Local scratch dir used only for dry-run/plan previews of harness commands.
PREVIEW_JOBS_DIR = "/tmp/agent-evals/harbor-jobs"


def read_task_file(path: str | None) -> list[str]:
    if not path:
        return []
    return [
        task_name
        for line in Path(path).read_text().splitlines()
        if (task_name := line.strip()) and not task_name.startswith("#")
    ]


def baseten_provider_json(
    *,
    model_id: str,
    api_url: str,
    display_name: str | None,
    max_tokens: int,
    max_output_tokens: int,
) -> str:
    provider = {
        config.BASETEN_PROVIDER_ID: {
            "api_url": api_url,
            "available_models": [
                {
                    "name": model_id,
                    "display_name": display_name or model_id,
                    "max_tokens": max_tokens,
                    "max_output_tokens": max_output_tokens,
                    "capabilities": {
                        "tools": True,
                        "images": False,
                        "parallel_tool_calls": False,
                        "prompt_cache_key": False,
                    },
                }
            ],
        }
    }
    return json.dumps(provider, separators=(",", ":"))


def resolve_model_preset(model: str) -> str:
    resolved = config.resolve_model_preset(model)
    if resolved != model:
        return resolved
    if model.startswith("baseten:"):
        return f"{config.BASETEN_PROVIDER_ID}/{model.split(':', 1)[1]}"
    return model


def resolve_model_options(
    args: argparse.Namespace,
) -> tuple[str, str | None, list[str]]:
    raw_model = getattr(args, "model", None) or config.DEFAULT_MODEL
    model = resolve_model_preset(raw_model)
    openai_compatible_provider_json = getattr(
        args, "openai_compatible_provider_json", None
    )
    extra_api_secrets = list(getattr(args, "extra_api_secret", None) or [])

    if getattr(args, "model_provider", None) == "baseten":
        baseten_model = getattr(args, "baseten_model", None) or model
        if baseten_model.startswith(f"{config.BASETEN_PROVIDER_ID}/"):
            baseten_model = baseten_model.split("/", 1)[1]
        if baseten_model.startswith("baseten:"):
            baseten_model = baseten_model.split(":", 1)[1]
        model = f"{config.BASETEN_PROVIDER_ID}/{baseten_model}"

    if model.startswith(f"{config.BASETEN_PROVIDER_ID}/"):
        baseten_model = model.split("/", 1)[1]
        if not openai_compatible_provider_json:
            openai_compatible_provider_json = baseten_provider_json(
                model_id=baseten_model,
                api_url=getattr(args, "baseten_api_url", config.BASETEN_API_URL),
                display_name=getattr(args, "baseten_model_display_name", None),
                max_tokens=getattr(
                    args, "baseten_model_max_tokens", config.BASETEN_DEFAULT_MAX_TOKENS
                ),
                max_output_tokens=getattr(
                    args,
                    "baseten_model_max_output_tokens",
                    config.BASETEN_DEFAULT_MAX_OUTPUT_TOKENS,
                ),
            )

    return model, openai_compatible_provider_json, extra_api_secrets


def derive_run_id(
    base_run_id: str | None, suite_id: str | None, suffix: str, index: int
) -> str:
    """Pick a run id for one leg of a (possibly multi-target) launch.

    An explicit --run-id is used verbatim for the first leg and suffixed for the
    rest; otherwise legs hang off the suite id, falling back to a timestamped id
    for a lone run.
    """
    if base_run_id:
        return base_run_id if index == 0 else f"{base_run_id}-{suffix}"
    if suite_id:
        return f"{suite_id}-{suffix}-{uuid.uuid4().hex[:6]}"
    return f"{utc_timestamp()}-{uuid.uuid4().hex[:6]}"


def mint_suite_id(args: argparse.Namespace, run_count: int) -> str | None:
    """A suite id groups multiple runs from one invocation; None for a lone run."""
    if run_count <= 1:
        return None
    explicit = getattr(args, "suite_id", None)
    if explicit:
        return explicit
    prefix = getattr(args, "run_id", None) or "run"
    return f"{source.sanitize_namespace(prefix)}-{utc_timestamp()}"


def build_rejudge_request(args: argparse.Namespace) -> dict[str, Any]:
    """Build the request for re-grading an existing run with a different judge.

    The positional `run_id` is the *parent* run; the derived run gets a new id
    under the same experiment so `report`/`list` group them together. Only the
    judge differs — no build, no agent model, no task selection.
    """
    judge_preset = args.judge
    config.get_judge(judge_preset)  # validate before spawning anything remote
    if getattr(args, "experiment_name", None):
        experiment_name = source.sanitize_namespace(args.experiment_name)
        namespace = default_namespace(args)
    else:
        entry = run_index.lookup(args.run_id)
        if not entry:
            raise ValueError(
                f"could not locate run '{args.run_id}' in the local run index "
                f"({run_index.index_path()}). Pass --experiment-name (and "
                "--namespace if it isn't yours)."
            )
        experiment_name = source.sanitize_namespace(entry["experiment_name"])
        namespace = source.sanitize_namespace(
            getattr(args, "namespace", None) or entry["namespace"]
        )
    parent_namespace = (
        source.sanitize_namespace(args.parent_namespace)
        if getattr(args, "parent_namespace", None)
        else namespace
    )
    parent_run_id = args.run_id
    judge_slug = source.sanitize_namespace(judge_preset)
    new_run_id = (
        args.new_run_id
        or f"{parent_run_id}-rejudge-{judge_slug}-{uuid.uuid4().hex[:6]}"
    )
    return {
        "namespace": namespace,
        "experiment_name": experiment_name,
        "run_id": new_run_id,
        "parent": {
            "namespace": parent_namespace,
            "experiment_name": experiment_name,
            "run_id": parent_run_id,
        },
        "judge_preset": judge_preset,
        "judge_model": getattr(args, "judge_model", None),
        "volume_name": args.volume,
        "api_secret_name": args.api_secret,
        "created_at": utc_now(),
    }


def command_rejudge(args: argparse.Namespace) -> int:
    rejudge_request = build_rejudge_request(args)
    if getattr(args, "dry_run", False) or getattr(args, "plan", False):
        print_json(rejudge_request)
        return 0

    controller = deployed_function(args, "rejudge_controller")
    call = controller.spawn(rejudge_request)
    run_index.record_run(
        {**rejudge_request, "volume_name": args.volume, "kind": "rejudge"}
    )
    parent = rejudge_request["parent"]
    print(f"Namespace:  {rejudge_request['namespace']}")
    print(f"Experiment: {rejudge_request['experiment_name']}")
    print(
        f"Source run: {parent['namespace']}/{parent['experiment_name']}/"
        f"{parent['run_id']}"
    )
    print(f"New run id: {rejudge_request['run_id']}")
    print(f"Judge:      {rejudge_request['judge_preset']}")
    print(f"Spawned rejudge controller: {modal_call_id(call)}")
    print("\nNext steps (run id alone is enough):")
    print(f"  zed-eval status {rejudge_request['run_id']}")
    print(f"  zed-eval report {rejudge_request['run_id']} --fetch")
    return 0


def common_run_request_fields(
    args: argparse.Namespace,
    *,
    namespace: str,
    run_id: str,
    experiment_name: str,
    judge_preset: str | None,
    build_id: str | None,
    suite_id: str | None,
) -> dict[str, Any]:
    """Fields shared by every benchmark run request."""
    agent_model, openai_compatible_provider_json, extra_api_secrets = (
        resolve_model_options(args)
    )
    task_names = dedupe_preserving_order(
        read_task_file(getattr(args, "tasks", None))
        + (getattr(args, "include_task_name", None) or [])
    )
    return {
        "created_at": utc_now(),
        "namespace": namespace,
        "run_id": run_id,
        "experiment_name": experiment_name,
        "volume_name": args.volume,
        "api_secret_name": args.api_secret,
        "modal_token_secret_name": args.modal_token_secret,
        "orchestration": config.orchestration_info(),
        "agent_model": agent_model,
        "judge_preset": judge_preset,
        "judge_model": getattr(args, "judge_model", None),
        "build_id": build_id,
        "task_names": task_names,
        "n_tasks": getattr(args, "n_tasks", None),
        "n_concurrent": getattr(args, "n_concurrent", config.DEFAULT_N_CONCURRENT),
        "override_cpus": getattr(args, "override_cpus", config.DEFAULT_OVERRIDE_CPUS),
        "override_memory_mb": getattr(
            args, "override_memory_mb", config.DEFAULT_OVERRIDE_MEMORY_MB
        ),
        "sandbox_timeout_secs": getattr(
            args, "sandbox_timeout_secs", config.DEFAULT_SANDBOX_TIMEOUT_SECS
        ),
        "sandbox_idle_timeout_secs": getattr(
            args, "sandbox_idle_timeout_secs", config.DEFAULT_SANDBOX_IDLE_TIMEOUT_SECS
        ),
        "build_wait_timeout_secs": getattr(args, "build_wait_timeout_secs", 7200),
        "extra_harbor_args": list(getattr(args, "extra_harbor_arg", None) or []),
        "openai_compatible_provider_json": openai_compatible_provider_json,
        "anthropic_available_models_json": getattr(
            args, "anthropic_available_models_json", None
        ),
        "extra_api_secrets": dedupe_preserving_order(extra_api_secrets),
        "suite_id": suite_id,
    }


def benchmark_plan_entry(
    benchmark_id: str, run_request: dict[str, Any], build_request: dict[str, Any] | None
) -> dict[str, Any]:
    return {
        "benchmark": benchmark_id,
        "run_id": run_request["run_id"],
        "harness": run_request["benchmark"]["harness"],
        "model": run_request["agent_model"],
        "judge": run_request.get("judge_preset"),
        "build_id": run_request.get("build_id"),
        "will_build": build_request is not None,
        "n_tasks": run_request.get("n_tasks"),
    }


def print_plan(
    summaries: list[dict[str, Any]],
    details: list[tuple[str, dict[str, Any], dict[str, Any] | None]],
    *,
    verbose: bool,
) -> None:
    print("Plan:")
    print_json(summaries)
    if verbose:
        for header, run_request, build_request in details:
            print(f"\n=== {header} ===")
            print_dry_run(run_request, build_request)


def print_dry_run(
    run_request: dict[str, Any], build_request: dict[str, Any] | None
) -> None:
    print("Run request:")
    print_json(run_request)
    if build_request:
        print("\nBuild request:")
        build_preview = dict(build_request)
        patch = build_preview.pop("patch", "")
        build_preview["patch_line_count"] = len(patch.splitlines())
        build_preview["source"] = source.public_source_info(
            build_request.get("source") or {}
        )
        print_json(build_preview)
    command = config.redacted_command(
        harness_command.build_harness_command(run_request, PREVIEW_JOBS_DIR)
    )
    print("\nHarness command:")
    print(command)


def execute_prepared_runs(
    args: argparse.Namespace,
    prepared: list[tuple[str, dict[str, Any], dict[str, Any] | None]],
    plan_entry,
) -> int:
    if args.plan:
        print_plan(
            [
                plan_entry(label, run_request, build_request)
                for label, run_request, build_request in prepared
            ],
            prepared,
            verbose=args.verbose,
        )
        return 0
    if args.dry_run:
        for label, run_request, build_request in prepared:
            print(f"\n=== {label} ===")
            print_dry_run(run_request, build_request)
        return 0

    spawned_builds: set[str] = set()
    for label, run_request, build_request in prepared:
        print(f"\n=== Launching {label} ===")
        launch_prepared_run(args, run_request, build_request, spawned_builds)
    return 0


def launch_prepared_run(
    args: argparse.Namespace,
    run_request: dict[str, Any],
    build_request: dict[str, Any] | None,
    spawned_builds: set[str] | None = None,
) -> None:
    build_function = None
    if build_request:
        run_request["source"] = source.public_source_info(build_request["source"])
        print_untracked_warning(build_request)
        build_function = deployed_function(args, "build_eval_cli")
    record_function = deployed_function(args, "create_run_record")
    controller_function = deployed_function(args, "run_controller")

    record_state = record_function.remote(run_request)
    run_index.record_run(run_request)

    print(f"Namespace:  {run_request['namespace']}")
    print(f"Experiment: {run_request['experiment_name']}")
    print(f"Run id:     {run_request['run_id']}")
    print(f"Volume:     {run_request['volume_name']}")
    print(f"Run state:  {record_state['status']}")
    print(f"Model:      {run_request['agent_model']}")
    if run_request.get("judge_preset"):
        print(f"Judge:      {run_request['judge_preset']}")
    if run_request.get("suite_id"):
        print(f"Suite:      {run_request['suite_id']}")
    if run_request.get("build_id"):
        print(f"Build id:  {run_request['build_id']}")
    if run_request.get("task_names"):
        print(f"Tasks:     {len(run_request['task_names'])} explicit task(s)")
    elif run_request.get("n_tasks"):
        print(f"Tasks:     Harbor --n-tasks {run_request['n_tasks']}")
    else:
        print("Tasks:     full dataset selection")

    build_id = run_request.get("build_id")
    if build_function is not None and build_id not in (spawned_builds or set()):
        build_call = build_function.spawn(build_request)
        print(f"Spawned build:      {modal_call_id(build_call)}")
        if spawned_builds is not None and build_id:
            spawned_builds.add(build_id)
    controller_call = controller_function.spawn(run_request)
    print(f"Spawned controller: {modal_call_id(controller_call)}")

    run_id = run_request["run_id"]
    print(
        "\nNext steps (run id alone is enough; namespace/experiment are resolved "
        "from this machine's local run index):"
    )
    print(f"  zed-eval status {run_id}")
    print(f"  zed-eval logs {run_id}")
    print(f"  zed-eval report {run_id} --fetch")


def resolve_benchmark_judge(
    args: argparse.Namespace, benchmark: benchmarks.Benchmark
) -> str | None:
    if not benchmark.needs_judge:
        return None
    judge = getattr(args, "judge", None) or config.DEFAULT_JUDGE_PRESET
    if judge == "auto":
        return benchmark.default_judge or "leaderboard"
    config.get_judge(judge)
    return judge


def benchmark_metadata_for_run(
    args: argparse.Namespace, benchmark: benchmarks.Benchmark
) -> dict[str, object]:
    metadata = benchmarks.benchmark_metadata(benchmark)
    dataset = metadata.get("dataset")
    if (
        isinstance(dataset, dict)
        and dataset.get("repo_url") == benchmarks.SWE_ATLAS_REPO_URL
    ):
        dataset["repo_url"] = (
            getattr(args, "swe_atlas_repo_url", None) or benchmarks.SWE_ATLAS_REPO_URL
        )
        dataset["repo_ref"] = (
            getattr(args, "swe_atlas_repo_ref", None) or benchmarks.SWE_ATLAS_REPO_REF
        )
    return metadata


def print_untracked_warning(build_request: dict[str, Any]) -> None:
    build_source = build_request.get("source") or {}
    untracked_files = build_source.get("untracked_files") or []
    if untracked_files:
        print(
            f"Warning: proceeding with {len(untracked_files)} untracked file(s) "
            "not included in the build patch.",
            file=sys.stderr,
        )


def prepare_shared_build(
    args: argparse.Namespace,
) -> tuple[str | None, dict[str, Any] | None]:
    """Resolve the build once for a whole (possibly multi-benchmark) run.

    Returns `(build_id, build_request)`. `build_request` is None when the target
    build already exists; otherwise the caller should spawn the build.
    """
    explicit_build_id = getattr(args, "build", None)
    validate_build_id(explicit_build_id)
    if (
        explicit_build_id
        and not getattr(args, "plan", False)
        and not getattr(args, "dry_run", False)
        and build_ready_on_volume(args, explicit_build_id)
    ):
        print(f"Reusing existing build {explicit_build_id} (already on volume)")
        return explicit_build_id, None

    base_sha, clean_source, source_label, pre_resolved = resolve_source(args)

    build_request = prepare_build_request(
        base_sha=base_sha,
        patch_path=getattr(args, "patch_path", None),
        build_id=explicit_build_id,
        allow_untracked=getattr(args, "allow_untracked", False),
        require_clean=getattr(args, "require_clean", False),
        repo_url=getattr(args, "repo_url", None),
        clean_source=clean_source,
        source_label=source_label,
        pre_resolved_base_sha=pre_resolved,
    )
    build_id = build_request["build_id"]

    if not getattr(args, "plan", False) and not getattr(args, "dry_run", False):
        if build_ready_on_volume(args, build_id):
            print(f"Reusing existing build {build_id} (already on volume)")
            return build_id, None

    return build_id, build_request


def command_build(args: argparse.Namespace) -> int:
    build_id, build_request = prepare_shared_build(args)
    if build_request is None:
        return 0

    print(f"Build id: {build_id}")
    print(f"Base sha: {build_request['base_sha']}")
    print(f"Patch sha256: {build_request['patch_sha256'] or '(none)'}")
    print_untracked_warning(build_request)

    build_function = deployed_function(args, "build_eval_cli")
    if args.detach:
        call = build_function.spawn(build_request)
        print(f"Spawned build: {modal_call_id(call)}")
    else:
        result = build_function.remote(build_request)
        print_json(result)
    return 0


def build_benchmark_run_request(
    args: argparse.Namespace,
    *,
    benchmark_id: str,
    build_id: str | None,
    suite_id: str | None,
    index: int,
    run_id_suffix: str | None = None,
) -> dict[str, Any]:
    benchmark = benchmarks.get_benchmark(benchmark_id)
    run_id = derive_run_id(
        getattr(args, "run_id", None), suite_id, run_id_suffix or benchmark_id, index
    )
    # Staff mode is off by default for remote runs: it enables the sandboxed
    # terminal, which hangs inside Modal sandboxes.
    extra_env = {"EVAL_CLI_STAFF": "true" if getattr(args, "staff", False) else "false"}

    return {
        **common_run_request_fields(
            args,
            namespace=default_namespace(args),
            run_id=run_id,
            # The benchmark id doubles as the experiment name for run storage
            # paths (runs/<namespace>/<experiment_name>/<run_id>), keeping
            # monitoring and fetch uniform across benchmarks.
            experiment_name=source.sanitize_namespace(benchmark_id),
            judge_preset=resolve_benchmark_judge(args, benchmark),
            build_id=build_id,
            suite_id=suite_id,
        ),
        "benchmark": benchmark_metadata_for_run(args, benchmark),
        "eval_cli_timeout": getattr(args, "eval_cli_timeout", None),
        "extra_env": extra_env,
    }


def prepare_runs_for_benchmarks(
    args: argparse.Namespace,
    benchmark_ids: list[str],
    *,
    suite_id: str | None,
    label_for_benchmark,
    mark_swe_atlas_parts: bool = False,
) -> list[tuple[str, dict[str, Any], dict[str, Any] | None]]:
    if not benchmark_ids:
        raise ValueError("choose at least one benchmark to run")

    build_id, build_request = prepare_shared_build(args)
    prepared: list[tuple[str, dict[str, Any], dict[str, Any] | None]] = []
    for index, benchmark_id in enumerate(benchmark_ids):
        label = label_for_benchmark(benchmark_id)
        run_request = build_benchmark_run_request(
            args,
            benchmark_id=benchmark_id,
            build_id=build_id,
            suite_id=suite_id,
            index=index,
            run_id_suffix=label,
        )
        if (
            mark_swe_atlas_parts
            and benchmark_id in benchmarks.SWE_ATLAS_PART_BENCHMARKS.values()
        ):
            run_request["suite_part"] = label
        # Only the first run carries the build_request; launch_prepared_run
        # dedups the actual spawn via spawned_builds anyway.
        prepared.append((label, run_request, build_request if index == 0 else None))
    return prepared


def prepare_benchmark_runs(
    args: argparse.Namespace,
) -> list[tuple[str, dict[str, Any], dict[str, Any] | None]]:
    benchmark_ids = benchmarks.resolve_benchmarks(
        list(getattr(args, "benchmark", None) or [])
    )
    return prepare_runs_for_benchmarks(
        args,
        benchmark_ids,
        suite_id=mint_suite_id(args, len(benchmark_ids)),
        label_for_benchmark=lambda benchmark_id: benchmark_id,
    )


def command_run(args: argparse.Namespace) -> int:
    return execute_prepared_runs(
        args,
        prepare_benchmark_runs(args),
        benchmark_plan_entry,
    )


def resolve_suite_parts(args: argparse.Namespace) -> list[str]:
    parts = parse_parts([args.parts] if getattr(args, "parts", None) else [])
    if parts:
        return parts
    raise ValueError(
        "choose at least one SWE-Atlas part with --parts (e.g. --parts rf,qna or --parts all)"
    )


def suite_plan_entry(
    part: str, run_request: dict[str, Any], build_request: dict[str, Any] | None
) -> dict[str, Any]:
    return {
        "part": part,
        **benchmark_plan_entry(
            run_request["benchmark"]["id"], run_request, build_request
        ),
    }


def suite_entry_label(benchmark_id: str) -> str:
    for part, part_benchmark_id in benchmarks.SWE_ATLAS_PART_BENCHMARKS.items():
        if part_benchmark_id == benchmark_id:
            return part
    return benchmark_id


def prepare_benchmark_suite(
    args: argparse.Namespace, selectors: list[str]
) -> list[tuple[str, dict[str, Any], dict[str, Any] | None]]:
    benchmark_ids = benchmarks.resolve_benchmarks(selectors)
    if not benchmark_ids:
        raise ValueError("choose at least one benchmark to run")
    if getattr(args, "zed_version", None):
        args.clean_source = True
        args.require_clean = True
    timestamp = utc_timestamp()
    prefix_seed = args.run_id_prefix or args.experiment_prefix or "swe-atlas"
    if getattr(args, "zed_version", None):
        prefix_seed = f"{prefix_seed}-{source.sanitize_namespace(args.zed_version)}"
    suite_id = args.suite_id or f"{source.sanitize_namespace(prefix_seed)}-{timestamp}"

    return prepare_runs_for_benchmarks(
        args,
        benchmark_ids,
        suite_id=suite_id,
        label_for_benchmark=suite_entry_label,
        mark_swe_atlas_parts=True,
    )


def prepare_suite(
    args: argparse.Namespace,
) -> list[tuple[str, dict[str, Any], dict[str, Any] | None]]:
    return prepare_benchmark_suite(args, resolve_suite_parts(args))


def command_swe_atlas(args: argparse.Namespace) -> int:
    from .interactive import configure_interactive_suite

    configure_interactive_suite(args)
    prepared = (
        prepare_benchmark_suite(args, args.benchmark)
        if getattr(args, "benchmark", None)
        else prepare_suite(args)
    )
    return execute_prepared_runs(args, prepared, suite_plan_entry)
