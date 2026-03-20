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

from harbor.agents.installed.base import BaseInstalledAgent, ExecInput
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

    @property
    def _install_agent_template_path(self) -> Path:
        return Path(__file__).parent / "install.sh.j2"

    async def setup(self, environment: BaseEnvironment) -> None:
        await environment.exec(command="mkdir -p /installed-agent")

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
            await environment.exec(command="chmod +x /usr/local/bin/eval-cli")

        await super().setup(environment)

    @property
    def _template_variables(self) -> dict[str, str]:
        variables = super()._template_variables
        if self._binary_path:
            variables["binary_uploaded"] = "true"
        if self._download_url:
            variables["download_url"] = self._download_url
        return variables

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

    def create_run_agent_commands(self, instruction: str) -> list[ExecInput]:
        escaped_instruction = shlex.quote(instruction)
        env = self._get_api_env()

        parts = ["eval-cli", "--workdir /testbed", "--output-dir /logs/agent"]

        if self.model_name:
            parts.append(f"--model {self.model_name}")

        timeout = self._extra_env.get("EVAL_CLI_TIMEOUT")
        if timeout:
            parts.append(f"--timeout {timeout}")

        parts.append(f"--instruction {escaped_instruction}")

        eval_cli_command = " ".join(parts) + " 2>&1 | stdbuf -oL tee /logs/agent/eval-cli.txt"

        patch_command = (
            "cd /testbed && "
            "git add -A && "
            "git diff --cached HEAD > /logs/agent/patch.diff && "
            "echo \"Patch size: $(wc -c < /logs/agent/patch.diff) bytes\""
        )

        return [
            ExecInput(command=eval_cli_command, env=env),
            ExecInput(command=patch_command),
        ]
