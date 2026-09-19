from pathlib import Path
import stat
import tempfile
import unittest

from doc_links.io import write_text_atomic


class IoTest(unittest.TestCase):
    def test_atomic_write_preserves_existing_permissions(self):
        with tempfile.TemporaryDirectory() as directory:
            path = Path(directory) / "page.md"
            path.write_text("before", encoding="utf-8")
            path.chmod(0o755)
            write_text_atomic(path, "after")
            self.assertEqual(path.read_text(encoding="utf-8"), "after")
            self.assertEqual(stat.S_IMODE(path.stat().st_mode), 0o755)

    def test_new_atomic_file_is_readable(self):
        with tempfile.TemporaryDirectory() as directory:
            path = Path(directory) / "report.md"
            write_text_atomic(path, "report")
            self.assertEqual(stat.S_IMODE(path.stat().st_mode), 0o644)


if __name__ == "__main__":
    unittest.main()
