from __future__ import annotations

import hashlib
import json
import os
import re
import subprocess
from pathlib import Path
from typing import Any

BUILD_SCHEMA_VERSION = "agent-evals-build-v3"
SOURCE_SCHEMA_VERSION = "agent-evals-source-v1"
BUILD_TARGET = "x86_64-unknown-linux-musl"
BUILD_IMAGE_RECIPE_VERSION = "agent-evals-build-image-v1"
RUST_VERSION = "1.95.0"
RUST_IMAGE_TAG = f"rust:{RUST_VERSION}"
RUST_IMAGE_DIGEST = (
    "sha256:f49565f188ee00bc2a18dd418183f2c5f23ef7d6e691890517ed341a598f67c3"
)
RUST_IMAGE = f"{RUST_IMAGE_TAG}@{RUST_IMAGE_DIGEST}"
ZIG_VERSION = "0.15.2"
CARGO_ZIGBUILD_VERSION = "0.22.3"
DEFAULT_REPO_URL = os.environ.get(
    "AGENT_EVALS_REPO_URL", "https://github.com/zed-industries/zed.git"
)


def package_root() -> Path:
    return Path(__file__).resolve().parent


def eval_cli_root() -> Path:
    return package_root().parent


def repo_root() -> Path:
    try:
        result = subprocess.run(
            ["git", "rev-parse", "--show-toplevel"],
            cwd=eval_cli_root(),
            check=True,
            capture_output=True,
            text=True,
        )
        return Path(result.stdout.strip())
    except (OSError, subprocess.CalledProcessError):
        return eval_cli_root().parents[1]


def git_output(args: list[str], cwd: Path | None = None) -> str:
    result = subprocess.run(
        ["git", *args],
        cwd=cwd or repo_root(),
        check=True,
        capture_output=True,
        text=True,
    )
    return result.stdout.strip()


def git_bytes(args: list[str], cwd: Path | None = None) -> bytes:
    result = subprocess.run(
        ["git", *args],
        cwd=cwd or repo_root(),
        check=True,
        capture_output=True,
    )
    return result.stdout


def git_path_list(args: list[str]) -> list[str]:
    data = git_bytes([*args, "-z"])
    return [path.decode("utf-8") for path in data.split(b"\0") if path]


def current_base_sha() -> str:
    return git_output(["rev-parse", "HEAD"])


def commit_present(sha: str) -> bool:
    """Whether `sha` resolves to a commit object in the local repo."""
    result = subprocess.run(
        ["git", "-C", repo_root(), "cat-file", "-e", f"{sha}^{{commit}}"],
        capture_output=True,
    )
    return result.returncode == 0


def base_sha_on_main(base_sha: str, repo_url: str) -> bool:
    """Whether `base_sha` is reachable from origin/main.

    Resolves origin/main's tip against the remote (so it doesn't depend on the
    caller's possibly-stale local refs), fetching main if the objects aren't
    present locally, then checks ancestry. Raises only when the main tip can't be
    made available locally (a network/remote problem), so callers can
    distinguish that from a definitive "not on main".
    """
    main_sha = resolve_remote_ref(repo_url, "main")
    if base_sha.lower() == main_sha.lower():
        return True
    if not (commit_present(base_sha) and commit_present(main_sha)):
        subprocess.run(
            ["git", "-C", repo_root(), "fetch", "--quiet", repo_url, "main"],
            capture_output=True,
        )
    if not commit_present(main_sha):
        raise RuntimeError(
            f"origin/main tip {main_sha[:12]} unavailable locally after fetch"
        )
    if not commit_present(base_sha):
        # Not present even after pulling main's history -> not reachable from main.
        return False
    ancestry = subprocess.run(
        ["git", "-C", repo_root(), "merge-base", "--is-ancestor", base_sha, main_sha],
        capture_output=True,
    )
    if ancestry.returncode in (0, 1):
        return ancestry.returncode == 0
    raise RuntimeError(
        (ancestry.stderr or b"").decode(errors="replace").strip()
        or "git merge-base --is-ancestor failed"
    )


def resolve_git_ref(ref: str) -> str:
    return git_output(["rev-parse", "--verify", f"{ref}^{{commit}}"])


_FULL_SHA_RE = re.compile(r"[0-9a-fA-F]{40}")


def resolve_remote_ref(repo_url: str, ref: str) -> str:
    """Resolve a git ref/tag/branch/SHA to a canonical commit SHA against the
    *remote* repo, so the resulting build id is identical for every launcher
    regardless of what they happen to have fetched locally.

    A full 40-char SHA is already canonical and is returned (lowercased) without
    a network call. For named refs and tags this uses `git ls-remote`, preferring
    the peeled commit of an annotated tag.
    """
    if _FULL_SHA_RE.fullmatch(ref):
        return ref.lower()
    output = git_output(
        [
            "ls-remote",
            repo_url,
            ref,
            f"refs/tags/{ref}",
            f"refs/heads/{ref}",
        ]
    )
    entries: list[tuple[str, str]] = []
    for line in output.splitlines():
        sha, _, name = line.partition("\t")
        if sha and name:
            entries.append((name, sha))
    # An annotated tag yields both the tag object and a peeled "<ref>^{}" entry
    # pointing at the underlying commit; the commit is what we want to build.
    for name, sha in entries:
        if name.endswith("^{}"):
            return sha
    if entries:
        return entries[0][1]
    raise ValueError(
        f"could not resolve ref '{ref}' against {repo_url}. "
        "Push the commit/tag, or pass a full commit SHA."
    )


