import json
import os
import runpy
import tempfile
import unittest
from pathlib import Path
from unittest.mock import patch


probe = Path(__file__).resolve().parents[1] / "src/kernels/python_kernel_env.py"
kernel_env = runpy.run_path(str(probe))["kernel_env"]


class PythonKernelEnvironmentTests(unittest.TestCase):
    def setUp(self):
        temporary = tempfile.TemporaryDirectory()
        self.addCleanup(temporary.cleanup)
        self.prefix = Path(temporary.name)
        self.spec = self.prefix / "share/jupyter/kernels/python3/kernel.json"
        self.spec.parent.mkdir(parents=True)

    def test_selected_environment_and_parent_substitutions(self):
        self.spec.write_text(json.dumps({
            "language": "python",
            "env": {"MARKER": "configured", "COPY": "${MARKER}", "EMPTY": ""},
        }), encoding="utf-8")
        with patch.dict(os.environ, MARKER="inherited"):
            self.assertEqual(kernel_env(self.prefix), {
                "MARKER": "configured", "COPY": "inherited", "EMPTY": "",
            })
            self.assertEqual(kernel_env(self.prefix / "other"), {})

    def test_optional_kernelspec(self):
        self.assertEqual(kernel_env(self.prefix), {})
        for content in (
            "{",
            '{"language":"python"}',
            '{"language":"python","env":{"INVALID":123}}',
            '{"language":"r","env":{"MARKER":"other kernel"}}',
        ):
            with self.subTest(content=content):
                self.spec.write_text(content, encoding="utf-8")
                self.assertEqual(kernel_env(self.prefix), {})
