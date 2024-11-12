import sys
import os
import shutil

crate_name = sys.argv[1]

src_path = f"crates/{crate_name}/src"
dest_path = f"crates/zed_common/src/{crate_name}"

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

# Check all the Cargo.toml files for {crate_name}.workspace = true and remove line
for root, dirs, files in os.walk("."):
    if "Cargo.toml" in files:
        cargo_toml_path = os.path.join(root, "Cargo.toml")
        with open(cargo_toml_path, "r") as f:
            lines = f.readlines()

        new_lines = [line for line in lines if f"{crate_name}.workspace = true" not in line]

        if len(new_lines) != len(lines):
            with open(cargo_toml_path, "w") as f:
                f.writelines(new_lines)
            print(f"Removed {crate_name}.workspace = true from {cargo_toml_path}")

# Add module to lib.rs file in the correct ordering
with open("crates/zed_common/src/lib.rs", "r") as f:
    lines = f.readlines()

mod_line = f"pub mod {crate_name};\n"

if mod_line not in lines:
    # Find the correct position to insert the new module
    insert_index = 0
    for i, line in enumerate(lines):
        if line.startswith("pub mod"):
            if line > mod_line:
                insert_index = i
                break
            else:
                insert_index = i + 1

    lines.insert(insert_index, mod_line)

    with open("crates/zed_common/src/lib.rs", "w") as f:
        f.writelines(lines)
    print(f"Added {crate_name} module to lib.rs")
else:
    print(f"Module {crate_name} already exists in lib.rs")

# Print the Cargo.toml content
cargo_toml_path = f"crates/{crate_name}/Cargo.toml"
if os.path.exists(cargo_toml_path):
    with open(cargo_toml_path, "r") as f:
        print(f"Contents of {cargo_toml_path}:")
        print("-----------------------------")
        print(f.read())
        print("-----------------------------")
else:
    print(f"Cargo.toml for {crate_name} not found")

# Remove the src path and print the cargo toml
if os.path.exists(f"crates/{crate_name}"):
    shutil.rmtree(f"crates/{crate_name}")
    print(f"Removed src directory from {crate_name}")
else:
    print(f"Source directory for {crate_name} not found")
