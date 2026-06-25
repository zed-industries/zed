from __future__ import annotations

import argparse
import json
import os
import subprocess
import sys
from pathlib import Path

from . import benchmarks, config, source
from .baseline import (
    command_baseline_list,
    command_baseline_record,
    command_baseline_show,
)
from .common import (
    AppNotDeployedError,
    command_exists,
    default_namespace,
    deploy_app,
    deployed_function,
    print_json,
)
from .launch import command_build, command_rejudge, command_run, command_swe_atlas
from .report import command_report
from .volume import (
    command_builds,
    command_fetch,
    command_list,
    command_logs,
    command_runs,
    command_status,
    command_suite_fetch,
    command_suite_logs,
    command_suite_status,
    ensure_volume_exists,
)


def modal_secret_names() -> set[str]:
    result = subprocess.run(
        ["modal", "secret", "list", "--json"],
        check=True,
        capture_output=True,
        text=True,
    )
    data = json.loads(result.stdout)
    if not isinstance(data, list):
        return set()
    return {
        item["name"] for item in data if isinstance(item, dict) and item.get("name")
    }


def command_doctor(args: argparse.Namespace) -> int:
    namespace = default_namespace(args)
    print("zed-eval doctor")
    print(f"  namespace:          {namespace}")
    print(f"  app:                {args.app_name}")
    print(f"  volume:             {args.volume}")
    print(f"  api secret:         {args.api_secret}")
    print(f"  modal token secret: {args.modal_token_secret}")
    print(f"  repo root:          {source.repo_root()}")
    print(f"  base sha:           {source.current_base_sha()}")
    print(f"  default model:      {config.DEFAULT_MODEL}")
    print("  default judges:     qna=deepseek-v4-pro, rf/tw=kimi-k2.7-code")

    missing = []
    for executable in ("git", "modal", "harbor"):
        if command_exists(executable):
            print(f"  {executable}:             found")
        else:
            print(f"  {executable}:             missing")
            missing.append(executable)

    if missing:
        print("\nMissing required executables: " + ", ".join(missing), file=sys.stderr)
        return 1

    try:
        secrets = modal_secret_names()
    except (subprocess.CalledProcessError, json.JSONDecodeError) as error:
        print(
            f"  secrets:          could not list Modal secrets: {error}",
            file=sys.stderr,
        )
    else:
        missing_secrets = [
            secret_name
            for secret_name in (args.api_secret, args.modal_token_secret)
            if secret_name not in secrets
        ]
        if missing_secrets:
            print(
                "\nMissing Modal secret(s): " + ", ".join(missing_secrets),
                file=sys.stderr,
            )
            print(
                "Use --api-secret/--modal-token-secret to point at existing secrets, "
                "or create the missing ones with `modal secret create`.",
                file=sys.stderr,
            )
            return 1
        print("  secrets:          found")
        print(
            "  controller token: use a dedicated Modal service-user token for production"
        )

    if args.create_volume:
        return ensure_volume_exists(args)
    return 0


def command_deploy(args: argparse.Namespace) -> int:
    deploy_app(args)
    print(f"Deployed Modal app '{args.app_name}'.")
    return 0


def command_cleanup(args: argparse.Namespace) -> int:
    request: dict[str, object] = {"dry_run": bool(args.dry_run)}
    if args.build_retention_days is not None:
        request["build_retention_days"] = args.build_retention_days
    result = deployed_function(args, "cleanup_artifacts").remote(request)
    print_json(result)
    return 0


def env_default(
    variable_name: str, default: object, *, include_defaults: bool = True
) -> object:
    if not include_defaults:
        return argparse.SUPPRESS
    return os.environ.get(variable_name, default)