def current_ref_name() -> str | None:
    try:
        ref_name = git_output(["rev-parse", "--abbrev-ref", "HEAD"])
    except (OSError, subprocess.CalledProcessError):
        return None
    return None if ref_name == "HEAD" else ref_name


def current_tracked_patch(base_sha: str) -> str:
    result = subprocess.run(
        ["git", "diff", "--binary", base_sha],
        cwd=repo_root(),
        check=True,
        capture_output=True,
    )
    return result.stdout.decode("utf-8")


def read_patch(patch_path: str | None, base_sha: str, *, clean: bool = False) -> str:
    if clean:
        return ""
    if not patch_path:
        return current_tracked_patch(base_sha)
    return Path(patch_path).read_text()


def untracked_files() -> list[str]:
    return git_path_list(["ls-files", "--others", "--exclude-standard"])


def sha256_text(text: str) -> str:
    return hashlib.sha256(text.encode()).hexdigest()


def format_untracked_warning(files: list[str]) -> str:
    preview = "\n".join(f"  - {path}" for path in files[:20])
    if len(files) > 20:
        preview += f"\n  ... and {len(files) - 20} more"
    return (
        "untracked files are present and will NOT be included in the build patch.\n"
        "Commit/remove them, or pass --allow-untracked to proceed anyway.\n"
        f"{preview}"
    )


def prepare_build_source(
    *,
    base_sha: str | None,
    patch_path: str | None,
    allow_untracked: bool,
    require_clean: bool,
    repo_url: str | None,
    clean: bool = False,
    source_label: str | None = None,
    pre_resolved_base_sha: str | None = None,
) -> tuple[dict[str, Any], str]:
    # `pre_resolved_base_sha` is a canonical SHA already resolved against the
    # remote (see `resolve_remote_ref`); it must be used verbatim because the
    # launcher may not have that commit fetched locally to `rev-parse`.
    if pre_resolved_base_sha:
        resolved_base_sha = pre_resolved_base_sha
    elif base_sha:
        resolved_base_sha = resolve_git_ref(base_sha)
    else:
        resolved_base_sha = current_base_sha()
    resolved_repo_url = repo_url or DEFAULT_REPO_URL
    patch = read_patch(patch_path, resolved_base_sha, clean=clean)
    patch_sha256 = sha256_text(patch) if patch.strip() else None
    untracked = [] if clean else untracked_files()
    is_dirty = bool(patch.strip())

    if untracked and not allow_untracked:
        raise ValueError(format_untracked_warning(untracked))

    if require_clean and is_dirty:
        raise ValueError(
            "tracked changes are present. Commit or stash them, or omit --require-clean."
        )

    return {
        "schema": SOURCE_SCHEMA_VERSION,
        "type": "git_patch",
        "repo_url": resolved_repo_url,
        "base_sha": resolved_base_sha,
        "base_ref": source_label or (base_sha if clean else current_ref_name()),
        "patch_sha256": patch_sha256,
        "is_dirty": is_dirty,
        "patch_path": "source.patch" if patch.strip() else None,
        "untracked_files": untracked,
        "untracked_files_included": False,
        "allow_untracked": allow_untracked,
        "require_clean": require_clean,
        "clean_source": clean,
    }, patch


def public_source_info(source_info: dict[str, Any]) -> dict[str, Any]:
    return dict(source_info)


def build_toolchain_info(
    *,
    target: str = BUILD_TARGET,
    rust_image: str = RUST_IMAGE,
    zig_version: str = ZIG_VERSION,
    cargo_zigbuild_version: str = CARGO_ZIGBUILD_VERSION,
) -> dict[str, str]:
    return {
        "build_image_recipe_version": BUILD_IMAGE_RECIPE_VERSION,
        "target": target,
        "rust_version": RUST_VERSION,
        "rust_image": rust_image,
        "rust_image_tag": RUST_IMAGE_TAG,
        "rust_image_digest": RUST_IMAGE_DIGEST,
        "zig_version": zig_version,
        "cargo_zigbuild_version": cargo_zigbuild_version,
    }


def compute_build_id(
    *,
    source_info: dict[str, Any],
    target: str = BUILD_TARGET,
    rust_image: str = RUST_IMAGE,
    zig_version: str = ZIG_VERSION,
    cargo_zigbuild_version: str = CARGO_ZIGBUILD_VERSION,
) -> str:
    payload = {
        "schema": BUILD_SCHEMA_VERSION,
        "source_type": "git_patch",
        "base_sha": source_info["base_sha"],
        "patch_sha256": source_info["patch_sha256"],
        **build_toolchain_info(
            target=target,
            rust_image=rust_image,
            zig_version=zig_version,
            cargo_zigbuild_version=cargo_zigbuild_version,
        ),
    }
    digest = hashlib.sha256(json.dumps(payload, sort_keys=True).encode()).hexdigest()
    return f"bld-{digest[:20]}"


def sanitize_namespace(value: str) -> str:
    sanitized = re.sub(r"[^A-Za-z0-9_.-]+", "-", value.strip().lower()).strip("-.")
    return sanitized or "default"


def default_namespace() -> str:
    configured = os.environ.get("AGENT_EVALS_NAMESPACE")
    if configured:
        return sanitize_namespace(configured)

    try:
        email = git_output(["config", "user.email"])
        if email:
            return sanitize_namespace(email.split("@", 1)[0])
    except (OSError, subprocess.CalledProcessError):
        pass

    return sanitize_namespace(
        os.environ.get("USER") or os.environ.get("LOGNAME") or "default"
    )
