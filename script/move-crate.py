import sys
import os
import shutil

crate_name = sys.argv[1]

src_path = f"crates/{crate_name}/src"
dest_path = f"crates/zed_common/{crate_name}/src"

# Move the crate to zed_common
if os.path.exists(src_path):
    shutil.move(src_path, dest_path)
    print(f"Moved code to zed_common")
else:
    print(f"Source path {src_path} does not exist")

src_mod_file = f"{dest_path}/{crate_name}.rs"
# Rename crate_name.rs to mod.rs
if os.path.exists(src_mod_file):
    os.rename(src_mod_file, f"{dest_path}/mod.rs")
    print("Created new mod.rs file")
else:
    print(f"Source path {dest_path}.rs does not exist")

print("Fixing imports...")

# Look through all the text files, scan for use crate:: and replace with use crate::{crate_name}
for root, dirs, files in os.walk(dest_path):
    for file in files:
        if file.endswith(".rs"):
            with open(os.path.join(root, file), "r") as f:
                content = f.read()
                new_content = content.replace(f"crate::", f"crate::{crate_name}::")
                if new_content != content:
                    with open(os.path.join(root, file), "w") as f:
                        f.write(new_content)

print("Fixed imports")

# Remove crate from workspace file
with open("Cargo.toml", "r") as f:
    lines = f.readlines()

new_lines = [line for line in lines if not (f"crates/{crate_name}" in line or f"{crate_name}.workspace = true" in line)]

if len(new_lines) != len(lines):
    with open("Cargo.toml", "w") as f:
        f.writelines(new_lines)
    print(f"Removed {crate_name} from workspace file")
else:
    print(f"Crate {crate_name} not found in workspace file")
