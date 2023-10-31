import os
from pathlib import Path

THIS_SCRIPT_PATH: Path = Path(__file__)
CRATES_DIR: Path = THIS_SCRIPT_PATH.parent.parent / "crates"

zed_1_crate_count: int = 0
zed_2_crate_count: int = 0

for child in os.listdir(CRATES_DIR):
    child_path: str = os.path.join(CRATES_DIR, child)

    if not os.path.isdir(child_path):
        continue

    if child.endswith("2"):
        zed_2_crate_count += 1
    else:
        zed_1_crate_count += 1

print(f"crates ported: {zed_2_crate_count}")
print(f"crates in total: {zed_1_crate_count}")

percent_complete: float = (zed_2_crate_count / zed_1_crate_count) * 100
percent_complete_rounded: float = round(percent_complete, 2)

print(f"progress: {percent_complete_rounded}%")
