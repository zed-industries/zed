## Usage

```sh
python3 gen-universal-table.py > ../src/hb/ot_shaper_use_table.rs

python3 ./gen-vowel-constraints.py > ../src/complex/vowel_constraints.rs
rustfmt ../src/complex/vowel_constraints.rs
```
