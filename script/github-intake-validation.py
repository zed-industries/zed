#!/usr/bin/env python3
"""
intake_validation.py
====================

Deterministic intake validation bot for `zed-industries/zed` GitHub issues.
Runs on issue open/edit/reopen and on new comments. Performs two checks:

1. MISSING-INFO   The body has no parseable `Zed:` or `OS:` line, or
                  contains only the template's placeholder example.
2. OUTDATED       The reporter's version is a minor (or more) behind
                  current stable — strict same-minor rule, patch ignored,
                  no grace window (deliberate; see `is_outdated`).
                  Non-stable channels are version-compared too, not
                  exempt; ahead-of-stable is silent automatically.

The bot posts a comment + applies `state:needs info` for either check.
On reporter follow-up (body edit or new comment), the bot re-parses and
clears the label if valid info is now present.

Modes
-----
- `live`     Calls GitHub API to post comments + add/remove labels.
- `shadow`   Writes structured "would have done X" records to
             $GITHUB_STEP_SUMMARY. No API writes. The pre-prod stress mode.
- `dry-run`  Local only: prints decisions to stdout, no API calls at all.

Auth
----
Reads GITHUB_TOKEN from env (or `gh auth token` as a fallback for local
runs). Inside the workflow we pass the community-bot app token.

Exit codes
----------
0 — script ran cleanly (including "no action needed").
1 — fatal error (auth, malformed input, etc.).

Outdated-version-lookup failures are NOT fatal: they downgrade the run
to missing-info-only and log a warning. A wrong "you're outdated" comment
is the worst failure mode, so we bias toward under-triggering.
"""

from __future__ import annotations

import argparse
import json
import os
import re
import shutil
import subprocess
import sys
from dataclasses import asdict, dataclass, field
from datetime import datetime, timezone
from enum import Enum
from pathlib import Path
from typing import Optional

import requests

# ---------------------------------------------------------------------------
# Constants

REPO_OWNER = "zed-industries"
REPO_NAME = "zed"
REPO_FULL = f"{REPO_OWNER}/{REPO_NAME}"

GITHUB_API = "https://api.github.com"

# Feature flag: whether the bot requires an OS in addition to a Zed version.
# Set to False (current) for a VERSION-ONLY bot — we still parse OS info
# (it's surfaced in the replay/CSV for context and future use), but a
# missing OS does NOT trigger a prompt. Flip to True to re-enable the
# OS-gating behavior preserved in `archive/`. See archive/README.md.
REQUIRE_OS = False

LABEL_NEEDS_INFO = "state:needs info"

# Issue types the bot gates on. Mirrors the dup bot's `should_skip()`:
# typed issues that are neither Bug nor Crash (Feature, Meta, Tracking,
# ...) are skipped entirely — they don't have a spec section, so asking
# them for a Zed version would be a guaranteed false positive. UNTYPED
# issues are allowed through: a brand-new bug report may not be typed
# yet, and those are exactly the issues we want to validate.
GATED_ISSUE_TYPES = {"Bug", "Crash"}

# Marker comments — used to keep the bot idempotent. Each is appended as an
# HTML comment to the bottom of the bot's posts. On every re-trigger we scan
# the issue's bot comments for these markers before deciding to post again.
MARKER_MISSING_INFO = "<!-- intake-validation: missing-info -->"
MARKER_OUTDATED_VERSION = "<!-- intake-validation: outdated-version -->"

# The git SHA in the bug/crash template's example spec block. The example
# is a YAML-form `placeholder:` (greyed-out, never submitted), so this
# exact SHA only appears when someone deliberately quotes the example.
# Lines containing it are stripped before parsing so the example's
# `v0.215.0` is never read as the reporter's version; real commit SHAs
# don't match. Update if the template's example changes.
TEMPLATE_PLACEHOLDER_SHA = "bfe141ea79aa4984028934067ba75c48d99136ae"

# Keys that appear in the system-specs output of
# `zed: copy system specs into clipboard`. Order matters only for
# readability — the parser handles them in any order.
SPEC_KEYS = ("Zed", "OS", "Memory", "Architecture", "GPU")

# Channels we recognize. Anything else parses as None and is treated as
# "unknown channel" — which we map to Stable for outdated checking.
KNOWN_CHANNELS = {"stable", "preview", "dev", "nightly"}


# ---------------------------------------------------------------------------
# Logging