def add_common_options(
    parser: argparse.ArgumentParser, *, include_defaults: bool = True
) -> None:
    default_volume = env_default(
        "AGENT_EVALS_VOLUME",
        config.DEFAULT_VOLUME_NAME,
        include_defaults=include_defaults,
    )
    default_namespace = None if include_defaults else argparse.SUPPRESS
    default_api_secret = env_default(
        "AGENT_EVALS_LLM_PROVIDERS_SECRET",
        config.DEFAULT_LLM_PROVIDERS_SECRET_NAME,
        include_defaults=include_defaults,
    )
    default_modal_token_secret = env_default(
        "AGENT_EVALS_MODAL_TOKEN_SECRET",
        config.DEFAULT_MODAL_TOKEN_SECRET_NAME,
        include_defaults=include_defaults,
    )
    default_app_name = env_default(
        "AGENT_EVALS_APP_NAME",
        config.DEFAULT_APP_NAME,
        include_defaults=include_defaults,
    )
    infra = parser.add_argument_group(
        "infra options", "Modal app/volume/namespace and secret names (rarely changed)"
    )
    infra.add_argument(
        "--app-name",
        default=default_app_name,
        help=f"Deployed Modal app name (default: {config.DEFAULT_APP_NAME})",
    )
    infra.add_argument(
        "--volume",
        default=default_volume,
        help=f"Modal volume for builds/runs (default: {config.DEFAULT_VOLUME_NAME})",
    )
    infra.add_argument(
        "--namespace",
        default=default_namespace,
        help="Run namespace inside the shared volume (default: env/git user/local user)",
    )
    infra.add_argument(
        "--api-secret",
        default=default_api_secret,
        help=(
            "Modal secret with the LLM provider keys mounted into trial sandboxes "
            f"(default: {config.DEFAULT_LLM_PROVIDERS_SECRET_NAME})"
        ),
    )
    infra.add_argument(
        "--modal-token-secret",
        default=default_modal_token_secret,
        help=(
            "Modal secret mounted into the controller with MODAL_TOKEN_ID and MODAL_TOKEN_SECRET; "
            f"use a dedicated service-user token for production (default: {config.DEFAULT_MODAL_TOKEN_SECRET_NAME})"
        ),
    )


def add_build_source_options(parser: argparse.ArgumentParser) -> None:
    parser.add_argument(
        "--from",
        dest="from_source",
        default=os.environ.get("AGENT_EVALS_FROM"),
        help=(
            "Unified build source: 'local' for current HEAD + tracked changes, "
            "'main' for origin/main's tip, or a git ref/tag/SHA (e.g. v0.210.0) "
            "for a clean build resolved canonically against the remote so "
            "teammates share one build"
        ),
    )
    parser.add_argument("--base-sha")
    parser.add_argument("--patch-path")
    parser.add_argument(
        "--repo-url",
        default=os.environ.get("AGENT_EVALS_REPO_URL", source.DEFAULT_REPO_URL),
        help="Git remote the Modal builder fetches the base SHA from",
    )
    parser.add_argument(
        "--allow-untracked",
        action="store_true",
        help="Proceed even when untracked files exist; they are not included in the build patch",
    )
    parser.add_argument(
        "--require-clean",
        action="store_true",
        help="Fail if tracked changes are present",
    )
    parser.add_argument(
        "--clean-source",
        action="store_true",
        help="Build exactly --base-sha/--zed-version with no local patch",
    )
    parser.add_argument(
        "--zed-version",
        help="Git ref/tag/SHA of Zed to build as a clean source snapshot",
    )


def add_model_options(parser: argparse.ArgumentParser) -> None:
    parser.add_argument(
        "-m",
        "--model",
        default=os.environ.get("AGENT_EVALS_MODEL", config.DEFAULT_MODEL),
        help="Base model as provider/model, sonnet-4.6, baseten:kimi-k2.7-code, or baseten:<model-id>",
    )
    parser.add_argument(
        "--extra-api-secret",
        action="append",
        help="Additional Modal secret name to mount into the trial sandbox",
    )
    advanced = parser.add_argument_group(
        "advanced model options",
        "Lower-level model routing. For Baseten, prefer --model baseten:<model-id>.",
    )
    advanced.add_argument(
        "--model-provider",
        choices=("zed", "baseten"),
        default="zed",
        help="Use 'zed' for built-in provider/model ids, or 'baseten' for Baseten Model APIs",
    )
    advanced.add_argument(
        "--baseten-model",
        help="Baseten model id when --model-provider baseten, e.g. moonshotai/Kimi-K2.7-Code",
    )
    advanced.add_argument("--baseten-model-display-name")
    advanced.add_argument(
        "--baseten-api-url",
        default=config.BASETEN_API_URL,
        help=f"Baseten OpenAI-compatible API URL (default: {config.BASETEN_API_URL})",
    )
    advanced.add_argument(
        "--baseten-model-max-tokens",
        type=int,
        default=config.BASETEN_DEFAULT_MAX_TOKENS,
    )
    advanced.add_argument(
        "--baseten-model-max-output-tokens",
        type=int,
        default=config.BASETEN_DEFAULT_MAX_OUTPUT_TOKENS,
    )
    advanced.add_argument(
        "--openai-compatible-provider-json",
        help="JSON object merged into language_models.openai_compatible before eval-cli resolves --model",
    )
    advanced.add_argument(
        "--anthropic-available-models-json",
        help="JSON array merged into language_models.anthropic.available_models before eval-cli resolves --model",
    )


