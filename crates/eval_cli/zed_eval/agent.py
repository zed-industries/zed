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
        # Detect the package manager and install base dependencies.
        # Supports Debian/Ubuntu (apt-get), Alpine (apk), and
        # Fedora/RHEL/CentOS (dnf/yum).
        await self.exec_as_root(
            environment,
            command=(
                "if command -v apt-get >/dev/null 2>&1; then "
                "  apt-get update && "
                "  apt-get install -y --no-install-recommends ca-certificates curl git; "
                "elif command -v apk >/dev/null 2>&1; then "
                "  apk add --no-cache ca-certificates curl git bash coreutils gcompat libstdc++; "
                "elif command -v dnf >/dev/null 2>&1; then "
                "  dnf install -y ca-certificates curl git; "
                "elif command -v yum >/dev/null 2>&1; then "
                "  yum install -y ca-certificates curl git; "
                "else "
                "  echo 'WARNING: No supported package manager found (apt-get, apk, dnf, yum)' >&2; "
                "fi"
            ),
            env={"DEBIAN_FRONTEND": "noninteractive"},
        )

        # ── Non-essential tooling ─────────────────────────────────────
        # Everything below here (Node.js, LSPs, uv/ruff) is nice-to-have.
        # If any step fails (e.g. musl incompatibility, network issues),
        # log a warning and continue — the agent can still work without
        # pre-installed language servers.

        await self._install_node(environment)
        await self._install_lsps(environment)
        await self._install_uv_and_ruff(environment)

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

    async def _install_node(self, environment: BaseEnvironment) -> None:
        """Install Node.js from official binary tarballs.

        Uses the musl build on Alpine and the glibc build elsewhere.
        Skips if node is already on PATH.
        """
        try:
            await self.exec_as_root(
                environment,
                command=(
                    "if command -v node >/dev/null 2>&1; then "
                    '  echo "Node.js already available: $(node --version)"; '
                    "else "
                    "  NODE_VER=v22.14.0; "
                    "  ARCH=$(uname -m); "
                    '  case "$ARCH" in '
                    "    x86_64)  NODE_ARCH=x64  ;; "
                    "    aarch64) NODE_ARCH=arm64 ;; "
                    '    *)       echo "WARNING: unsupported arch $ARCH for Node.js" >&2; exit 0 ;; '
                    "  esac; "
                    "  if ldd /bin/sh 2>&1 | grep -qi musl; then "
                    '    NODE_URL="https://unofficial-builds.nodejs.org/download/release/${NODE_VER}/node-${NODE_VER}-linux-${NODE_ARCH}-musl.tar.gz"; '
                    "  else "
                    '    NODE_URL="https://nodejs.org/dist/${NODE_VER}/node-${NODE_VER}-linux-${NODE_ARCH}.tar.gz"; '
                    "  fi; "
                    '  echo "Downloading Node.js from $NODE_URL"; '
                    '  curl -fsSL "$NODE_URL" | tar -xz -C /usr/local --strip-components=1; '
                    '  echo "Installed Node.js $(node --version)"; '
                    "fi"
                ),
            )
        except Exception as exc:
            self.logger.warning("Node.js installation failed (non-fatal): %s", exc)

    async def _install_lsps(self, environment: BaseEnvironment) -> None:
        """Pre-install language servers so Zed doesn't download them at runtime.

        Each LSP is installed independently so one failure doesn't block the rest.
        """
        # npm-based LSPs — skip all if npm is not available.
        try:
            await self.exec_as_agent(
                environment,
                command="command -v npm >/dev/null 2>&1",
            )
        except Exception:
            self.logger.warning("npm not available — skipping npm-based LSP installs")
            return

        lsp_installs = [
            (
                "basedpyright",
                'DIR="$ZED_DATA_DIR/languages/basedpyright"; '
                'mkdir -p "$DIR" && npm install --prefix "$DIR" --save-exact basedpyright',
            ),
            (
                "typescript-language-server",
                'DIR="$ZED_DATA_DIR/languages/typescript-language-server"; '
                'mkdir -p "$DIR" && npm install --prefix "$DIR" --save-exact typescript typescript-language-server',
            ),
            (
                "vtsls",
                'DIR="$ZED_DATA_DIR/languages/vtsls"; '
                'mkdir -p "$DIR" && npm install --prefix "$DIR" --save-exact @vtsls/language-server typescript',
            ),
            (
                "tailwindcss-language-server",
                'DIR="$ZED_DATA_DIR/languages/tailwindcss-language-server"; '
                'mkdir -p "$DIR" && npm install --prefix "$DIR" --save-exact @tailwindcss/language-server',
            ),
        ]

        for name, cmd in lsp_installs:
            try:
                await self.exec_as_agent(
                    environment,
                    command=(
                        'ZED_DATA_DIR="${XDG_DATA_HOME:-$HOME/.local/share}/zed"; '
                        + cmd
                    ),
                )
            except Exception as exc:
                self.logger.warning(
                    "LSP install '%s' failed (non-fatal): %s", name, exc
                )

        # eslint — downloaded from GitHub and compiled separately.
        try:
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
        except Exception as exc:
            self.logger.warning("eslint LSP install failed (non-fatal): %s", exc)

        # gopls — only when Go is present.  Guarded by a 120s timeout so slow
        # compilation can never eat the full setup budget.
        gopls_script = (
            "if command -v go >/dev/null 2>&1; then "
            "if go install golang.org/x/tools/gopls@latest 2>/dev/null; then "
            "echo 'Installed gopls@latest'; "
            "else "
            '  MY_GO=$(go env GOVERSION | sed "s/^go//"); '
            "  for v in $(curl -fsSL "
            "https://proxy.golang.org/golang.org/x/tools/gopls/@v/list 2>/dev/null"
            " | grep -E '^v[0-9]+\\.[0-9]+\\.[0-9]+$' | sort -rV | head -5); do "
            "    NEED=$(curl -fsSL "
            '"https://proxy.golang.org/golang.org/x/tools/gopls/@v/${v}.mod"'
            " 2>/dev/null | awk '/^go /{print $2; exit}'); "
            '    if [ -n "$NEED" ] '
            '    && [ "$(printf \'%s\\n%s\\n\' "$NEED" "$MY_GO" '
            '         | sort -V | head -1)" = "$NEED" ]; then '
            '      echo "Installing gopls $v (compatible with Go $MY_GO)"; '
            '      go install "golang.org/x/tools/gopls@$v" && break; '
            "    fi; "
            "  done; "
            "fi; "
            "fi"
        )
        try:
            await self.exec_as_agent(
                environment,
                command=(
                    "timeout 120 bash -c "
                    + shlex.quote(gopls_script)
                    + " || echo 'WARNING: gopls installation timed out or failed -- skipping'"
                ),
            )
        except Exception as exc:
            self.logger.warning("gopls install failed (non-fatal): %s", exc)

    async def _install_uv_and_ruff(self, environment: BaseEnvironment) -> None:
        """Install uv and ruff for Python tooling."""
        try:
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
                self.logger.warning(
                    "Could not determine agent home directory — skipping uv symlinks"
                )
                return

            await self.exec_as_root(
                environment,
                command=(
                    f"ln -sf {shlex.quote(agent_home + '/.local/bin/uv')} /usr/local/bin/uv && "
                    f"ln -sf {shlex.quote(agent_home + '/.local/bin/uvx')} /usr/local/bin/uvx"
                ),
            )

            await self.exec_as_agent(
                environment,
                command='export PATH="$HOME/.local/bin:$PATH" && uv tool install ruff',
            )
        except Exception as exc:
            self.logger.warning("uv/ruff installation failed (non-fatal): %s", exc)

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
                " ".join(parts) + " 2>&1 | if command -v stdbuf >/dev/null 2>&1;"
                " then stdbuf -oL tee /logs/agent/eval-cli.txt;"
                " else tee /logs/agent/eval-cli.txt; fi"
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
