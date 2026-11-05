"""Benchmark registry.

A `Benchmark` describes everything the orchestrator needs to run one benchmark
end-to-end that is *not* about the build source or the model: which harness runs
it (Harbor or its Pier fork), how its dataset is provisioned, how a trial is
scored, whether it needs an LLM judge, and the per-task timeout.

Adding a benchmark should be a data change here, not new control flow. The three
benchmark families currently supported are all Harbor-family harnesses:

  - SWE-Atlas (qna / rf / tw): Harbor, rubric LLM judge.
  - Terminal-Bench 2.1: Harbor, test-script scoring, no judge.
  - DeepSWE: Pier (a Harbor fork with per-agent network allowlists, required
    because DeepSWE tasks run with `allow_internet = false`), test scoring.
"""

from __future__ import annotations

from dataclasses import dataclass, field

# How a benchmark's dataset is made available to the harness.
#   registry -> `-d <name>` pulled from the harness hub
#   path     -> `-p <dir>` from a git repo cloned by the controller
#   pier_path -> path dataset run under Pier instead of Harbor
DATASET_REGISTRY = "registry"
DATASET_PATH = "path"
DATASET_PIER_PATH = "pier_path"

HARNESS_HARBOR = "harbor"
HARNESS_PIER = "pier"

SCORING_RUBRIC_JUDGE = "rubric-judge"
SCORING_TESTS = "tests"


@dataclass(frozen=True)
class DatasetRef:
    kind: str
    # For registry datasets: the hub dataset name. For path datasets: the repo
    # URL plus a ref and the sub-directory the tasks live in.
    name: str | None = None
    repo_url: str | None = None
    repo_ref: str | None = None
    data_dir: str | None = None


@dataclass(frozen=True)
class Benchmark:
    id: str
    label: str
    harness: str
    dataset: DatasetRef
    default_timeout_secs: int
    scoring: str
    needs_judge: bool
    default_judge: str | None = None
    # Hosts the in-sandbox agent must reach even on air-gapped tasks. Required by
    # Pier benchmarks (DeepSWE) so eval-cli can still call the model API.
    network_allowlist: tuple[str, ...] = ()
    env: dict[str, str] = field(default_factory=dict)


SWE_ATLAS_REPO_URL = "https://github.com/scaleapi/SWE-Atlas.git"
SWE_ATLAS_REPO_REF = "main"
DEEPSWE_REPO_URL = "https://github.com/datacurve-ai/deep-swe.git"
DEEPSWE_REPO_REF = "main"

# Model/judge API hosts the agent needs even under `allow_internet = false`.
# Pier grants these to the agent via its per-agent allowlist.
AGENT_API_HOSTS: tuple[str, ...] = (
    "api.anthropic.com",
    "api.openai.com",
    "inference.baseten.co",
)


BENCHMARKS: dict[str, Benchmark] = {
    "swe-atlas-qna": Benchmark(
        id="swe-atlas-qna",
        label="SWE-Atlas Codebase Q&A",
        harness=HARNESS_HARBOR,
        dataset=DatasetRef(kind=DATASET_REGISTRY, name="scale-ai/swe-atlas-qna"),
        default_timeout_secs=7200,
        scoring=SCORING_RUBRIC_JUDGE,
        needs_judge=True,
        default_judge="deepseek-v4-pro",
    ),
    "swe-atlas-rf": Benchmark(
        id="swe-atlas-rf",
        label="SWE-Atlas Refactoring",
        harness=HARNESS_HARBOR,
        dataset=DatasetRef(kind=DATASET_REGISTRY, name="scale-ai/swe-atlas-rf"),
        default_timeout_secs=3300,
        scoring=SCORING_RUBRIC_JUDGE,
        needs_judge=True,
        default_judge="kimi-k2.7-code",
    ),
    "swe-atlas-tw": Benchmark(
        id="swe-atlas-tw",
        label="SWE-Atlas Test Writing",
        harness=HARNESS_HARBOR,
        # Test writing is not consistently published in the same registry shape as qna/rf.
        dataset=DatasetRef(
            kind=DATASET_PATH,
            repo_url=SWE_ATLAS_REPO_URL,
            repo_ref=SWE_ATLAS_REPO_REF,
            data_dir="data/tw",
        ),
        default_timeout_secs=3300,
        scoring=SCORING_RUBRIC_JUDGE,
        needs_judge=True,
        default_judge="kimi-k2.7-code",
    ),
    "terminal-bench-2.1": Benchmark(
        id="terminal-bench-2.1",
        label="Terminal-Bench 2.1",
        harness=HARNESS_HARBOR,
        dataset=DatasetRef(
            kind=DATASET_REGISTRY, name="terminal-bench/terminal-bench-2-1"
        ),
        default_timeout_secs=3300,
        scoring=SCORING_TESTS,
        needs_judge=False,
        # Air-gapped tasks: the fetch/web-search tools can't reach the network,
        # so disable them (via the agent profile in eval-cli) to stop the agent
        # wasting its budget on tools that can only fail.
        env={"ZED_EVAL_DISABLE_TOOLS": "fetch,search_web"},
    ),
    "deepswe": Benchmark(
        id="deepswe",
        label="DeepSWE",
        harness=HARNESS_PIER,
        dataset=DatasetRef(
            kind=DATASET_PIER_PATH,
            repo_url=DEEPSWE_REPO_URL,
            repo_ref=DEEPSWE_REPO_REF,
            # DeepSWE keeps its Harbor-compatible tasks at the repo root.
            data_dir="tasks",
        ),
        default_timeout_secs=7200,
        scoring=SCORING_TESTS,
        needs_judge=False,
        network_allowlist=AGENT_API_HOSTS,
        # Air-gapped except the model API allowlist; fetch/web-search are useless.
        env={"ZED_EVAL_DISABLE_TOOLS": "fetch,search_web"},
    ),
}