def log(msg: str, level: str = "INFO") -> None:
    ts = datetime.now(timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ")
    print(f"[{ts}] [{level}] {msg}", file=sys.stderr, flush=True)


# ---------------------------------------------------------------------------
# Auth


def get_token() -> str:
    token = os.environ.get("GITHUB_TOKEN", "").strip()
    if token:
        return token
    if not shutil.which("gh"):
        sys.exit("ERROR: GITHUB_TOKEN not set and `gh` not on PATH")
    try:
        return subprocess.check_output(["gh", "auth", "token"], text=True).strip()
    except subprocess.CalledProcessError as e:
        sys.exit(f"ERROR: failed to read token from gh: {e}")


# ---------------------------------------------------------------------------
# Version


@dataclass(frozen=True, order=True)
class Version:
    """Semver-ish (major, minor, patch). Comparisons are tuple-order."""

    major: int
    minor: int
    patch: int

    def __str__(self) -> str:
        return f"v{self.major}.{self.minor}.{self.patch}"

    @classmethod
    def parse(cls, raw: str) -> Optional["Version"]:
        """Parse a Zed version string into (major, minor, patch).

        Tries progressively looser forms (the looser ones mainly matter
        for older v0.x reporters who use shorthand):
          - `v1.2.3` / `1.2.3` / `1.2.3+stable.42.SHA`  -> (1, 2, 3)
          - `1.4` (patch omitted)                       -> (1, 4, 0)
          - `.227` (major + patch omitted, v0.x shorthand) -> (0, 227, 0)

        Returns None when nothing version-like is present.
        """
        if raw is None:
            return None
        s = raw.strip()
        # 1. Full 3-component (preferred).
        m = re.search(r"\bv?(\d+)\.(\d+)\.(\d+)\b", s)
        if m:
            return cls(int(m.group(1)), int(m.group(2)), int(m.group(3)))
        # 2. Two-component, patch implied 0 (`1.4`).
        m = re.search(r"\bv?(\d+)\.(\d+)\b", s)
        if m:
            return cls(int(m.group(1)), int(m.group(2)), 0)
        # 3. Leading-dot minor, major + patch implied 0 (`.227`). Last
        #    resort — only reached for strings with no fuller version form.
        m = re.search(r"(?<!\d)\.(\d+)\b", s)
        if m:
            return cls(0, int(m.group(1)), 0)
        return None


def is_outdated(reported: Version, current_stable: Version) -> bool:
    """Outdated iff the reported (major, minor) is strictly less than the
    current stable's (major, minor). Patch is intentionally ignored — patch
    releases ship 2-4x/week and we don't want the bot pinging reporters who
    are one patch behind.

    The strictness here means a Wednesday minor promotion immediately
    flips every reporter on the previous minor into "outdated" status the
    moment the new minor is published. That's the intended behavior at
    this stage: we want the data from a sharp rule before deciding whether
    to soften it with a grace window. See `spec.md` and the Linear
    discussion for context."""
    return (reported.major, reported.minor) < (
        current_stable.major,
        current_stable.minor,
    )


# ---------------------------------------------------------------------------
# Parser


@dataclass
class ParsedSpecs:
    """What we extracted from the issue body.

    Design note on the OS side: trying to maintain an exhaustive list of
    distro names and OS-token variants doesn't scale. The bot's job is to
    detect whether the reporter put SOMETHING substantive in the OS field,
    not to validate the format. So `has_os` gates on `os_raw` (the literal
    value the reporter wrote, minus obvious placeholders), and
    `os_family` / `os_version` are best-effort for display purposes only.
    """

    zed_version: Optional[Version] = None
    zed_channel: Optional[str] = None  # "stable" | "preview" | "dev" | "nightly" | None
    os_raw: Optional[str] = None  # stripped value after `OS:`, None if placeholder
    os_family: Optional[str] = None  # best-effort canonical family (display only)
    os_version: Optional[str] = None  # best-effort numeric version (display only)
    # Reporter offered a commit SHA as their version (local/source build).
    # No semver to compare, but they DID identify their build — prompting
    # them would be a false positive, so this makes missing-info silent.
    has_commit_ref: bool = False
    # Reporter wrote "latest" (or similar) where the version goes. Still
    # missing_info — "latest" drifts with channel and auto-update timing —
    # but the prompt copy acknowledges their answer and names current
    # stable so they can confirm the exact build.
    says_latest: bool = False

    @property
    def has_zed(self) -> bool:
        return self.zed_version is not None

    @property
    def has_os(self) -> bool:
        # Any non-placeholder value in the OS field counts. We deliberately
        # don't gate on `os_family` because real-world OS values include
        # `Arch Linux`, `CachyOS`, `Win11`, `Ubuntu 20.04 (Focal)`, etc.
        # — the reporter clearly provided info even if we can't classify it.
        return self.os_raw is not None

    @property
    def is_complete(self) -> bool:
        # Version is always required. OS is only required when REQUIRE_OS is
        # on (currently off — version-only bot).
        if not self.has_zed:
            return False
        if REQUIRE_OS and not self.has_os:
            return False
        return True

    def to_dict(self) -> dict:
        return {
            "zed_version": str(self.zed_version) if self.zed_version else None,
            "zed_channel": self.zed_channel,
            "os_raw": self.os_raw,
            "os_family": self.os_family,
            "os_version": self.os_version,
            "has_commit_ref": self.has_commit_ref,
            "says_latest": self.says_latest,
            "is_complete": self.is_complete,
        }


# Spec-line regexes. We anchor each to its own validation pattern (Zed: must
# be followed by a version; OS: must be followed by a known family) rather
# than using a generic key splitter, which would otherwise match prose like
# `... as LLM in Zed:` and then capture arbitrary text as the value.
#
# Both regexes work for multi-line spec blocks AND single-line concatenated
# blocks (e.g. `Zed: v1.2.7+stable.42.SHA (Zed) OS: Linux X11 ubuntu 24.04`),
# because we bound by the next spec key rather than by newline.

# Boundary for the trailing content of a spec key. We stop at:
#   - the next spec key (so single-line concatenated specs work:
#     `Zed: v1.2.7 (Zed) OS: macOS 15.1 Memory: 36 GiB`)
#   - a newline (so multi-line specs don't accidentally absorb prose
#     written below the spec block)
#   - end of string
_NEXT_KEY_BOUNDARY = r"(?=\s+(?:Zed|OS|Memory|Architecture|GPU)\s*:|\n|$)"

# Measurement-unit words that disqualify a number from being a Zed version.
# Prose like `Zed eats 1.5 GB of RAM`, `Zed takes .5s to open`, or
# `Zed froze for 1.5 minutes` puts a decimal within the prefix-tolerance
# window of the word "Zed"; without this guard those decimals parse as
# versions — worst case an outdated-version comment about "v0.5.0". A
# number immediately followed by a unit word is a measurement, not a
# version, so we reject the match and let the body fall through to the
# other strategies (usually ending in missing_info, which is the correct
# answer for these bodies). Extend the list via fixtures as new unit
# false-positives show up in replay.
_UNIT_WORDS = (
    r"s\b|secs?\b|seconds?\b|ms\b|mins?\b|minutes?\b|hours?\b|days?\b"
    r"|%|x\b|times\b|stars?\b"
    r"|[KMGT]i?B\b|[KMGT]Hz\b|fps\b|px\b|pt\b"
)
_NOT_A_UNIT_AHEAD = r"(?!\s{0,1}(?:" + _UNIT_WORDS + r"))"

_ZED_SPEC_RE = re.compile(
    # `Zed` followed by a parseable version. Group 1: version (with optional
    # channel suffix — either `+channel.N.SHA` (full v1.x build metadata) or
    # the bare `+channel` form seen on early v1.0.x releases). Group 2:
    # optional `(Zed Nightly SHA)` legacy parenthetical channel hint.
    #
    # Prefix tolerates:
    #   - `Zed:` / `Zed ` (the usual forms)
    #   - `Zed version:` / `Zed version ` (the `--version` / About dialog forms)
    #   - markdown emphasis around the label: `**Zed**:`, `__Zed__:`, `` `Zed` ``
    # Then any of:
    #   - colon (`Zed: 1.2.7`)
    #   - whitespace alone (`Zed 1.2.7`)
    #   - colon + backticks / brackets (`Zed: `v1.2.7``)
    # Version capture accepts 3-component, 2-component (`1.4`), or the
    # leading-dot v0.x shorthand (`.227` — 2+ digits required, so prose
    # decimals like `.5s` can't masquerade as versions).
    # The trailing `_NOT_A_UNIT_AHEAD` guard rejects measurements
    # (`1.5 GB`, `3.4 seconds`) — see `_UNIT_WORDS` above.
    r"\bZed[*_`]{0,2}(?:\s+version)?[*_`]{0,2}[:\s]\s*(?P<gap>[^:\n]{0,15}?)?"
    r"v?(?P<ver>\d+\.\d+\.\d+(?:\+\w+(?:\.\d+\.[0-9a-f]+)?)?|\d+\.\d+|\.\d{2,})"
    + _NOT_A_UNIT_AHEAD
    + r"\s*(?:\((?P<paren>Zed(?:\s+\w+)?)[^)]*\))?",
    re.IGNORECASE,
)

# Emphasized `Zed` label at the start of a line, e.g.
#     - **Zed**: Stable channel (auto-updated)1.4.4
# (real intake #58294). Reporters who build their own "Environment"
# lists often put prose between the label and the version, blowing past
# the 15-char gap `_ZED_SPEC_RE` tolerates. A markdown-emphasized label
# at line start is a strong structural signal that the line is a spec
# field rather than prose, so we allow a longer same-line gap (60 chars)
# — but ONLY for the emphasized form. Plain mid-sentence `Zed: ...` prose
# never gets the long gap.
_ZED_LABELED_LINE_RE = re.compile(
    r"^[ \t]*(?:[-*+]\s+|>\s*)*[*_`]{1,3}Zed[*_`]{1,3}(?:\s+version)?\s*:\s*"
    r"(?P<gap>[^\n]{0,60}?)"
    r"v?(?P<ver>\d+\.\d+\.\d+(?:\+\w+(?:\.\d+\.[0-9a-f]+)?)?|\d+\.\d+)"
    + _NOT_A_UNIT_AHEAD,
    re.IGNORECASE | re.MULTILINE,
)

# Commit-SHA-as-version detection. Reporters building Zed from source
# often identify their build by commit instead of a version (real intake
# #58699: `Zed: most recent commit: 6d68395c...`). There's no semver to
# compare, but they DID identify their build — prompting them is a false
# positive (team call, Lucas 2026-06). A bare hex string is NOT enough:
# issue bodies are full of SHAs from logs, stack traces, and "introduced
# by commit X" references. We accept only shapes where the SHA is offered
# AS the reporter's build:
#   1. the word commit/build/rev(ision) + hex (7-40 chars), same line
#   2. a zed-industries/zed commit URL
# plus, in `_extract_zed_from_section` only, a bare hex token inside the
# spec section (the template told them to put their version there).
# The `(?=[0-9]*[a-f])` lookahead requires at least one hex letter, so
# pure-decimal runs (dates like 20260325, build ids) don't count as SHAs.
_COMMIT_REF_RE = re.compile(
    r"\b(?:commit|build|rev(?:ision)?)\b[:\s`]*(?=[0-9]*[a-f])[0-9a-f]{7,40}\b"
    r"|github\.com/zed-industries/zed/commit/[0-9a-f]{7,40}\b",
    re.IGNORECASE,
)
_BARE_SHA_RE = re.compile(r"\b(?=[0-9]*[a-f])[0-9a-f]{7,40}\b", re.IGNORECASE)

# "latest"-as-version detection. Reporters answer the version section
# with `Zed: latest stable` (#58386) or just `most latest zed version`
# (#57222). The word "latest" is everywhere in prose ("the latest update
# broke X"), so detection is deliberately narrow — two shapes only:
#   1. a `Zed:`-labeled line whose value contains "latest" (≤15-char gap,
#      same prose tolerance as version parsing)
#   2. a SHORT (≤40 chars), digit-free line containing "latest" inside
#      the spec-section chunk (the template told them to answer there;
#      digit-free because a line with digits would carry a real version)
# Only consulted when no version parsed; the result is still
# missing_info, just with copy that acknowledges the answer.
_ZED_LATEST_LABEL_RE = re.compile(
    r"\bZed[*_`]{0,2}(?:\s+version)?[*_`]{0,2}\s*:\s*[^:\n]{0,15}?\blatest\b",
    re.IGNORECASE,
)
_LATEST_WORD_RE = re.compile(r"\blatest\b", re.IGNORECASE)

# `### Zed version and system specs` (or `## Zed version ...`) — the
# canonical section header in the bug template. Used by the bare-version
# fallback below when the main `_ZED_SPEC_RE` finds nothing.
_SPEC_SECTION_HEADER_RE = re.compile(
    r"^#{1,4}\s*Zed\s+version[^\n]*$",
    re.IGNORECASE | re.MULTILINE,
)

# A bare semver-ish version, no `Zed` prefix required. Used only inside the
# spec section (bounded chunk) so this very-permissive regex doesn't fire on
# random version mentions elsewhere in the body.
_BARE_VERSION_RE = re.compile(r"\bv?(\d+\.\d+\.\d+(?:\+\w+(?:\.\d+\.[0-9a-f]+)?)?)\b")

# Other spec keys that bound the "this is the Zed version" search inside
# the section. Once we hit `OS:`, `Memory:`, etc., we're past the Zed line.
_OTHER_SPEC_KEY_RE = re.compile(
    r"\b(?:OS|Memory|Architecture|GPU)\s*:",
    re.IGNORECASE,
)

# Template noise we strip before body-scanning for OS. The bug template
# includes the literal line:
#     ### If you are using WSL on Windows, what flavor of Linux are you using?
# which contains both `Windows` and `Linux` tokens. The reporter's answer
# is on a subsequent line; the question itself is just template boilerplate
# and shouldn't drive family detection.
_TEMPLATE_NOISE_RE = re.compile(
    r"#{1,4}\s*If you are using WSL on Windows[^\n]*",
    re.IGNORECASE,
)

# `OS:` followed by some value. We capture the rest of the value up to the
# next spec key boundary so we can search inside it for a family (or distro
# that implies Linux) and a version token. We deliberately don't anchor on
# a specific family here — real-world `OS:` values include `Arch Linux`,
# `CachyOS`, `Ubuntu 20.04 (Focal)`, etc., where the canonical family isn't
# the first token. Family detection happens against `rest` via
# `_detect_os_family`.
_OS_KEY_RE = re.compile(
    r"\bOS\s*:\s*(?P<rest>.+?)" + _NEXT_KEY_BOUNDARY,
    re.IGNORECASE | re.DOTALL,
)

# Obvious-placeholder OS values. If the reporter typed any of these into
# the OS field (case-insensitive, after stripping whitespace), we treat
# the field as not-filled-in. This list is intentionally TINY — the
# tradeoff is being lenient on "is this a real OS?" so we don't have to
# maintain a catalog of every distro and abbreviation under the sun.
_OS_PLACEHOLDER_VALUES = frozenset(
    {
        "",
        "n/a",
        "na",
        "none",
        "tbd",
        "idk",
        "unknown",
        "?",
        ".",
    }
)

# Linux distros we recognize as implying "Linux" when no canonical family
# token is present. Used both inside `OS:` values (e.g. `OS: CachyOS`) and
# in the body-scan fallback (e.g. `Fedora release 44` on a standalone
# line). Order doesn't matter; all map to "Linux".
_LINUX_DISTRO_ALIASES = (
    "ubuntu",
    "fedora",
    "arch",
    "debian",
    "mint",
    "nixos",
    "manjaro",
    "opensuse",
    "suse",
    "centos",
    "rhel",
    "gentoo",
    "pop[\\s_!-]?os",
    "cachyos",
    "endeavouros",
    "slackware",
    "alpine",
    "tumbleweed",
    "kali",
    "raspbian",
    "elementary",
    "void",
    "garuda",
    "asahi",
)

# OS family detection. Used both to populate the display field `os_family`
# (when we have an explicit `OS:` value) and as the body-scan fallback
# when no `OS:` key is found at all.
_OS_FAMILY_PATTERNS: tuple[tuple[re.Pattern[str], str], ...] = (
    (re.compile(r"\b(?:macOS|Mac\s*OS|OS\s*X)\b", re.IGNORECASE), "macOS"),
    (re.compile(r"\b(?:Windows|Win\s*\d+)\b", re.IGNORECASE), "Windows"),
    (re.compile(r"\bFreeBSD\b", re.IGNORECASE), "FreeBSD"),
    (re.compile(r"\bOpenBSD\b", re.IGNORECASE), "OpenBSD"),
    (re.compile(r"\bLinux\b", re.IGNORECASE), "Linux"),
    (
        re.compile(r"\b(?:" + "|".join(_LINUX_DISTRO_ALIASES) + r")\b", re.IGNORECASE),
        "Linux",
    ),
)

# Numeric version token: 1-4 dotted components. Allows `15.1`, `24.04`, `11`,
# and `15.1.2`. Used to find the OS version in the trailing OS text.
_VERSION_TOKEN_RE = re.compile(r"\b(\d+(?:\.\d+){0,3})\b")


def _channel_from_version_suffix(version_segment: str) -> Optional[str]:
    """Extract `stable` from `v1.2.7+stable.42.SHA`-style strings AND from
    the bare `v1.0.1+stable` form (no build metadata) seen on early v1.0.x
    releases. Both are valid in the wild."""
    m = re.search(r"\+(\w+)(?:\.\d+\.[0-9a-f]+)?", version_segment, re.IGNORECASE)
    if m:
        ch = m.group(1).lower()
        if ch in KNOWN_CHANNELS:
            return ch
    return None


def _channel_from_paren(paren_text: Optional[str]) -> Optional[str]:
    """Extract `nightly` from `(Zed Nightly SHA)`-style legacy parenthetical."""
    if not paren_text:
        return None
    # `Zed Nightly` -> `nightly`. Bare `Zed` (no channel suffix) -> None.
    parts = paren_text.split()
    if len(parts) >= 2:
        ch = parts[1].lower()
        if ch in KNOWN_CHANNELS:
            return ch
    return None


# Standalone channel word, e.g. the `Dev` in `1.4.0 Dev @ <sha>` or the
# `Preview` in `Zed 1.4.0 Preview`. We only scan a small window immediately
# after the version so we don't false-match the word "dev" elsewhere
# (paths like /home/dev, the word "development", etc.).
_CHANNEL_WORD_RE = re.compile(r"\b(stable|preview|dev|nightly)\b", re.IGNORECASE)

# How many characters after a parsed version we'll scan for a trailing
# channel word.
_CHANNEL_WORD_WINDOW = 20


def _channel_from_trailing_word(text: str) -> Optional[str]:
    """Detect a channel name appearing as a standalone word in `text`
    (expected to be the short window right after the version token)."""
    m = _CHANNEL_WORD_RE.search(text)
    if m:
        return m.group(1).lower()
    return None


def _best_version_match(pattern: re.Pattern[str], body: str) -> Optional[re.Match[str]]:
    """First match wins — unless a later match carries `+channel` build
    metadata (`v0.232.2+stable.229.SHA`), which is machine-generated
    `copy system specs` output and always beats a bare prose mention.
    Real intake #54311: prose said "last working version of Zed 0.230.2"
    above a spec block showing the reporter was actually on current
    stable — first-match-wins would have flagged an up-to-date reporter
    as outdated."""
    first = None
    for m in pattern.finditer(body):
        if first is None:
            first = m
        if "+" in m.group("ver"):
            return m
    return first


def _extract_zed(body: str) -> tuple[Optional[Version], Optional[str]]:
    """Find the Zed version (and channel, if detectable) in the body.

    Three strategies tried in order:

    1. Explicit `Zed ...` prefix (or `Zed version ...`, `Zed: ...`, etc.).
       Catches the common formats including the About-dialog / release-page
       `Zed 1.3.5` and the `Zed version: 1.3.6` from `--version` output.
    2. Emphasized label line (`- **Zed**: Stable channel (auto-updated)1.4.4`,
       real intake #58294). Same idea but allows a longer same-line gap,
       justified by the strong structural signal of a markdown-emphasized
       label at line start.
    3. Bare-version-in-section fallback. Look for the
       `### Zed version and system specs` header and scan the chunk
       between it and the next spec key (`OS:`, `Memory:`, ...) for any
       parseable version. Catches reporters who paste just `v1.4.0.` or
       `1.4.0 Dev @ <sha>` directly under the section header.

    Channel is `stable | preview | dev | nightly` or None when unrecognized.
    """
    for pattern in (_ZED_SPEC_RE, _ZED_LABELED_LINE_RE):
        m = _best_version_match(pattern, body)
        if not m:
            continue
        version = Version.parse(m.group("ver"))
        if version is None:
            continue
        groups = m.groupdict()
        # Channel detection, in priority order:
        #   1. `+channel.N.SHA` / bare `+channel` version suffix
        #   2. `(Zed Nightly SHA)` legacy parenthetical
        #   3. channel word in the gap BEFORE the version
        #      (`Zed Dev 1.1.0`, `**Zed**: Stable channel ... 1.4.4`)
        #   4. standalone channel word right after the version
        #      (`Zed 1.4.0 Preview`)
        channel = (
            _channel_from_version_suffix(m.group("ver"))
            or _channel_from_paren(groups.get("paren"))
            or _channel_from_trailing_word(groups.get("gap") or "")
            or _channel_from_trailing_word(
                body[m.end() : m.end() + _CHANNEL_WORD_WINDOW]
            )
        )
        return version, channel

    return _extract_zed_from_section(body)


def _spec_section_chunk(body: str) -> Optional[str]:
    """The bounded text between the `### Zed version and system specs`
    header and the next spec key or markdown header — the spot the
    template tells reporters to put their version. None if no header."""
    section = _SPEC_SECTION_HEADER_RE.search(body)
    if not section:
        return None
    chunk = body[section.end() : section.end() + 1000]
    boundary_positions = []
    other_key = _OTHER_SPEC_KEY_RE.search(chunk)
    if other_key:
        boundary_positions.append(other_key.start())
    next_header = re.search(r"^#{1,4}\s", chunk, re.MULTILINE)
    if next_header:
        boundary_positions.append(next_header.start())
    if boundary_positions:
        chunk = chunk[: min(boundary_positions)]
    return _TEMPLATE_NOISE_RE.sub("", chunk)


def _detect_says_latest(body: str) -> bool:
    """True when the reporter wrote \"latest\" where their version goes —
    see `_ZED_LATEST_LABEL_RE` for the two accepted shapes. Prose
    \"latest\" anywhere else never triggers this."""
    if _ZED_LATEST_LABEL_RE.search(body):
        return True
    chunk = _spec_section_chunk(body)
    if not chunk:
        return False
    return any(
        len(line.strip()) <= 40
        and _LATEST_WORD_RE.search(line)
        and not any(c.isdigit() for c in line)
        for line in chunk.splitlines()
    )


def _detect_commit_ref(body: str) -> bool:
    """True when the reporter offered a commit SHA as their build id:
    a `commit:`/`build:`/`rev:` + hex anywhere, a zed commit URL, or a
    bare hex token inside the spec section. Bare hex elsewhere in the
    body (logs, stack traces, culprit-commit mentions) doesn't count
    via the bare rule — though `commit <sha>` phrasing anywhere does,
    which errs toward silence by design."""
    if _COMMIT_REF_RE.search(body):
        return True
    chunk = _spec_section_chunk(body)
    return bool(chunk and _BARE_SHA_RE.search(chunk))


def _extract_zed_from_section(body: str) -> tuple[Optional[Version], Optional[str]]:
    """Fallback: find the spec-section header, then scan the bounded chunk
    between it and the next spec key for a bare version. Bounded so we don't
    accidentally pick up version numbers from OS lines or unrelated prose."""
    chunk = _spec_section_chunk(body)
    if chunk is None:
        return None, None
    v_match = _BARE_VERSION_RE.search(chunk)
    if not v_match:
        return None, None
    version = Version.parse(v_match.group(0))
    if version is None:
        return None, None
    # Catch a trailing channel word like the `Dev` in
    # `1.4.0 Dev @ <sha>` so non-stable builds aren't treated as stable.
    channel = _channel_from_version_suffix(
        v_match.group(0)
    ) or _channel_from_trailing_word(
        chunk[v_match.end() : v_match.end() + _CHANNEL_WORD_WINDOW]
    )
    return version, channel


def _extract_os(body: str) -> tuple[Optional[str], Optional[str], Optional[str]]:
    """Find OS info in the body and return (raw, family, version).

    Two strategies, tried in order:

    1. Explicit `OS:` key (e.g. `OS: macOS 15.1`). Anything non-placeholder
       after the colon counts as has_os=True.
    2. Body-scan fallback. If no `OS:` key is found, scan the whole body
       for a known family or Linux-distro token. Reporters who use custom
       formats like:
         === OS ===
         Fedora release 44 (Forty Four)
       or:
         Zed 1.3.5
         Windows 11 25H2
       still get accepted because we detect `Fedora` or `Windows` anywhere
       in the body.

    - `raw` is the stripped value or matched snippet, or None when missing.
    - `family` is the best-effort canonical family (None if unrecognized).
    - `version` is the best-effort numeric version (None if unparseable).

    Tradeoff with the body-scan fallback: an issue that mentions multiple
    OSes in prose ("this also affects Linux users") may give a different
    family than the reporter's actual platform. Acceptable — the triager
    sees the full body and adjusts. The bot's job is to detect whether
    OS info exists, not to perfectly classify it.
    """
    # Strategy 1: explicit `OS:` key
    m = _OS_KEY_RE.search(body)
    if m:
        rest = m.group("rest") or ""
        stripped = rest.strip()
        if stripped.lower() not in _OS_PLACEHOLDER_VALUES:
            family = _detect_os_family(rest)
            tokens = _VERSION_TOKEN_RE.findall(rest)
            version = _pick_os_version(tokens)
            return stripped, family, version

    # Strategy 2: body-scan fallback
    return _scan_body_for_os(body)


def _scan_body_for_os(body: str) -> tuple[Optional[str], Optional[str], Optional[str]]:
    """Search the body for any known OS family or distro mention as a
    fallback when no `OS:` key is present.

    Picks the EARLIEST match by position in the body, not by pattern order.
    The bug template has a hard-coded line
        ### If you are using WSL on Windows, what flavor of Linux are you using?
    which contains both `Windows` and `Linux` tokens. We strip this line
    before scanning so reporters who didn't fill in OS aren't falsely
    classified as Windows-on-WSL just because the template question is
    present.
    """
    cleaned = _TEMPLATE_NOISE_RE.sub("", body)
    best: Optional[tuple[int, str, re.Match[str]]] = None
    for pattern, family in _OS_FAMILY_PATTERNS:
        match = pattern.search(cleaned)
        if not match:
            continue
        if best is None or match.start() < best[0]:
            best = (match.start(), family, match)
    if best is None:
        return None, None, None
    _, family, match = best
    start = max(0, match.start() - 2)
    end = min(len(cleaned), match.end() + 60)
    snippet = cleaned[start:end].strip().replace("\n", " ")
    tokens = _VERSION_TOKEN_RE.findall(snippet)
    version = _pick_os_version(tokens)
    return snippet[:120], family, version


def _detect_os_family(value: str) -> Optional[str]:
    """Best-effort canonical family detection for display only. Returns
    None when none of the canonical patterns match."""
    for pattern, family in _OS_FAMILY_PATTERNS:
        if pattern.search(value):
            return family
    return None


def _pick_os_version(tokens: list[str]) -> Optional[str]:
    """Choose the most-likely OS version from a list of numeric tokens.
    Prefer the last multi-component token (most dots) over bare integers,
    so `macOS Tahoe 26.4.1 (25E253) arm64` picks `26.4.1` over `253`.
    Falls back to the last bare integer when no multi-component tokens
    exist (handles `Windows 11`, `fedora 39`)."""
    if not tokens:
        return None
    multi = [t for t in tokens if "." in t]
    if multi:
        return multi[-1]
    return tokens[-1]


def parse_body(body: str) -> ParsedSpecs:
    """Parse a GitHub issue body. Returns the structured ParsedSpecs.
    Robust to:
    - Multi-line vs. single-line spec blocks
    - Spec keys in any order
    - Markdown headings and surrounding prose
    - Trailing whitespace, mixed casing on keys
    - Placeholder SHA presence (treated as placeholder, not real info)
    - Linux distros where Zed emits `OS: Linux ... unknown` with no
      numeric version (still counts as "has OS" because family is present)
    """
    if not body:
        return ParsedSpecs()

    # Strip lines quoting the template example (identified by its SHA) so
    # the example's `Zed: v0.215.0 (...)` is never read as the reporter's
    # version. Real specs elsewhere in the body still parse normally.
    if TEMPLATE_PLACEHOLDER_SHA in body:
        body = "\n".join(
            line for line in body.splitlines() if TEMPLATE_PLACEHOLDER_SHA not in line
        )

    parsed = ParsedSpecs()
    parsed.zed_version, parsed.zed_channel = _extract_zed(body)
    parsed.os_raw, parsed.os_family, parsed.os_version = _extract_os(body)
    parsed.has_commit_ref = _detect_commit_ref(body)
    parsed.says_latest = _detect_says_latest(body)
    return parsed


def merge_parsed(*parses: ParsedSpecs) -> ParsedSpecs:
    """Combine multiple parses (body + comment bodies) into one. Later
    parses fill in gaps left by earlier ones. Used to decide whether the
    reporter has answered a bot prompt in a follow-up comment.

    Anything present in a parse is real reporter-provided info by
    construction — `parse_body` strips template-example lines before
    parsing — so first-non-None-wins needs no special cases."""
    merged = ParsedSpecs()
    for p in parses:
        if p.zed_version is not None and merged.zed_version is None:
            merged.zed_version = p.zed_version
            merged.zed_channel = p.zed_channel
        if p.has_os and not merged.has_os:
            merged.os_raw = p.os_raw
            merged.os_family = p.os_family
            merged.os_version = p.os_version
        merged.has_commit_ref = merged.has_commit_ref or p.has_commit_ref
        merged.says_latest = merged.says_latest or p.says_latest
    return merged


# ---------------------------------------------------------------------------
# Decision


class Decision(str, Enum):
    SILENT = "silent"
    MISSING_INFO = "missing_info"
    OUTDATED_VERSION = "outdated_version"


@dataclass
class DecisionResult:
    decision: Decision
    reason: str
    marker: Optional[str] = None
    comment_body: Optional[str] = None
    label_to_add: Optional[str] = None
    label_to_remove: Optional[str] = None
    parsed: Optional[ParsedSpecs] = None

    def to_dict(self) -> dict:
        d = asdict(self)
        d["decision"] = self.decision.value
        d["parsed"] = self.parsed.to_dict() if self.parsed else None
        return d


def decide(
    parsed: ParsedSpecs,
    current_stable: Optional[Version],
    issue_has_label: bool,
    existing_markers: set[str],
) -> DecisionResult:
    """Compute the action to take given the parsed body+comments and the
    current state of the issue (label + previously-posted bot markers).

    - `current_stable=None` means the version lookup failed; we silently
      skip the outdated-version check.
    - `issue_has_label=True` means `state:needs info` is currently on the
      issue and may need to be cleared if the reporter has answered.
    - `existing_markers` is the set of bot-marker strings already present
      in bot comments on the issue; used to avoid double-posting.
    """
    # Path 1: complete info present. If the issue currently has the label
    # AND we previously posted a missing-info comment, the reporter has
    # answered — clear the label.
    if parsed.is_complete:
        # Outdated check. This is a pure version comparison against current
        # stable — we do NOT blanket-exempt preview/nightly/dev. A reporter
        # on any channel whose version is genuinely behind current stable
        # is flagged; a reporter ahead of stable (the normal case for
        # preview/nightly) is silent automatically because `is_outdated`
        # requires `reported < stable`. So we're verifying the version,
        # not assuming a channel means "current."
        if (
            current_stable is not None
            and parsed.zed_version is not None
            and is_outdated(parsed.zed_version, current_stable)
        ):
            if MARKER_OUTDATED_VERSION in existing_markers:
                return DecisionResult(
                    decision=Decision.SILENT,
                    reason="already-prompted-outdated",
                    parsed=parsed,
                )
            return DecisionResult(
                decision=Decision.OUTDATED_VERSION,
                reason=f"reported {parsed.zed_version} vs stable {current_stable}",
                marker=MARKER_OUTDATED_VERSION,
                comment_body=_render_outdated_comment(parsed, current_stable),
                label_to_add=LABEL_NEEDS_INFO,
                parsed=parsed,
            )

        # Complete + current → clear label if we previously labeled.
        if issue_has_label and MARKER_MISSING_INFO in existing_markers:
            return DecisionResult(
                decision=Decision.SILENT,
                reason="reporter-responded-with-valid-info",
                label_to_remove=LABEL_NEEDS_INFO,
                parsed=parsed,
            )
        if issue_has_label and MARKER_OUTDATED_VERSION in existing_markers:
            # Reporter responded; their new version is current. Clear.
            return DecisionResult(
                decision=Decision.SILENT,
                reason="reporter-now-on-current-version",
                label_to_remove=LABEL_NEEDS_INFO,
                parsed=parsed,
            )
        return DecisionResult(
            decision=Decision.SILENT,
            reason="complete-and-current",
            parsed=parsed,
        )

    # Path 2: no parseable version. A reporter who offered a commit SHA
    # as their build id (source/local build) gets silence instead of a
    # prompt — there's no semver to compare, but they're almost always on
    # recent main, and "please share your version" would be a false
    # positive (team call, Lucas 2026-06, from review of #58699). If we
    # previously prompted and they answered with a commit ref, clear the
    # label — they've identified their build; a human takes it from here.
    if parsed.has_commit_ref:
        if issue_has_label and MARKER_MISSING_INFO in existing_markers:
            return DecisionResult(
                decision=Decision.SILENT,
                reason="reporter-provided-commit-ref",
                label_to_remove=LABEL_NEEDS_INFO,
                parsed=parsed,
            )
        return DecisionResult(
            decision=Decision.SILENT,
            reason="commit-ref-in-lieu-of-version",
            parsed=parsed,
        )
    if MARKER_MISSING_INFO in existing_markers:
        return DecisionResult(
            decision=Decision.SILENT,
            reason="already-prompted-missing-info",
            parsed=parsed,
        )
    return DecisionResult(
        decision=Decision.MISSING_INFO,
        reason="said-latest-confirm-exact"
        if parsed.says_latest
        else _missing_reason(parsed),
        marker=MARKER_MISSING_INFO,
        comment_body=_render_missing_info_comment(parsed, current_stable),
        label_to_add=LABEL_NEEDS_INFO,
        parsed=parsed,
    )


def _missing_reason(parsed: ParsedSpecs) -> str:
    missing = []
    if not parsed.has_zed:
        missing.append("zed-version")
    if REQUIRE_OS and not parsed.has_os:
        missing.append("os")
    return "missing:" + ",".join(missing) if missing else "missing:unknown"


# ---------------------------------------------------------------------------
# Comment templates
#
# Tone here matters a lot — these are public posts on user-submitted issues.
# Voice matches the dup bot's house style (team call, Lucas 2026-06):
#   - no greeting, no human mimicry — reads as the automated check it is
#   - hedged claims ("appears to be", "was detected") — the parser can
#     be wrong and the copy admits it
#   - an explicit "if this check is wrong → no action needed" branch —
#     turns a false positive from an accusation into a shrug
#   - the dup bot's standard footer: <sub>This is an automated check and
#     might be incorrect.</sub>
# Plus the runbook's rules: every ask is specific, names the exact one
# action (the copy-system-specs command, with keystrokes), and says
# where to put the answer.

# Shared footer — identical wording to the dup bot's, so the account
# posts with one consistent voice.
_AUTOMATED_FOOTER = """\
---
<sub>This is an automated check and might be incorrect.</sub>"""

# Version-only ask (REQUIRE_OS = False). We still recommend the
# system-specs command because it's the single easiest action for the
# reporter and it captures the version cleanly.
_MISSING_VERSION_COMMENT = (
    """\
No Zed version was detected in this issue. Knowing the exact version \
helps us check whether the bug is already fixed.

To add it: open the command palette (`cmd-shift-p` on macOS, \
`ctrl-shift-p` on Linux/Windows), run **`zed: copy system specs into \
clipboard`**, and paste the result into the issue (editing your \
original post works).

**If your version is already in the issue** and this check missed it: \
no action needed — a maintainer will review this shortly.

"""
    + _AUTOMATED_FOOTER
    + "\n\n{marker}"
)

# Version + OS ask (REQUIRE_OS = True). Preserved for when OS gating is
# re-enabled.
_MISSING_VERSION_AND_OS_COMMENT = (
    """\
No Zed version or OS was detected in this issue. Knowing both helps us \
check whether the bug is already fixed and route it correctly.

To add them: open the command palette (`cmd-shift-p` on macOS, \
`ctrl-shift-p` on Linux/Windows), run **`zed: copy system specs into \
clipboard`**, and paste the result into the issue (editing your \
original post works) — that captures version, channel, OS, memory, and \
GPU in one step.

**If your specs are already in the issue** and this check missed them: \
no action needed — a maintainer will review this shortly.

"""
    + _AUTOMATED_FOOTER
    + "\n\n{marker}"
)

# Variant of the missing-version ask for reporters who wrote "latest"
# where the version goes. Acknowledges their answer (the generic ask
# would read as if they'd said nothing) and names current stable so they
# can self-confirm — "latest" drifts with channel and auto-update timing.
# Used only when current stable is known; otherwise we fall back to the
# generic ask rather than claim a version we couldn't fetch.
_MISSING_VERSION_SAID_LATEST_COMMENT = (
    """\
This issue mentions being on the latest version, but no exact version \
was detected. Current stable is `{stable}` — "latest" can differ by \
channel and auto-update timing, so the exact build helps us check \
whether the bug is already fixed.

To confirm it: open the command palette (`cmd-shift-p` on macOS, \
`ctrl-shift-p` on Linux/Windows), run **`zed: copy system specs into \
clipboard`**, and paste the result into the issue (editing your \
original post works).

**If your exact version is already in the issue** and this check missed \
it: no action needed — a maintainer will review this shortly.

"""
    + _AUTOMATED_FOOTER
    + "\n\n{marker}"
)

_OUTDATED_COMMENT = (
    """\
The version in this issue appears to be `{reported}` — current stable \
is `{stable}`.

Many reported issues are already fixed in newer releases, so it's worth \
re-testing on {update_target}:

- **Still happening after updating:** reply with your new version \
(**`zed: copy system specs into clipboard`** is the easiest way).
- **Fixed by updating:** feel free to close this issue.
- **If this check misread your version:** no action needed — a \
maintainer will review this shortly.

"""
    + _AUTOMATED_FOOTER
    + "\n\n{marker}"
)


def _render_missing_info_comment(
    parsed: ParsedSpecs, current_stable: Optional[Version] = None
) -> str:
    if REQUIRE_OS:
        return _MISSING_VERSION_AND_OS_COMMENT.format(marker=MARKER_MISSING_INFO)
    if parsed.says_latest and current_stable is not None:
        return _MISSING_VERSION_SAID_LATEST_COMMENT.format(
            stable=current_stable, marker=MARKER_MISSING_INFO
        )
    return _MISSING_VERSION_COMMENT.format(marker=MARKER_MISSING_INFO)


def _render_outdated_comment(parsed: ParsedSpecs, stable: Version) -> str:
    # Tailor the "update to" suggestion to the reporter's channel. A
    # preview/nightly reporter who is behind probably wants the latest
    # build on their channel, but we also nudge toward stable since that's
    # what we triage against.
    channel = parsed.zed_channel
    if channel in ("preview", "nightly", "dev"):
        update_target = f"the latest {channel} build (or the latest stable)"
    else:
        update_target = "the latest stable"
    return _OUTDATED_COMMENT.format(
        reported=parsed.zed_version,
        stable=stable,
        update_target=update_target,
        marker=MARKER_OUTDATED_VERSION,
    )


# ---------------------------------------------------------------------------
# GitHub API


class GitHub:
    def __init__(self, token: str, repo: str = REPO_FULL) -> None:
        self.token = token
        self.repo = repo
        self.session = requests.Session()
        self.session.headers.update(
            {
                "Authorization": f"Bearer {token}",
                "Accept": "application/vnd.github+json",
                "X-GitHub-Api-Version": "2022-11-28",
            }
        )

    def _url(self, path: str) -> str:
        return f"{GITHUB_API}{path}"

    def get_issue(self, number: int) -> dict:
        r = self.session.get(self._url(f"/repos/{self.repo}/issues/{number}"))
        r.raise_for_status()
        return r.json()

    def list_comments(self, number: int) -> list[dict]:
        comments: list[dict] = []
        url: Optional[str] = self._url(
            f"/repos/{self.repo}/issues/{number}/comments?per_page=100"
        )
        while url:
            r = self.session.get(url)
            r.raise_for_status()
            comments.extend(r.json())
            url = r.links.get("next", {}).get("url")
        return comments

    def post_comment(self, number: int, body: str) -> dict:
        r = self.session.post(
            self._url(f"/repos/{self.repo}/issues/{number}/comments"),
            json={"body": body},
        )
        r.raise_for_status()
        return r.json()

    def add_label(self, number: int, label: str) -> None:
        r = self.session.post(
            self._url(f"/repos/{self.repo}/issues/{number}/labels"),
            json={"labels": [label]},
        )
        r.raise_for_status()

    def remove_label(self, number: int, label: str) -> None:
        # GH returns 404 if the label isn't on the issue; that's fine.
        r = self.session.delete(
            self._url(f"/repos/{self.repo}/issues/{number}/labels/{label}"),
        )
        if r.status_code not in (200, 404):
            r.raise_for_status()

    def fetch_current_stable(self) -> Optional[Version]:
        """Convenience wrapper around fetch_release_history() that returns
        just the highest-versioned stable release. Use this when you only
        need the threshold for an outdated check."""
        history = self.fetch_release_history()
        if not history:
            return None
        return max(v for v, _ in history)

    def fetch_release_history(self, pages: int = 1) -> list[tuple[Version, datetime]]:
        """List stable releases as `(version, published_at)` pairs, sorted
        by version desc.

        `pages` controls how many pages of 100 raw releases to pull. The
        live bot needs only `pages=1` (it just wants current stable). The
        replay tool passes a larger value to reach further back in time —
        most raw releases are prereleases (preview/nightly), so each page
        yields only a fraction of stable releases, and you need several
        pages to cover many months of history.

        Returns an empty list on failure (caller decides what to do)."""
        history: list[tuple[Version, datetime]] = []
        try:
            for page in range(1, pages + 1):
                r = self.session.get(
                    self._url(f"/repos/{self.repo}/releases?per_page=100&page={page}"),
                    timeout=15,
                )
                r.raise_for_status()
                batch = r.json()
                if not batch:
                    break  # ran out of releases
                for rel in batch:
                    if rel.get("prerelease") or rel.get("draft"):
                        continue
                    v = Version.parse(rel.get("tag_name") or "")
                    if v is None:
                        continue
                    published_at = rel.get("published_at") or rel.get("created_at")
                    if not published_at:
                        continue
                    # GitHub timestamps end with `Z` (UTC).
                    ts = published_at.replace("Z", "+00:00")
                    try:
                        dt = datetime.fromisoformat(ts)
                    except ValueError:
                        continue
                    history.append((v, dt))
            history.sort(key=lambda pair: pair[0], reverse=True)
            return history
        except Exception as e:  # noqa: BLE001
            log(f"fetch_release_history failed: {e!r}", "WARN")
            return history  # return whatever we got before the failure


# ---------------------------------------------------------------------------
# Marker scanning


def collect_markers(comments: list[dict]) -> set[str]:
    """Find existing bot markers across the issue's comment thread.
    Used for idempotency — if the bot already asked for missing info on
    this issue, don't ask again.

    Deliberately author-agnostic: the marker strings are unique to this
    bot, and filtering on a hardcoded bot login would be a silent failure
    mode (login drift → empty scans → re-post loop, since each post fires
    `issue_comment: created`). Scanning all comments can only err toward
    silence — the preferred failure direction."""
    markers: set[str] = set()
    for c in comments:
        body = c.get("body") or ""
        if MARKER_MISSING_INFO in body:
            markers.add(MARKER_MISSING_INFO)
        if MARKER_OUTDATED_VERSION in body:
            markers.add(MARKER_OUTDATED_VERSION)
    return markers


# ---------------------------------------------------------------------------
# Runner


@dataclass
class RunInputs:
    issue_number: int
    mode: str  # live | shadow | dry-run
    summary_file: Optional[Path] = None


def should_skip_issue_type(type_name: Optional[str]) -> bool:
    """True when the issue's type takes it out of the bot's scope.
    Typed and not Bug/Crash -> skip. Untyped (None/empty) -> in scope
    (new bug reports are often not typed yet). Mirrors the dup bot."""
    return bool(type_name) and type_name not in GATED_ISSUE_TYPES


def gather(
    gh: GitHub, inp: RunInputs
) -> tuple[ParsedSpecs, bool, set[str], Optional[Version], Optional[str]]:
    """Pull everything we need from GH: issue body, comments, label state,
    bot markers, and current stable. Returns (parsed, issue_has_label,
    existing_markers, current_stable, issue_type).

    Follow-up parsing is intentionally restricted to comments authored by
    the original reporter (the issue author). A third-party comment that
    happens to include a spec block does not count as the reporter answering
    the bot's prompt.
    """
    issue = gh.get_issue(inp.issue_number)
    issue_type = (issue.get("type") or {}).get("name")
    if should_skip_issue_type(issue_type):
        # Out-of-scope type: skip ALL the expensive fetches — the caller
        # short-circuits on issue_type before looking at the parse.
        return ParsedSpecs(), False, set(), None, issue_type
    body = issue.get("body") or ""
    labels = [lab["name"] for lab in issue.get("labels", []) if isinstance(lab, dict)]
    issue_has_label = LABEL_NEEDS_INFO in labels
    reporter_login = (issue.get("user") or {}).get("login")

    comments = gh.list_comments(inp.issue_number)
    follow_up_parses = [
        parse_body(c.get("body") or "")
        for c in comments
        if (c.get("user") or {}).get("login") == reporter_login
    ]
    body_parse = parse_body(body)
    parsed = merge_parsed(body_parse, *follow_up_parses)

    existing_markers = collect_markers(comments)
    current_stable = gh.fetch_current_stable()
    return parsed, issue_has_label, existing_markers, current_stable, issue_type


def apply(gh: GitHub, inp: RunInputs, result: DecisionResult) -> None:
    """Execute the decision in the configured mode."""
    if inp.mode == "dry-run":
        print(json.dumps(result.to_dict(), indent=2))
        return

    if inp.mode == "shadow":
        _write_shadow_summary(inp, result)
        return

    # Live mode.
    if result.comment_body:
        log(f"Posting comment to issue #{inp.issue_number}: {result.reason}")
        gh.post_comment(inp.issue_number, result.comment_body)
    if result.label_to_add:
        log(f"Adding label '{result.label_to_add}' to issue #{inp.issue_number}")
        gh.add_label(inp.issue_number, result.label_to_add)
    if result.label_to_remove:
        log(f"Removing label '{result.label_to_remove}' from issue #{inp.issue_number}")
        gh.remove_label(inp.issue_number, result.label_to_remove)


def _write_shadow_summary(inp: RunInputs, result: DecisionResult) -> None:
    """In shadow mode, append a structured record to the workflow's job
    summary so a human can review what the bot would have done."""
    target = inp.summary_file
    if target is None:
        env_path = os.environ.get("GITHUB_STEP_SUMMARY")
        target = Path(env_path) if env_path else None
    lines: list[str] = []
    lines.append(f"## Intake validation — shadow run (#{inp.issue_number})")
    lines.append("")
    lines.append(f"**Decision:** `{result.decision.value}`  ")
    lines.append(f"**Reason:** {result.reason}  ")
    if result.parsed is not None:
        lines.append(
            f"**Parsed:** `{json.dumps(result.parsed.to_dict(), sort_keys=True)}`"
        )
    if result.label_to_add:
        lines.append(f"- Would add label: `{result.label_to_add}`")
    if result.label_to_remove:
        lines.append(f"- Would remove label: `{result.label_to_remove}`")
    if result.comment_body:
        lines.append("")
        lines.append("<details><summary>Would have commented</summary>")
        lines.append("")
        lines.append("```")
        lines.append(result.comment_body)
        lines.append("```")
        lines.append("")
        lines.append("</details>")
    lines.append("")
    payload = "\n".join(lines) + "\n"
    if target:
        with target.open("a", encoding="utf-8") as f:
            f.write(payload)
    print(payload, file=sys.stdout)


def run(gh: GitHub, inp: RunInputs) -> DecisionResult:
    parsed, has_label, markers, stable, issue_type = gather(gh, inp)
    if should_skip_issue_type(issue_type):
        # Feature/Meta/Tracking/etc. — no spec section exists, so a
        # missing-info prompt would be a guaranteed false positive.
        # Mirrors the dup bot's should_skip(). Untyped issues pass through.
        result = DecisionResult(
            decision=Decision.SILENT,
            reason=f"skipped-issue-type:{issue_type}",
            parsed=parsed,
        )
    else:
        result = decide(parsed, stable, has_label, markers)
    log(
        f"issue={inp.issue_number} decision={result.decision.value} "
        f"reason={result.reason} parsed={parsed.to_dict()}"
    )
    apply(gh, inp, result)
    return result


# ---------------------------------------------------------------------------
# CLI


def _parse_args() -> argparse.Namespace:
    p = argparse.ArgumentParser(
        description="Intake validation bot for zed-industries/zed"
    )
    p.add_argument("--issue", type=int, required=True, help="Issue number to evaluate")
    p.add_argument(
        "--mode",
        choices=("live", "shadow", "dry-run"),
        default=os.environ.get("INTAKE_VALIDATION_MODE", "dry-run"),
        help="How to apply the decision (defaults to $INTAKE_VALIDATION_MODE or dry-run)",
    )
    p.add_argument(
        "--summary-file",
        type=Path,
        default=None,
        help="In shadow mode, append summary here (defaults to $GITHUB_STEP_SUMMARY)",
    )
    return p.parse_args()


def main() -> int:
    args = _parse_args()
    token = get_token()
    gh = GitHub(token=token)
    inp = RunInputs(
        issue_number=args.issue,
        mode=args.mode,
        summary_file=args.summary_file,
    )
    run(gh, inp)
    return 0


if __name__ == "__main__":
    sys.exit(main())
