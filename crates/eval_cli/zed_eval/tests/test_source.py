from __future__ import annotations

import argparse
import unittest
from unittest.mock import patch

from zed_eval import source
from zed_eval.builds import prepare_build_request, resolve_source


class ResolveRemoteRefTests(unittest.TestCase):
    def test_full_sha_needs_no_network(self) -> None:
        sha = "a" * 40
        with patch("zed_eval.source.git_output") as git_output:
            self.assertEqual(source.resolve_remote_ref("url", sha), sha)
            git_output.assert_not_called()

    def test_annotated_tag_prefers_peeled_commit(self) -> None:
        ls_remote = (
            "1111111111111111111111111111111111111111\trefs/tags/v0.210.0\n"
            "2222222222222222222222222222222222222222\trefs/tags/v0.210.0^{}"
        )
        with patch("zed_eval.source.git_output", return_value=ls_remote):
            self.assertEqual(source.resolve_remote_ref("url", "v0.210.0"), "2" * 40)

    def test_unresolvable_ref_raises(self) -> None:
        with patch("zed_eval.source.git_output", return_value=""):
            with self.assertRaises(ValueError):
                source.resolve_remote_ref("url", "missing")


class BuildIdDedupTests(unittest.TestCase):
    """A clean tag and the SHA it resolves to must yield the *same* build id, so
    a build done by one teammate is reused by another."""

    def _build_id_for(self, from_source: str, resolved_sha: str) -> str:
        args = argparse.Namespace(
            from_source=from_source,
            base_sha=None,
            clean_source=False,
            zed_version=None,
            repo_url=source.DEFAULT_REPO_URL,
        )
        with patch("zed_eval.source.resolve_remote_ref", return_value=resolved_sha):
            base_sha, clean, label, pre_resolved = resolve_source(args)
        request = prepare_build_request(
            base_sha=base_sha,
            patch_path=None,
            build_id=None,
            allow_untracked=False,
            require_clean=False,
            repo_url=source.DEFAULT_REPO_URL,
            clean_source=clean,
            source_label=label,
            pre_resolved_base_sha=pre_resolved,
        )
        return request["build_id"]

    def test_tag_and_sha_share_build_id(self) -> None:
        sha = "c" * 40
        from_tag = self._build_id_for("v0.210.0", sha)
        from_sha = self._build_id_for(sha, sha)
        self.assertEqual(from_tag, from_sha)
        self.assertTrue(from_tag.startswith("bld-"))


if __name__ == "__main__":
    unittest.main()
