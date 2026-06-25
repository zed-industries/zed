"""Pier-native agent wrapper for Zed's eval-cli binary.

Pier (a Harbor fork) is the harness for DeepSWE, whose tasks run air-gapped
(`allow_internet = false`). Pier's installed-agent interface differs from
Harbor's — it requires `install_spec()` and, crucially, a `network_allowlist()`
the egress proxy honors so the agent can still reach the model API. Harbor's
`BaseInstalledAgent` has neither, so this is a separate, Pier-specific class
(the sanctioned cross-framework exception): everything else in the project has a
single canonical implementation, but two genuinely different harness SDKs need
two thin agent shells.

Air-gap notes:
  - The eval-cli binary is a static musl executable copied from the mounted Modal
    volume (`/data`, local — not network), so installing it needs no egress.
  - Network installs (Node, LSPs) are intentionally skipped: there's no egress to
    fetch them, and eval-cli's core tools (read/edit/terminal/...) don't need
    them. Only the model API host is allowlisted.
"""

from __future__ import annotations

import json
import shlex
from urllib.parse import urlparse

from pier.agents.installed.base import BaseInstalledAgent, with_prompt_template
from pier.environments.base import BaseEnvironment
from pier.models.agent.context import AgentContext
from pier.models.agent.install import AgentInstallSpec, InstallStep
from pier.models.agent.network import NetworkAllowlist
from pier.models.trial.paths import EnvironmentPaths

from .agent_common import (
    add_anthropic_available_models_env,
    add_openai_compatible_provider_env,
    add_zed_eval_env,
    detect_workdir,
    eval_cli_with_log_command,
    patch_command,
    populate_context_from_result,
    provider_api_env,
)

# Default model-provider API hosts the agent must reach even on air-gapped tasks.
# Mirrors benchmarks.AGENT_API_HOSTS; duplicated here so this module has no
# dependency on the orchestration package when loaded inside a sandbox.
DEFAULT_PROVIDER_DOMAINS: dict[str, list[str]] = {
    "anthropic": ["api.anthropic.com"],
    "openai": ["api.openai.com"],
    "google": [".googleapis.com"],
    "gemini": [".googleapis.com"],
    "deepseek": ["api.deepseek.com"],
    "baseten": ["inference.baseten.co"],
}
# Base-URL env vars Pier-style allowlist resolution should also honor.
BASE_URL_ENV_VARS = (
    "ANTHROPIC_BASE_URL",
    "OPENAI_BASE_URL",
    "OPENAI_API_BASE",
    "GEMINI_API_BASE",
)


