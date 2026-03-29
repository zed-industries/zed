"""Harbor agent wrapper for Zed's eval-cli binary.

Usage:
    # Build eval-cli locally first:
    cargo build --release -p eval_cli

    # Run via Harbor with a local binary:
    harbor run -d "dataset@version" \
        --agent-import-path zed_eval.agent:ZedAgent \
        --ae binary_path=/path/to/target/release/eval-cli \
        --agent-model anthropic/claude-sonnet-4-6-latest

    # Or with a download URL (for CI):
    harbor run -d "dataset@version" \
        --agent-import-path zed_eval.agent:ZedAgent \
        --ae download_url=https://example.com/eval-cli \
        --agent-model anthropic/claude-sonnet-4-6-latest
"""

import json
import os
import shlex
from pathlib import Path

from harbor.agents.installed.base import BaseInstalledAgent, with_prompt_template
from harbor.environments.base import BaseEnvironment
from harbor.models.agent.context import AgentContext


class ZedAgent(BaseInstalledAgent):
    """Runs Zed's headless AI agent (eval-cli) to solve tasks.

    The eval-cli binary boots a headless GPUI application and uses the same
    NativeAgent + AcpThread pipeline as the production Zed editor, driving
    the full agentic loop (tool calls, subagents, retries) without a GUI.
    """

    def __init__(
        self,
        logs_dir: Path,
        binary_path: str | None = None,
        download_url: str | None = None,
        *args,
        **kwargs,
    ):
        super().__init__(logs_dir, *args, **kwargs)
        self._binary_path = binary_path
        self._download_url = download_url or os.environ.get("EVAL_CLI_DOWNLOAD_URL")

    @staticmethod
    def name() -> str:
        return "zed"

    async def _detect_workdir(self, environment: BaseEnvironment) -> str:
        """Detect the repo working directory inside the container.

        Checks, in order:
          1. Explicit ``EVAL_CLI_WORKDIR`` extra-env override
          2. ``/app``      (SWE-bench Pro)
          3. ``/testbed``  (SWE-bench Verified)
          4. ``/repo``
          5. First git repo found under ``/`` (max depth 3)
        """
        override = self._extra_env.get("EVAL_CLI_WORKDIR")
        if override:
            return override

        result = await self.exec_as_agent(
            environment,
            command=(
                "for d in /app /testbed /repo; do "
                '  if [ -d "$d/.git" ]; then echo "$d"; exit 0; fi; '
                "done; "
                "find / -maxdepth 3 -name .git -type d 2>/dev/null "
                '| head -1 | sed "s|/.git$||"'
            ),
        )
        workdir = result.stdout.strip()
        if not workdir:
            raise RuntimeError(
                "Could not find a git repository in the container. "
                "Set EVAL_CLI_WORKDIR explicitly via --ae EVAL_CLI_WORKDIR=/path/to/repo"
            )
        return workdir

    async def install(self, environment: BaseEnvironment) -> None:
        await self.exec_as_root(
            environment,
            command=(
                "apt-get update && "
                "apt-get install -y --no-install-recommends "
                "ca-certificates "
                "curl "
                "git"
            ),
            env={"DEBIAN_FRONTEND": "noninteractive"},
        )

        await self.exec_as_root(
            environment,
            command=(
                "curl -fsSL https://deb.nodesource.com/setup_22.x | bash - && "
                "apt-get install -y --no-install-recommends nodejs"
            ),
            env={"DEBIAN_FRONTEND": "noninteractive"},
        )

        # Pre-install default LSPs so Zed doesn't have to download them at
        # runtime.  Each gets its own subdirectory under $ZED_DATA_DIR/languages.
        await self.exec_as_agent(
            environment,
            command=(
                "set -euo pipefail; "
                'ZED_DATA_DIR="${XDG_DATA_HOME:-$HOME/.local/share}/zed"; '
                # basedpyright (Python - default type checker)
                'BASEDPYRIGHT_DIR="$ZED_DATA_DIR/languages/basedpyright"; '
                'mkdir -p "$BASEDPYRIGHT_DIR"; '
                'npm install --prefix "$BASEDPYRIGHT_DIR" --save-exact basedpyright; '
                # typescript-language-server (TypeScript/JS - default LSP)
                'TSSERVER_DIR="$ZED_DATA_DIR/languages/typescript-language-server"; '
                'mkdir -p "$TSSERVER_DIR"; '
                'npm install --prefix "$TSSERVER_DIR" --save-exact typescript typescript-language-server; '
                # vtsls (VS Code TypeScript language features)
                'VTSLS_DIR="$ZED_DATA_DIR/languages/vtsls"; '
                'mkdir -p "$VTSLS_DIR"; '
                'npm install --prefix "$VTSLS_DIR" --save-exact @vtsls/language-server typescript; '
                # tailwindcss-language-server
                'TAILWIND_DIR="$ZED_DATA_DIR/languages/tailwindcss-language-server"; '
                'mkdir -p "$TAILWIND_DIR"; '
                'npm install --prefix "$TAILWIND_DIR" --save-exact @tailwindcss/language-server'
            ),
        )

        # eslint LSP (downloaded from zed-industries/vscode-eslint GitHub release,
        # then compiled — this mirrors what Zed does at runtime).
        await self.exec_as_agent(
            environment,
            command=(
                "set -euo pipefail; "
                'ZED_DATA_DIR="${XDG_DATA_HOME:-$HOME/.local/share}/zed"; '
                'ESLINT_DIR="$ZED_DATA_DIR/languages/eslint/vscode-eslint-2.4.4"; '
                'mkdir -p "$ESLINT_DIR"; '
                'curl -fsSL "https://github.com/zed-industries/vscode-eslint/archive/refs/tags/release/2.4.4.tar.gz" '
                '| tar -xz -C "$ESLINT_DIR"; '
                'mv "$ESLINT_DIR"/vscode-eslint-release-2.4.4 "$ESLINT_DIR/vscode-eslint"; '
                'cd "$ESLINT_DIR/vscode-eslint" && npm install && npm run compile'
            ),
        )

        # gopls (Go - default LSP).  Only install when Go is present in the
        # container (i.e. Go-related SWE-bench tasks).
        await self.exec_as_agent(
            environment,
            command=(
                "if command -v go >/dev/null 2>&1; then "
                "go install golang.org/x/tools/gopls@latest; "
                "fi"
            ),
        )

        await self.exec_as_agent(
            environment,
            command=(
                "curl -LsSf https://astral.sh/uv/install.sh | sh && "
                '. "$HOME/.local/bin/env"'
            ),
        )

        agent_home_result = await self.exec_as_agent(
            environment,
            command='printf %s "$HOME"',
        )
        agent_home = agent_home_result.stdout.strip()
        if not agent_home:
            raise RuntimeError("Could not determine agent home directory")

        await self.exec_as_root(
            environment,
            command=(
                f"ln -sf {shlex.quote(agent_home + '/.local/bin/uv')} /usr/local/bin/uv && "
                f"ln -sf {shlex.quote(agent_home + '/.local/bin/uvx')} /usr/local/bin/uvx"
            ),
        )

        # Install a modern ruff so `ruff server` works without --preview.
        # This also makes it available as a CLI tool for the agent.
        await self.exec_as_agent(
            environment,
            command=('export PATH="$HOME/.local/bin:$PATH" && uv tool install ruff'),
        )

        if self._binary_path:
            binary = Path(self._binary_path)
            if not binary.exists():
                raise FileNotFoundError(
                    f"eval-cli binary not found at {binary}. "
                    "Build it with: cargo build --release -p eval_cli"
                )
            await environment.upload_file(
                source_path=binary,
                target_path="/usr/local/bin/eval-cli",
            )
            await self.exec_as_root(
                environment,
                command="chmod +x /usr/local/bin/eval-cli && eval-cli --help",
            )
            return

        if self._download_url:
            await self.exec_as_root(
                environment,
                command=(
                    f"curl -fsSL {shlex.quote(self._download_url)} "
                    "-o /usr/local/bin/eval-cli && "
                    "chmod +x /usr/local/bin/eval-cli && "
                    "eval-cli --help"
                ),
            )
            return

        raise ValueError(
            "No eval-cli binary provided. "
            "Either pass binary_path=/path/to/target/release/eval-cli "
            "or set download_url=/EVAL_CLI_DOWNLOAD_URL."
        )

    def populate_context_post_run(self, context: AgentContext) -> None:
        result_data = None
        for json_file in self.logs_dir.rglob("result.json"):
            try:
                result_data = json.loads(json_file.read_text())
                break
            except (json.JSONDecodeError, OSError):
                continue

        if result_data is None:
            self.logger.warning("Could not find or parse result.json from eval-cli")
            return

        if result_data.get("input_tokens") is not None:
            context.n_input_tokens = result_data["input_tokens"]
        if result_data.get("output_tokens") is not None:
            context.n_output_tokens = result_data["output_tokens"]
        if result_data.get("cache_read_input_tokens") is not None:
            context.n_cache_tokens = result_data["cache_read_input_tokens"]

        context.metadata = {
            "status": result_data.get("status"),
            "duration_secs": result_data.get("duration_secs"),
            "model": result_data.get("model"),
        }

    def _get_api_env(self) -> dict[str, str]:
        env: dict[str, str] = {}
        if not self.model_name or "/" not in self.model_name:
            return env

        provider = self.model_name.split("/", 1)[0]
        provider_env_map = {
            "anthropic": "ANTHROPIC_API_KEY",
            "openai": "OPENAI_API_KEY",
            "google": "GEMINI_API_KEY",
            "gemini": "GEMINI_API_KEY",
            "deepseek": "DEEPSEEK_API_KEY",
            "mistral": "MISTRAL_API_KEY",
        }

        env_var = provider_env_map.get(provider)
        if env_var:
            api_key = os.environ.get(env_var, "")
            if api_key:
                env[env_var] = api_key

        return env

    @with_prompt_template
    async def run(
        self, instruction: str, environment: BaseEnvironment, context: AgentContext
    ) -> None:
        escaped_instruction = shlex.quote(instruction)
        env = self._get_api_env()

        workdir = await self._detect_workdir(environment)

        parts = [
            "eval-cli",
            f"--workdir {shlex.quote(workdir)}",
            "--output-dir /logs/agent",
        ]

        if self.model_name:
            parts.append(f"--model {shlex.quote(self.model_name)}")

        timeout = self._extra_env.get("EVAL_CLI_TIMEOUT")
        if timeout:
            parts.append(f"--timeout {shlex.quote(timeout)}")

        staff = self._extra_env.get("EVAL_CLI_STAFF")
        if staff and staff.lower() == "false":
            parts.append("--no-staff")

        reasoning_effort = self._extra_env.get("EVAL_CLI_REASONING_EFFORT")
        if reasoning_effort:
            parts.append(f"--reasoning-effort {shlex.quote(reasoning_effort)}")

        enable_thinking = self._extra_env.get("EVAL_CLI_ENABLE_THINKING")
        if enable_thinking:
            if enable_thinking.lower() == "true":
                parts.append("--enable-thinking")
            elif enable_thinking.lower() == "false":
                parts.append("--disable-thinking")

        parts.append(f"--instruction {escaped_instruction}")

        await self.exec_as_agent(
            environment,
            command=(
                " ".join(parts) + " 2>&1 | stdbuf -oL tee /logs/agent/eval-cli.txt"
            ),
            env=env,
        )

        await self.exec_as_agent(
            environment,
            command=(
                "git add -A && "
                "git diff --cached HEAD > /logs/agent/patch.diff && "
                'echo "Patch size: $(wc -c < /logs/agent/patch.diff) bytes"'
            ),
            cwd=workdir,
        )
