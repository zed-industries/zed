from __future__ import annotations

import argparse
import io
import os
import shlex
import subprocess
import tarfile
import tempfile
import unittest
from pathlib import Path
from unittest.mock import patch

from zed_eval import common
from zed_eval.agent_common import (
    add_anthropic_available_models_env,
    eval_cli_with_log_command,
)


class EvalCliWithLogCommandTests(unittest.TestCase):
    def run_logged_command(self, script: str, *, timeout_message: str | None = None):
        with tempfile.TemporaryDirectory() as temporary_directory:
            log_path = Path(temporary_directory) / "eval-cli.txt"
            command = eval_cli_with_log_command(
                ["sh", "-c", shlex.quote(script)],
                str(log_path),
                timeout_message=timeout_message,
            )
            completed = subprocess.run(
                command,
                shell=True,
                capture_output=True,
                text=True,
            )
            log = log_path.read_text() if log_path.exists() else ""
            return completed, log

    def test_preserves_non_timeout_exit_status_through_tee(self) -> None:
        completed, log = self.run_logged_command("echo before-failure; exit 7")

        self.assertEqual(completed.returncode, 7)
        self.assertIn("before-failure", completed.stdout)
        self.assertIn("before-failure", log)

    def test_maps_eval_cli_timeout_exit_to_success(self) -> None:
        completed, log = self.run_logged_command(
            "echo partial-output; exit 2",
            timeout_message="timeout converted",
        )

        self.assertEqual(completed.returncode, 0)
        self.assertIn("partial-output", completed.stdout)
        self.assertIn("timeout converted", completed.stdout)
        self.assertIn("partial-output", log)
        self.assertIn("timeout converted", log)


class EnvForwardingTests(unittest.TestCase):
    def test_anthropic_available_models_env_is_added(self) -> None:
        env: dict[str, str] = {}
        add_anthropic_available_models_env(env, '[{"name":"model"}]')

        self.assertEqual(env["ZED_ANTHROPIC_AVAILABLE_MODELS"], '[{"name":"model"}]')

    def test_configure_modal_environment_sets_documented_secret_env(self) -> None:
        args = argparse.Namespace(
            app_name="app",
            volume="volume",
            modal_token_secret="modal-secret",
            api_secret="llm-secret",
        )

        with patch.dict(os.environ, {}, clear=False):
            common.configure_modal_environment(args)

            self.assertEqual(os.environ["AGENT_EVALS_APP_NAME"], "app")
            self.assertEqual(os.environ["AGENT_EVALS_VOLUME"], "volume")
            self.assertEqual(
                os.environ["AGENT_EVALS_MODAL_TOKEN_SECRET"], "modal-secret"
            )
            self.assertEqual(
                os.environ["AGENT_EVALS_LLM_PROVIDERS_SECRET"], "llm-secret"
            )


class SafeExtractArchiveTests(unittest.TestCase):
    def make_archive(
        self, archive_path: Path, member_name: str, data: bytes = b"data"
    ) -> None:
        with tarfile.open(archive_path, "w:gz") as archive:
            info = tarfile.TarInfo(member_name)
            info.size = len(data)
            archive.addfile(info, io.BytesIO(data))

    def test_extracts_normal_members(self) -> None:
        with tempfile.TemporaryDirectory() as temporary_directory:
            root = Path(temporary_directory)
            archive_path = root / "archive.tar.gz"
            destination = root / "out"
            destination.mkdir()
            self.make_archive(archive_path, "job/result.json", b"{}")

            with tarfile.open(archive_path, "r:gz") as archive:
                common.safe_extract_archive(archive, destination)

            self.assertEqual((destination / "job" / "result.json").read_text(), "{}")

    def test_rejects_path_traversal_members(self) -> None:
        with tempfile.TemporaryDirectory() as temporary_directory:
            root = Path(temporary_directory)
            archive_path = root / "archive.tar.gz"
            destination = root / "out"
            destination.mkdir()
            self.make_archive(archive_path, "../evil.txt")

            with tarfile.open(archive_path, "r:gz") as archive:
                with self.assertRaises(ValueError):
                    common.safe_extract_archive(archive, destination)

    def test_rejects_links(self) -> None:
        with tempfile.TemporaryDirectory() as temporary_directory:
            root = Path(temporary_directory)
            archive_path = root / "archive.tar.gz"
            destination = root / "out"
            destination.mkdir()
            with tarfile.open(archive_path, "w:gz") as archive:
                info = tarfile.TarInfo("link")
                info.type = tarfile.SYMTYPE
                info.linkname = "/tmp/target"
                archive.addfile(info)

            with tarfile.open(archive_path, "r:gz") as archive:
                with self.assertRaises(ValueError):
                    common.safe_extract_archive(archive, destination)


if __name__ == "__main__":
    unittest.main()
