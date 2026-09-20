"""Harbor installed-agent adapter for Zed's eval-cli binary."""

import json
import os
import shlex
import tomllib
from pathlib import Path

from harbor.agents.installed.base import BaseInstalledAgent, with_prompt_template
from harbor.environments.base import BaseEnvironment
from harbor.models.agent.context import AgentContext

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

# Leave eval-cli time to exit cleanly and flush logs before Harbor's outer
# asyncio.wait_for can kill the agent coroutine.
EVAL_CLI_FINALIZE_BUFFER_SEC = 45
EVAL_CLI_MIN_TIMEOUT_SEC = 60


class ZedAgent(BaseInstalledAgent):
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
        return await detect_workdir(
            environment,
            self.exec_as_agent,
            self._extra_env.get,
            "Could not detect a working directory in the container. "
            "Set EVAL_CLI_WORKDIR explicitly via --ae EVAL_CLI_WORKDIR=/path/to/repo",
        )

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

        # Tooling setup is best-effort; benchmark images vary enough that one
        # unsupported runtime should not fail the trial before eval-cli starts.
        await self._install_node(environment)
        await self._install_lsps(environment)
        await self._install_uv_and_ruff(environment)

        # Modal can mount a prebuilt binary inside each sandbox, avoiding uploads.
        container_path = self._extra_env.get("EVAL_CLI_CONTAINER_PATH")
        if container_path:
            await self.exec_as_root(
                environment,
                command=(
                    f"cp {shlex.quote(container_path)} /usr/local/bin/eval-cli && "
                    "chmod +x /usr/local/bin/eval-cli && "
                    "eval-cli --help"
                ),
            )
            return

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
            "Either pass binary_path=/path/to/target/release/eval-cli, "
            "set download_url=/EVAL_CLI_DOWNLOAD_URL, "
            "or set --ae EVAL_CLI_CONTAINER_PATH=/path/inside/container."
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
        populate_context_from_result(self.logs_dir, context, self.logger)

    def _get_api_env(self) -> dict[str, str]:
        env = provider_api_env(self.model_name)
        add_openai_compatible_provider_env(
            env, self._extra_env.get("ZED_OPENAI_COMPATIBLE_PROVIDERS")
        )
        add_anthropic_available_models_env(
            env, self._extra_env.get("ZED_ANTHROPIC_AVAILABLE_MODELS")
        )
        return env

    def _harbor_agent_budget_sec(self) -> float | None:
        """Recover Harbor's hidden outer timeout so eval-cli can exit first.

        Harbor 0.15.0 computes this on ``Trial`` but does not pass it to custom
        installed agents. Replaying the calculation host-side lets us preserve
        trajectories instead of losing them to Harbor's coroutine kill.
        """
        config_path = Path(self.logs_dir).parent / "config.json"
        try:
            trial_config = json.loads(config_path.read_text())
        except (OSError, ValueError) as exc:
            print(
                f"[zed-eval] agent budget: could not read {config_path}: {exc!r}",
                flush=True,
            )
            return None

        agent_cfg = trial_config.get("agent") or {}
        base_sec = agent_cfg.get("override_timeout_sec")
        if base_sec is None:
            base_sec = self._task_declared_agent_timeout_sec(
                trial_config.get("task") or {}
            )
        if base_sec is None:
            return None

        max_sec = agent_cfg.get("max_timeout_sec")
        if max_sec is not None:
            base_sec = min(base_sec, max_sec)
        multiplier = trial_config.get("agent_timeout_multiplier")
        if multiplier is None:
            multiplier = trial_config.get("timeout_multiplier", 1.0)
        return float(base_sec) * float(multiplier)

    def _task_declared_agent_timeout_sec(self, task_cfg: dict) -> float | None:
        task_dir = self._resolve_task_dir(task_cfg)
        if task_dir is None:
            return None
        task_toml = task_dir / "task.toml"
        try:
            data = tomllib.loads(task_toml.read_text())
        except (OSError, ValueError) as exc:
            print(
                f"[zed-eval] agent budget: could not parse {task_toml}: {exc!r}",
                flush=True,
            )
            return None
        timeout_sec = (data.get("agent") or {}).get("timeout_sec")
        return float(timeout_sec) if timeout_sec is not None else None

    def _resolve_task_dir(self, task_cfg: dict) -> Path | None:
        path = task_cfg.get("path")
        if path:
            return Path(path).expanduser().resolve()

        # config.json omits Harbor's package content hash; use the newest cached
        # digest that contains a task.toml.
        name = task_cfg.get("name")
        if name and "/" in name:
            try:
                from harbor.constants import PACKAGE_CACHE_DIR
            except ImportError as exc:
                print(
                    f"[zed-eval] agent budget: harbor PACKAGE_CACHE_DIR "
                    f"unavailable: {exc!r}",
                    flush=True,
                )
                return None
            download_dir = task_cfg.get("download_dir")
            base_dir = Path(download_dir) if download_dir else PACKAGE_CACHE_DIR
            org, package = name.split("/", 1)
            package_root = base_dir / org / package
            candidates = [
                digest_dir
                for digest_dir in package_root.glob("*")
                if (digest_dir / "task.toml").is_file()
            ]
            if not candidates:
                return None
            return max(candidates, key=lambda p: p.stat().st_mtime)

        return None

    @with_prompt_template
    async def run(
        self, instruction: str, environment: BaseEnvironment, context: AgentContext
    ) -> None:
        escaped_instruction = shlex.quote(instruction)
        env = self._get_api_env()

        add_zed_eval_env(
            env, self._extra_env, exclude={"ZED_EVAL_INSTRUCTION_SUFFIX_FILE"}
        )

        workdir = await self._detect_workdir(environment)

        parts = [
            "eval-cli",
            f"--workdir {shlex.quote(workdir)}",
            "--output-dir /logs/agent",
        ]

        if self.model_name:
            parts.append(f"--model {shlex.quote(self.model_name)}")

        instruction_suffix_file = self._extra_env.get(
            "ZED_EVAL_INSTRUCTION_SUFFIX_FILE"
        )
        if instruction_suffix_file:
            parts.append(
                f"--instruction-suffix-file {shlex.quote(instruction_suffix_file)}"
            )

        # Prefer eval-cli's own timeout over Harbor's coroutine kill so logs and
        # partial answers are still available for delivery.
        configured_timeout_raw = self._extra_env.get("EVAL_CLI_TIMEOUT")
        configured_timeout: int | None
        if configured_timeout_raw:
            try:
                configured_timeout = int(configured_timeout_raw)
            except ValueError:
                print(
                    "[zed-eval] ignoring non-integer EVAL_CLI_TIMEOUT="
                    f"{configured_timeout_raw!r}",
                    flush=True,
                )
                configured_timeout = None
        else:
            configured_timeout = None

        budget_sec = self._harbor_agent_budget_sec()
        timeout_arg: int | None = None
        if budget_sec is not None:
            ceiling = (
                configured_timeout
                if configured_timeout is not None
                else int(budget_sec)
            )
            timeout_arg = min(ceiling, int(budget_sec - EVAL_CLI_FINALIZE_BUFFER_SEC))
            timeout_arg = max(timeout_arg, EVAL_CLI_MIN_TIMEOUT_SEC)
            print(
                f"[zed-eval] eval-cli --timeout {timeout_arg}s (harbor agent "
                f"budget {int(budget_sec)}s - {EVAL_CLI_FINALIZE_BUFFER_SEC}s "
                f"finalize buffer; EVAL_CLI_TIMEOUT="
                f"{configured_timeout if configured_timeout is not None else 'unset'})",
                flush=True,
            )
        elif configured_timeout is not None:
            timeout_arg = configured_timeout
            print(
                f"[zed-eval] eval-cli --timeout {timeout_arg}s (harbor agent "
                "budget unavailable; using EVAL_CLI_TIMEOUT)",
                flush=True,
            )

        if timeout_arg is not None:
            parts.append(f"--timeout {timeout_arg}")

        staff = self._extra_env.get("EVAL_CLI_STAFF")
        if staff and staff.lower() == "false":
            parts.append("--no-staff")

        reasoning_effort = self._extra_env.get("EVAL_CLI_REASONING_EFFORT")
        if reasoning_effort:
            parts.append(f"--reasoning-effort {shlex.quote(reasoning_effort)}")

        enable_thinking = self._extra_env.get("EVAL_CLI_ENABLE_THINKING")
        if enable_thinking:
            if enable_thinking.lower() == "true":
                parts.append("--thinking true")
            elif enable_thinking.lower() == "false":
                parts.append("--thinking false")

        parts.append(f"--instruction {escaped_instruction}")

        # Exit 2 is eval-cli's timeout, not a crash; keep the trajectory judgeable.
        await self.exec_as_agent(
            environment,
            command=eval_cli_with_log_command(
                parts,
                "/logs/agent/eval-cli.txt",
                timeout_message=(
                    "[zed-eval] eval-cli timed out (exit 2); continuing to "
                    "delivery/verification"
                ),
                line_buffered=True,
            ),
            env=env,
        )

        # Modal-style remote runs can differ in agent-log collection behavior.
        await self.exec_as_agent(
            environment,
            command=(
                "mkdir -p /logs/artifacts && "
                "for f in result.json thread.json thread.md eval-cli.txt; do "
                '  cp "/logs/agent/$f" /logs/artifacts/ 2>/dev/null || true; '
                "done; true"
            ),
        )

        # Some harnesses mount an initialized repo before creating the first commit.
        await self.exec_as_agent(
            environment,
            command=patch_command("/logs/agent"),
            cwd=workdir,
        )
