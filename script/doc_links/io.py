import json
import os
from pathlib import Path
import stat
import tempfile
from typing import Any, Callable, TextIO


def read_json(path: Path) -> Any:
    try:
        return json.loads(path.read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError) as error:
        raise RuntimeError(f"Could not read JSON from {path}: {error}") from error


def write_atomic(path: Path, writer: Callable[[TextIO], None]) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    mode = stat.S_IMODE(path.stat().st_mode) if path.exists() else 0o644
    descriptor, temporary_name = tempfile.mkstemp(
        dir=path.parent,
        prefix=f".{path.name}.",
        suffix=".tmp",
    )
    temporary_path = Path(temporary_name)
    try:
        os.fchmod(descriptor, mode)
        with os.fdopen(descriptor, "w", encoding="utf-8") as file:
            writer(file)
            file.flush()
            os.fsync(file.fileno())
        temporary_path.replace(path)
    except Exception:
        temporary_path.unlink(missing_ok=True)
        raise


def write_json_atomic(path: Path, value: Any) -> None:
    def write(file: TextIO) -> None:
        json.dump(value, file, indent=2, sort_keys=True)
        file.write("\n")

    write_atomic(path, write)


def write_text_atomic(path: Path, value: str) -> None:
    write_atomic(path, lambda file: file.write(value))
