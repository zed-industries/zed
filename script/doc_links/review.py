import json
from pathlib import Path

from . import SCHEMA_VERSION
from .io import read_json, write_text_atomic
from .schema import Report, content_hash

UI_DIR = Path(__file__).parent / "ui"


def load_report(path: Path) -> Report:
    try:
        return Report.from_dict(read_json(path))
    except ValueError as error:
        raise RuntimeError(f"Invalid audit report in {path}: {error}") from error


def review_data(report: Report, docs_dir: Path) -> dict:
    decisions = tuple(
        decision for decision in report.decisions if decision.queue != "rejected"
    )
    page_paths = {
        path
        for decision in decisions
        for path in (decision.source_path, decision.target_path)
    }
    pages = {}
    for path in sorted(page_paths):
        metadata = report.pages.get(path)
        if metadata is None:
            raise RuntimeError(f"Audit report is missing page metadata for {path}")
        markdown_path = docs_dir / path
        if not markdown_path.is_file():
            raise RuntimeError(f"Documentation page no longer exists: {path}")
        markdown = markdown_path.read_text(encoding="utf-8")
        if content_hash(markdown) != metadata["content_hash"]:
            raise RuntimeError(
                f"Documentation page changed after the audit: {path}. Rerun the audit."
            )
        pages[path] = {
            "title": metadata["title"],
            "markdown": markdown,
        }
    return {
        "schema_version": SCHEMA_VERSION,
        "report_hash": report.report_hash,
        "decisions": [decision.to_dict() for decision in decisions],
        "pages": pages,
    }


def generate_html(report_path: Path, docs_dir: Path, output_path: Path) -> int:
    report = load_report(report_path)
    data = review_data(report, docs_dir)
    template = (UI_DIR / "review.html").read_text(encoding="utf-8")
    style = (UI_DIR / "review.css").read_text(encoding="utf-8")
    script = (UI_DIR / "review.js").read_text(encoding="utf-8")
    serialized = json.dumps(data, separators=(",", ":")).replace("<", "\\u003c")
    replacements = {
        "/*__STYLE__*/": style,
        "/*__SCRIPT__*/": script,
        "null /*__DATA__*/": serialized,
    }
    html = template
    for marker, value in replacements.items():
        if html.count(marker) != 1:
            raise RuntimeError(f"Review template must contain one {marker} marker")
        html = html.replace(marker, value)
    write_text_atomic(output_path, html)
    return len(data["decisions"])
