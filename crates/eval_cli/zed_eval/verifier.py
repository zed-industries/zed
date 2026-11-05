from __future__ import annotations

from harbor.models.verifier.result import VerifierResult
from harbor.utils.env import resolve_env_vars
from harbor.verifier.verifier import Verifier

from .judge_proxy import JUDGE_PROXY_ENSURE_SCRIPT, JUDGE_PROXY_SCRIPT


class ZedJudgeProxyVerifier(Verifier):
    async def verify(self) -> VerifierResult:
        await self._ensure_judge_proxy()
        return await super().verify()

    def _merged_verifier_env(self) -> dict[str, str]:
        return {
            **self.task.config.verifier.env,
            **(self.verifier_env or {}),
            **self.override_env,
        }

    async def _ensure_judge_proxy(self) -> None:
        merged_env = self._merged_verifier_env()
        if not merged_env.get("ZED_JUDGE_UPSTREAM"):
            return

        env = resolve_env_vars(merged_env)
        result = await self.environment.exec(
            command=(
                "set -e\n"
                "mkdir -p /usr/local/lib /usr/local/bin\n"
                "cat > /usr/local/lib/zed_judge_proxy.py <<'ZED_PROXY_EOF'\n"
                f"{JUDGE_PROXY_SCRIPT}"
                "ZED_PROXY_EOF\n"
                "cat > /usr/local/bin/ensure-judge-proxy <<'ZED_ENSURE_EOF'\n"
                f"{JUDGE_PROXY_ENSURE_SCRIPT}"
                "ZED_ENSURE_EOF\n"
                "chmod +x /usr/local/bin/ensure-judge-proxy\n"
                "ensure-judge-proxy\n"
                "python3 - <<'ZED_PROXY_CHECK_EOF'\n"
                "import os, socket\n"
                "port = int(os.environ.get('ZED_JUDGE_PROXY_PORT', '8089'))\n"
                "with socket.create_connection(('127.0.0.1', port), timeout=5):\n"
                "    pass\n"
                "ZED_PROXY_CHECK_EOF\n"
                "if [ -f /tmp/zed-judge-proxy.log ]; then "
                "tail -2 /tmp/zed-judge-proxy.log; fi\n"
            ),
            user="root",
            env=env,
        )
        if result.return_code != 0:
            output = "\n".join(
                part for part in (result.stdout, result.stderr) if part
            ).strip()
            detail = f": {output}" if output else ""
            raise RuntimeError(
                f"Failed to install or start judge proxy (exit {result.return_code}){detail}"
            )
