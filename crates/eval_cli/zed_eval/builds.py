from __future__ import annotations

import argparse
from typing import Any

from . import source


def validate_build_id(build_id: str | None) -> None:
    if build_id and source.sanitize_namespace(build_id) != build_id:
        raise ValueError(
            "build ids may only contain lowercase letters, numbers, '.', '_', and '-'"
        )


def prepare_build_request(
    *,
    base_sha: str | None,
    patch_path: str | None,
    build_id: str | None,
    allow_untracked: bool,
    require_clean: bool,
    repo_url: str | None,
    clean_source: bool = False,
    source_label: str | None = None,
    pre_resolved_base_sha: str | None = None,
) -> dict[str, Any]:
    validate_build_id(build_id)
    source_info, patch = source.prepare_build_source(
        base_sha=base_sha,
        patch_path=patch_path,
        allow_untracked=allow_untracked,
        require_clean=require_clean,
        repo_url=repo_url,
        clean=clean_source,
        source_label=source_label,
        pre_resolved_base_sha=pre_resolved_base_sha,
    )
    resolved_build_id = build_id or source.compute_build_id(source_info=source_info)
    return {
        "build_id": resolved_build_id,
        "base_sha": source_info["base_sha"],
        "patch": patch,
        "patch_sha256": source_info.get("patch_sha256"),
        "source": source_info,
        "toolchain": source.build_toolchain_info(),
    }


def resolve_source(
    args: argparse.Namespace,
) -> tuple[str | None, bool, str | None, str | None]:
    """Unify the source selectors into
    `(base_sha, clean_source, source_label, pre_resolved_base_sha)`.

    Precedence: `--from` wins over `--zed-version` / `--base-sha`.

      --from local         -> current HEAD + tracked patch (dev iteration)
      --from <ref/tag/sha>  -> clean build of that ref, resolved canonically
                               against the remote so everyone shares one build
    """
    repo_url = getattr(args, "repo_url", None) or source.DEFAULT_REPO_URL
    from_source = getattr(args, "from_source", None)
    if from_source:
        if from_source.strip().lower() == "local":
            return (
                getattr(args, "base_sha", None),
                bool(getattr(args, "clean_source", False)),
                None,
                None,
            )
        resolved = source.resolve_remote_ref(repo_url, from_source)
        return None, True, from_source, resolved

    zed_version = getattr(args, "zed_version", None)
    if zed_version:
        resolved = source.resolve_remote_ref(repo_url, zed_version)
        return None, True, zed_version, resolved

    return (
        getattr(args, "base_sha", None),
        bool(getattr(args, "clean_source", False)),
        None,
        None,
    )