class ZedPierAgent(BaseInstalledAgent):
    """Runs Zed's headless eval-cli binary under Pier."""

    SUPPORTS_ATIF: bool = False

    @staticmethod
    def name() -> str:
        return "zed"

    def _container_binary_path(self) -> str:
        path = self._get_env("EVAL_CLI_CONTAINER_PATH")
        if not path:
            raise ValueError(
                "ZedPierAgent requires EVAL_CLI_CONTAINER_PATH (the eval-cli binary "
                "on the mounted volume, e.g. /data/builds/<id>/eval-cli)"
            )
        return path

    def install_spec(self) -> AgentInstallSpec:
        # The binary lives on the runtime-mounted volume, which isn't available at
        # image-build time, so the real placement happens in run() via
        # exec_as_root. install_spec only needs a non-empty, no-egress step.
        return AgentInstallSpec(
            agent_name=self.name(),
            version=self._version,
            steps=[
                InstallStep(
                    user="root",
                    run="true  # eval-cli is copied from the volume at run time",
                )
            ],
        )

    def network_allowlist(self) -> NetworkAllowlist:
        """Hosts the egress proxy must allow so eval-cli can reach the model API
        on air-gapped DeepSWE tasks."""
        domains: list[str] = []

        provider = None
        if self.model_name and "/" in self.model_name:
            provider = self.model_name.split("/", 1)[0]
        domains.extend(DEFAULT_PROVIDER_DOMAINS.get(provider or "", []))

        # Any explicitly configured base URL (e.g. an OpenAI-compatible gateway
        # such as Baseten) wins/adds alongside the provider default.
        for env_var in BASE_URL_ENV_VARS:
            value = self._get_env(env_var)
            if value:
                host = urlparse(value).hostname
                if host:
                    domains.append(host)

        # A Baseten (or other OpenAI-compatible) provider is wired via this JSON;
        # allow its api_url host too.
        providers_json = self._get_env("ZED_OPENAI_COMPATIBLE_PROVIDERS")
        if providers_json:
            try:
                parsed = json.loads(providers_json)
            except json.JSONDecodeError:
                parsed = {}
            for provider_config in (parsed or {}).values():
                api_url = (
                    provider_config.get("api_url")
                    if isinstance(provider_config, dict)
                    else None
                )
                host = urlparse(api_url).hostname if api_url else None
                if host:
                    domains.append(host)

        if not domains:
            # Never return an empty allowlist: without the model host the agent
            # can do nothing on an air-gapped task. Default to Anthropic.
            domains = ["api.anthropic.com"]
        return NetworkAllowlist(domains=domains)

    def _api_env(self) -> dict[str, str]:
        env = provider_api_env(self.model_name, self._get_env)
        add_openai_compatible_provider_env(
            env, self._get_env("ZED_OPENAI_COMPATIBLE_PROVIDERS")
        )
        add_anthropic_available_models_env(
            env, self._get_env("ZED_ANTHROPIC_AVAILABLE_MODELS")
        )
        add_zed_eval_env(
            env, self._extra_env, exclude={"ZED_EVAL_INSTRUCTION_SUFFIX_FILE"}
        )
        return env

    async def _detect_workdir(self, environment: BaseEnvironment) -> str:
        return await detect_workdir(
            environment,
            self.exec_as_agent,
            self._get_env,
            "Could not detect a working directory; set EVAL_CLI_WORKDIR via --ae",
        )

    @with_prompt_template
    async def run(
        self, instruction: str, environment: BaseEnvironment, context: AgentContext
    ) -> None:
        # Place the static binary from the mounted volume (no egress needed).
        await self.exec_as_root(
            environment,
            command=(
                f"install -m755 {shlex.quote(self._container_binary_path())} "
                "/usr/local/bin/eval-cli && eval-cli --help >/dev/null"
            ),
        )

        workdir = await self._detect_workdir(environment)
        agent_dir = str(EnvironmentPaths.agent_dir)
        env = self.build_process_env(self._api_env())

        parts = [
            "eval-cli",
            f"--workdir {shlex.quote(workdir)}",
            f"--output-dir {shlex.quote(agent_dir)}",
        ]
        if self.model_name:
            parts.append(f"--model {shlex.quote(self.model_name)}")
        timeout = self._get_env("EVAL_CLI_TIMEOUT")
        if timeout:
            parts.append(f"--timeout {shlex.quote(timeout)}")
        instruction_suffix_file = self._get_env("ZED_EVAL_INSTRUCTION_SUFFIX_FILE")
        if instruction_suffix_file:
            parts.append(
                f"--instruction-suffix-file {shlex.quote(instruction_suffix_file)}"
            )
        staff = self._get_env("EVAL_CLI_STAFF")
        if staff and staff.lower() == "false":
            parts.append("--no-staff")
        reasoning_effort = self._get_env("EVAL_CLI_REASONING_EFFORT")
        if reasoning_effort:
            parts.append(f"--reasoning-effort {shlex.quote(reasoning_effort)}")
        enable_thinking = self._get_env("EVAL_CLI_ENABLE_THINKING")
        if enable_thinking:
            if enable_thinking.lower() == "true":
                parts.append("--thinking true")
            elif enable_thinking.lower() == "false":
                parts.append("--thinking false")
        parts.append(f"--instruction {shlex.quote(instruction)}")

        # Tolerate exit 2 (timeout): deliver whatever the agent produced.
        await self.exec_as_agent(
            environment,
            command=eval_cli_with_log_command(
                parts,
                agent_dir + "/eval-cli.txt",
                timeout_message=None,
            ),
            env=env,
        )

        # SWE-bench-style patch for the verifier (DeepSWE tasks are git repos).
        await self.exec_as_agent(
            environment,
            command=patch_command(agent_dir),
            cwd=workdir,
        )

    def populate_context_post_run(self, context: AgentContext) -> None:
        populate_context_from_result(self.logs_dir, context, self.logger)