def add_launch_options(parser: argparse.ArgumentParser) -> None:
    add_model_options(parser)
    parser.add_argument(
        "-j",
        "--judge",
        default=os.environ.get("AGENT_EVALS_JUDGE", config.DEFAULT_JUDGE_PRESET),
        choices=["auto", *sorted(config.JUDGES)],
        help="Judge preset; auto uses qna=deepseek-v4-pro and rf/tw=kimi-k2.7-code",
    )
    parser.add_argument("--judge-model")
    parser.add_argument(
        "--build",
        metavar="ID",
        help="Reuse this build if it exists, otherwise create it with this id",
    )
    add_build_source_options(parser)
    parser.add_argument("--build-wait-timeout-secs", type=int, default=7200)
    parser.add_argument("--tasks", help="File containing one full task name per line")
    parser.add_argument("--include-task-name", action="append")
    parser.add_argument("-n", "--n-tasks", type=int, help="Forward harness --n-tasks")
    parser.add_argument("--n-concurrent", type=int, default=config.DEFAULT_N_CONCURRENT)
    parser.add_argument(
        "--override-cpus", type=int, default=config.DEFAULT_OVERRIDE_CPUS
    )
    parser.add_argument(
        "--override-memory-mb", type=int, default=config.DEFAULT_OVERRIDE_MEMORY_MB
    )
    parser.add_argument(
        "--sandbox-timeout-secs", type=int, default=config.DEFAULT_SANDBOX_TIMEOUT_SECS
    )
    parser.add_argument(
        "--sandbox-idle-timeout-secs",
        type=int,
        default=config.DEFAULT_SANDBOX_IDLE_TIMEOUT_SECS,
    )
    parser.add_argument(
        "--eval-cli-timeout",
        type=int,
        help="Override the per-task agent timeout (defaults to the benchmark's)",
    )
    parser.add_argument("--run-id")
    parser.add_argument(
        "--dry-run",
        action="store_true",
        help="Print full manifests and harness commands without launching",
    )
    parser.add_argument(
        "--plan",
        action="store_true",
        help="Print a concise launch plan without launching",
    )
    parser.add_argument(
        "--verbose",
        action="store_true",
        help="With --plan, include full manifests and harness commands",
    )
    parser.add_argument(
        "--extra-harbor-arg",
        action="append",
        help="Append one raw argument to the harness run command",
    )
    parser.add_argument(
        "--swe-atlas-repo-url",
        default=os.environ.get(
            "AGENT_EVALS_SWE_ATLAS_REPO_URL", benchmarks.SWE_ATLAS_REPO_URL
        ),
        help="SWE-Atlas repo URL used for path-backed datasets",
    )
    parser.add_argument(
        "--swe-atlas-repo-ref",
        default=os.environ.get(
            "AGENT_EVALS_SWE_ATLAS_REPO_REF", benchmarks.SWE_ATLAS_REPO_REF
        ),
        help="SWE-Atlas repo ref used for path-backed datasets",
    )


def add_suite_options(parser: argparse.ArgumentParser) -> None:
    add_launch_options(parser)
    parser.add_argument(
        "--parts",
        help="Comma-separated SWE-Atlas parts to run: qna,rf,tw (or 'all')",
    )
    parser.add_argument("--experiment-prefix", default="swe-atlas")
    parser.add_argument("--run-id-prefix")
    parser.add_argument(
        "--suite-id", help="Explicit suite id for grouping multi-part runs"
    )
    parser.add_argument("--interactive", action="store_true")
    parser.add_argument(
        "-y",
        "--yes",
        action="store_true",
        help="Never prompt; require flags/defaults",
    )