# Groups expand to multiple benchmarks in one launch.
BENCHMARK_GROUPS: dict[str, tuple[str, ...]] = {
    "swe-atlas": ("swe-atlas-qna", "swe-atlas-rf", "swe-atlas-tw"),
}

# One short alias per benchmark whose canonical id is verbose.
BENCHMARK_ALIASES: dict[str, str] = {
    "qna": "swe-atlas-qna",
    "rf": "swe-atlas-rf",
    "tw": "swe-atlas-tw",
    "tb21": "terminal-bench-2.1",
}

SWE_ATLAS_PART_BENCHMARKS: dict[str, str] = {
    "qna": "swe-atlas-qna",
    "rf": "swe-atlas-rf",
    "tw": "swe-atlas-tw",
}


def get_benchmark(benchmark_id: str) -> Benchmark:
    try:
        return BENCHMARKS[benchmark_id]
    except KeyError as error:
        valid = ", ".join(sorted(BENCHMARKS))
        raise ValueError(
            f"unknown benchmark '{benchmark_id}' (valid: {valid})"
        ) from error


def is_benchmark_selector(selector: str) -> bool:
    """Whether `selector` names a known benchmark id, alias, or group.

    Used by `run` to validate benchmark positionals before preparing builds."""
    normalized = selector.strip().lower()
    return (
        normalized in BENCHMARK_GROUPS
        or normalized in BENCHMARKS
        or normalized in BENCHMARK_ALIASES
    )


def resolve_benchmark_selector(selector: str) -> list[str]:
    """Expands a user-facing selector (benchmark id, alias, or group) into the
    concrete benchmark ids it refers to, preserving order."""
    normalized = selector.strip().lower()
    if normalized in BENCHMARK_GROUPS:
        return list(BENCHMARK_GROUPS[normalized])
    if normalized in BENCHMARKS:
        return [normalized]
    if normalized in BENCHMARK_ALIASES:
        return [BENCHMARK_ALIASES[normalized]]
    valid = ", ".join(sorted({*BENCHMARKS, *BENCHMARK_GROUPS, *BENCHMARK_ALIASES}))
    raise ValueError(f"unknown benchmark '{selector}' (valid: {valid})")


def resolve_benchmarks(selectors: list[str]) -> list[str]:
    resolved: list[str] = []
    for selector in selectors:
        for part in selector.split(","):
            part = part.strip()
            if not part:
                continue
            for benchmark_id in resolve_benchmark_selector(part):
                if benchmark_id not in resolved:
                    resolved.append(benchmark_id)
    return resolved


def benchmark_metadata(benchmark: Benchmark) -> dict[str, object]:
    """The self-describing block embedded in a run request so the controller and
    harness-command builder need no separate registry lookup."""
    return {
        "id": benchmark.id,
        "label": benchmark.label,
        "harness": benchmark.harness,
        "dataset": {
            "kind": benchmark.dataset.kind,
            "name": benchmark.dataset.name,
            "repo_url": benchmark.dataset.repo_url,
            "repo_ref": benchmark.dataset.repo_ref,
            "data_dir": benchmark.dataset.data_dir,
        },
        "default_timeout_secs": benchmark.default_timeout_secs,
        "scoring": benchmark.scoring,
        "needs_judge": benchmark.needs_judge,
        "default_judge": benchmark.default_judge,
        "network_allowlist": list(benchmark.network_allowlist),
        "env": dict(benchmark.env),
    }
