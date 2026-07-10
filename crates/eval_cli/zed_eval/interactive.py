from __future__ import annotations

import argparse
import sys

from .common import dedupe_preserving_order, deployed_function

INTERACTIVE_BENCHMARK_CHOICES = [
    "qna",
    "rf",
    "tw",
    "terminal-bench-2.1",
    "deepswe",
]
INTERACTIVE_BENCHMARK_ALIASES = {
    "terminal-bench": "terminal-bench-2.1",
    "tb21": "terminal-bench-2.1",
    "deep-swe": "deepswe",
}
INTERACTIVE_MODEL_CHOICES = [
    "sonnet-4.6",
    "opus-4.5",
    "baseten:kimi-k2.7-code",
    "baseten:deepseek-v4-pro",
    "custom-zed-model",
    "custom-baseten-model",
]


def prompt(label: str, default: str | None = None) -> str:
    suffix = f" [{default}]" if default else ""
    value = input(f"{label}{suffix}: ").strip()
    return value or (default or "")


def prompt_required(label: str) -> str:
    while True:
        value = prompt(label)
        if value:
            return value
        print("Enter a value", file=sys.stderr)


def prompt_choice(label: str, choices: list[str], default: str) -> str:
    print(label)
    for index, choice in enumerate(choices, start=1):
        marker = " (default)" if choice == default else ""
        print(f"  {index}. {choice}{marker}")
    while True:
        value = prompt("Choose", default)
        if value in choices:
            return value
        if value.isdigit():
            index = int(value) - 1
            if 0 <= index < len(choices):
                return choices[index]
        print(f"Enter one of: {', '.join(choices)}", file=sys.stderr)


def prompt_multi(
    label: str,
    choices: list[str],
    default: list[str],
    aliases: dict[str, str] | None = None,
) -> list[str]:
    default_text = ",".join(default)
    canonical_choices = {choice.lower(): choice for choice in choices}
    aliases = aliases or {}
    print(label)
    for choice in choices:
        print(f"  - {choice}")
    while True:
        value = prompt("Comma-separated choices, or all", default_text)
        selections = []
        for selection in (part.strip().lower() for part in value.split(",")):
            if not selection:
                continue
            if selection == "all":
                return list(choices)
            canonical = aliases.get(selection) or canonical_choices.get(selection)
            if canonical is None:
                valid = ", ".join([*choices, *sorted(aliases)])
                print(f"unknown choice '{selection}' (valid: {valid})", file=sys.stderr)
                break
            selections.append(canonical)
        else:
            selections = dedupe_preserving_order(selections)
            if selections:
                return selections
            print("Choose at least one benchmark", file=sys.stderr)


def should_prompt(args: argparse.Namespace) -> bool:
    if getattr(args, "interactive", False):
        return True
    if getattr(args, "yes", False):
        return False
    return sys.stdin.isatty() and not getattr(args, "parts", None)


def choose_existing_build(args: argparse.Namespace) -> str:
    try:
        rows = deployed_function(args, "list_builds").remote(20)
    except Exception as error:
        print(
            f"Could not list builds ({error}); falling back to manual entry.",
            file=sys.stderr,
        )
        return prompt("Existing build id")
    if not rows:
        return prompt("No builds found; enter build id")
    print("Existing builds")
    for index, row in enumerate(rows, start=1):
        build_id = row.get("build_id")
        base = str(row.get("base_sha") or "")[:12]
        patch = "dirty" if row.get("patch_sha256") else "clean"
        ready = "ready" if row.get("ready") else "not-ready"
        print(f"  {index}. {build_id}  {base}  {patch}  {ready}")
    while True:
        value = prompt("Choose build number or id", "1")
        if value.isdigit():
            index = int(value) - 1
            if 0 <= index < len(rows):
                return rows[index]["build_id"]
        if value:
            return value


def configure_interactive_suite(args: argparse.Namespace) -> None:
    if not should_prompt(args):
        return

    args.benchmark = prompt_multi(
        "Which benchmarks should run?",
        INTERACTIVE_BENCHMARK_CHOICES,
        ["qna", "rf"],
        aliases=INTERACTIVE_BENCHMARK_ALIASES,
    )

    if not getattr(args, "build", None):
        build_choice = prompt_choice(
            "Build selection",
            ["auto", "existing-build"],
            "auto",
        )
        if build_choice == "existing-build":
            args.build = choose_existing_build(args)

    model_choice = prompt_choice("Base model", INTERACTIVE_MODEL_CHOICES, "sonnet-4.6")
    if model_choice == "custom-zed-model":
        args.model = prompt("Zed model id (provider/model)", args.model)
    elif model_choice == "custom-baseten-model":
        args.model_provider = "baseten"
        args.baseten_model = prompt_required("Baseten model id")
    else:
        args.model = model_choice

    judge_choice = prompt_choice(
        "Judge preset",
        ["auto", "leaderboard", "deepseek-v4-pro", "kimi-k2.7-code", "gpt55"],
        args.judge or "auto",
    )
    args.judge = judge_choice