def add_run_lookup_options(
    parser: argparse.ArgumentParser,
    *,
    require_experiment: bool = False,
    run_id_optional: bool = False,
) -> None:
    if run_id_optional:
        parser.add_argument(
            "run_id",
            nargs="?",
            help="Run id (defaults to the most recent run launched from this machine)",
        )
    else:
        parser.add_argument("run_id")
    parser.add_argument(
        "-e",
        "--experiment-name",
        required=require_experiment,
        help="Benchmark storage name (auto-resolved from the local run index when omitted)",
    )


def add_command_parser(
    subparsers,
    name: str,
    *,
    func: object | None = None,
    **kwargs,
) -> argparse.ArgumentParser:
    parser = subparsers.add_parser(name, **kwargs)
    add_common_options(parser, include_defaults=False)
    if func is not None:
        parser.set_defaults(func=func)
    return parser


def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(
        prog="zed-eval",
        description="Launch, monitor, and fetch remote SWE-Atlas agent evals on Modal.",
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog=(
            "Common workflows:\n"
            "  zed-eval run swe-atlas -m sonnet-4.6       # launch the SWE-Atlas suite\n"
            "  zed-eval run rf -m sonnet-4.6              # launch one benchmark\n"
            "  zed-eval runs                              # what did I launch recently?\n"
            "  zed-eval status                            # check my most recent run\n"
            "  zed-eval status <run-id>                   # check a specific run\n"
            "  zed-eval logs <run-id>                     # print controller logs\n"
            "  zed-eval report <run-id> --fetch           # fetch + score a run\n"
            "\n"
            "After launching, a run id alone locates the run — namespace and\n"
            "benchmark storage name are resolved from this machine's local run index."
        ),
    )
    add_common_options(parser)
    subparsers = parser.add_subparsers(dest="command", required=True)

    doctor = add_command_parser(
        subparsers,
        "doctor",
        func=command_doctor,
        help="Check local prerequisites and defaults",
    )
    doctor.add_argument("--create-volume", action="store_true")

    add_command_parser(
        subparsers,
        "deploy",
        func=command_deploy,
        help=(
            "Publish harness code to Modal. This is the ONLY command that "
            "deploys; deploying cancels in-flight runs. Run it once after "
            "changing harness code — all other commands just invoke the "
            "already-deployed functions and never deploy."
        ),
    )

    build = add_command_parser(
        subparsers,
        "build",
        func=command_build,
        help="Build eval-cli on Modal into builds/<build-id>",
    )
    add_build_source_options(build)
    build.add_argument(
        "--build",
        metavar="ID",
        help="Reuse this build if it exists, otherwise create it with this id",
    )
    build.add_argument(
        "--detach", action="store_true", help="Spawn the build and return immediately"
    )

    builds = add_command_parser(
        subparsers,
        "builds",
        func=command_builds,
        help="List content-addressed builds on the volume",
    )
    builds.add_argument("--details", action="store_true")
    builds.add_argument("--json", action="store_true")
    builds.add_argument("--limit", type=int, default=50)

    run_cmd = add_command_parser(
        subparsers,
        "run",
        func=command_run,
        help="Run benchmarks — the everyday entry point",
        description=(
            "Launch runs by benchmark id, alias, or group. It picks the right "
            "default judge for each benchmark and never prompts. The sibling "
            "`swe-atlas` command launches SWE-Atlas parts (qna/rf/tw) with "
            "interactive prompts."
        ),
    )
    run_cmd.add_argument(
        "benchmark",
        nargs="+",
        metavar="target",
        help=(
            "Benchmark id/alias/group (swe-atlas (= qna,rf,tw), swe-atlas-rf, "
            "qna, rf, tw, terminal-bench-2.1 (tb21), deepswe). Comma-separated "
            "and repeated values are combined."
        ),
    )
    add_launch_options(run_cmd)

    run_cmd.add_argument(
        "--suite-id", help="Explicit suite id for multi-benchmark runs"
    )
    run_cmd.add_argument(
        "--staff",
        action="store_true",
        help=(
            "Run the agent with staff mode ON (default OFF). Staff mode enables "
            "the sandboxed terminal, which hangs inside Modal sandboxes, so keep "
            "it off for remote runs."
        ),
    )
    run_cmd.add_argument(
        "-y",
        "--yes",
        action="store_true",
        help="Accepted for parity with swe-atlas; run never prompts",
    )

    report = add_command_parser(
        subparsers,
        "report",
        func=command_report,
        help="Success-conditioned metrics (rate, tokens, tool calls, steps)",
    )
    report.add_argument("run_id", nargs="?")
    report.add_argument(
        "-e",
        "--experiment-name",
        help="Benchmark storage name (auto-resolved from the local run index when omitted)",
    )
    report.add_argument("--job-dir", help="Analyze a local job directory directly")
    report.add_argument("--jobs-dir", default=str(Path.home() / ".cache/harbor/jobs"))
    report.add_argument(
        "--fetch", action="store_true", help="Fetch the run archive before reporting"
    )
    report.add_argument(
        "--json",
        dest="as_json",
        action="store_true",
        help="Emit JSON instead of a table",
    )
    report.add_argument(
        "--timeouts-as-failures",
        dest="timeouts_as_failures",
        action=argparse.BooleanOptionalAction,
        default=None,
        help=(
            "Count agent timeouts as failures rather than excluded errors "
            "(default: auto — on for test-scored benchmarks like terminal-bench "
            "and deepswe)"
        ),
    )

    cleanup = add_command_parser(
        subparsers,
        "cleanup",
        func=command_cleanup,
        help="Prune stale builds and cold build cache (never eval results)",
    )
    cleanup.add_argument(
        "--dry-run",
        action="store_true",
        help="Report what would be removed without deleting anything",
    )
    cleanup.add_argument(
        "--build-retention-days",
        type=float,
        help="Remove builds older than this many days (default 14)",
    )

    suite = add_command_parser(
        subparsers,
        "swe-atlas",
        func=command_swe_atlas,
        help="Launch an interactive benchmark suite (SWE-Atlas, Terminal-Bench, DeepSWE)",
    )
    add_suite_options(suite)

    list_runs = add_command_parser(
        subparsers,
        "list",
        func=command_list,
        help="List benchmark runs on the volume, optionally with metadata",
    )
    list_runs.add_argument("-e", "--experiment-name")
    list_runs.add_argument("--details", action="store_true")
    list_runs.add_argument("--json", action="store_true")
    list_runs.add_argument("--limit", type=int, default=50)
    list_runs.add_argument("--all-namespaces", action="store_true")

    runs = add_command_parser(
        subparsers,
        "runs",
        func=command_runs,
        help="List recent runs launched from this machine (local, fast)",
        description=(
            "Show runs recorded in the local run index when you launched them "
            "from this machine — newest first, no network call. Use this for a "
            "quick 'what did I launch lately?'; use `list --details` to query "
            "run state on the volume, or `status <run-id>` for one run's state."
        ),
    )
    runs.add_argument("--json", action="store_true")
    runs.add_argument("--limit", type=int, default=20)

    status = add_command_parser(
        subparsers,
        "status",
        func=command_status,
        help="Print a run's state.json once (no run id = most recent run)",
    )
    add_run_lookup_options(status, run_id_optional=True)

    logs = add_command_parser(
        subparsers,
        "logs",
        func=command_logs,
        help="Print a run's controller.log once (no run id = most recent run)",
    )
    add_run_lookup_options(logs, run_id_optional=True)

    fetch = add_command_parser(
        subparsers,
        "fetch",
        func=command_fetch,
        help="Fetch and extract a run's Harbor job archive",
    )
    add_run_lookup_options(fetch)
    fetch.add_argument("--jobs-dir", default=str(Path.home() / ".cache/harbor/jobs"))

    rejudge = add_command_parser(
        subparsers,
        "rejudge",
        func=command_rejudge,
        help="Re-grade a finished run with a different judge (new derived run, "
        "reuses the agent work)",
    )
    add_run_lookup_options(rejudge)
    rejudge.add_argument(
        "-j",
        "--judge",
        required=True,
        help="Judge preset to re-grade with (see config.JUDGES, e.g. "
        "deepseek-v4-pro, kimi-k2.7-code, leaderboard)",
    )
    rejudge.add_argument(
        "--judge-model",
        dest="judge_model",
        help="Override the judge preset's model id",
    )
    rejudge.add_argument(
        "--parent-namespace",
        dest="parent_namespace",
        help="Namespace of the source run, if different from --namespace",
    )
    rejudge.add_argument(
        "--new-run-id",
        dest="new_run_id",
        help="Explicit id for the derived run (default: <parent>-rejudge-<judge>-<rand>)",
    )
    rejudge.add_argument(
        "--dry-run",
        action="store_true",
        help="Print the rejudge request without spawning the controller",
    )

    suite_group = add_command_parser(
        subparsers,
        "suite",
        help="Inspect or fetch grouped benchmark suite runs",
    )
    suite_subparsers = suite_group.add_subparsers(dest="suite_command", required=True)
    suite_status = add_command_parser(
        suite_subparsers,
        "status",
        func=command_suite_status,
        help="Show status for each run in a suite",
    )
    suite_status.add_argument("suite_id")
    suite_status.add_argument("--json", action="store_true")
    suite_logs = add_command_parser(
        suite_subparsers,
        "logs",
        func=command_suite_logs,
        help="Print logs for each run in a suite",
    )
    suite_logs.add_argument("suite_id")
    suite_logs.add_argument("--follow", action="store_true")
    suite_logs.add_argument("--interval", type=float, default=30.0)
    suite_fetch = add_command_parser(
        suite_subparsers,
        "fetch",
        func=command_suite_fetch,
        help="Fetch all run archives in a suite",
    )
    suite_fetch.add_argument("suite_id")
    suite_fetch.add_argument(
        "--jobs-dir", default=str(Path.home() / ".cache/harbor/jobs")
    )

    baseline_group = add_command_parser(
        subparsers,
        "baseline",
        help="Record and inspect baseline-of-record results (clean commits on main)",
    )
    baseline_subparsers = baseline_group.add_subparsers(
        dest="baseline_command", required=True
    )
    baseline_record = add_command_parser(
        baseline_subparsers,
        "record",
        func=command_baseline_record,
        help="Promote completed run(s) to the baseline of record for their (benchmark, model)",
    )
    baseline_record.add_argument("run_id", nargs="+")
    baseline_record.add_argument("--experiment-name", required=True)
    baseline_record.add_argument(
        "--allow-dirty",
        action="store_true",
        help="Record even though the build carries a local patch (not a clean commit)",
    )
    baseline_record.add_argument(
        "--allow-off-main",
        action="store_true",
        help="Record even though base_sha can't be verified as reachable from origin/main",
    )
    baseline_record.add_argument(
        "--repo-url",
        default=os.environ.get("AGENT_EVALS_REPO_URL"),
        help="Git remote to resolve origin/main against (default: AGENT_EVALS_REPO_URL or the canonical repo)",
    )

    baseline_list = add_command_parser(
        baseline_subparsers,
        "list",
        func=command_baseline_list,
        help="List the current baseline of record for every (benchmark, model)",
    )
    baseline_list.add_argument("--json", action="store_true")

    baseline_show = add_command_parser(
        baseline_subparsers,
        "show",
        func=command_baseline_show,
        help="Show the baseline record for one (benchmark, model)",
    )
    baseline_show.add_argument("experiment_name")
    baseline_show.add_argument("--model", required=True)
    baseline_show.add_argument(
        "--history", action="store_true", help="Include superseded baselines"
    )

    return parser


def main(argv: list[str] | None = None) -> int:
    parser = build_parser()
    args = parser.parse_args(argv)
    try:
        return args.func(args)
    except (ValueError, AppNotDeployedError) as error:
        print(f"error: {error}", file=sys.stderr)
        return 1
    except subprocess.CalledProcessError as error:
        if error.stdout:
            print(error.stdout, end="")
        if error.stderr:
            print(error.stderr, end="", file=sys.stderr)
        return error.returncode


if __name__ == "__main__":
    raise SystemExit(main())
