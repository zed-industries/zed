# LLVM Reference Files

These are a copy of the relevant LLVM files that were ported to Rust from the
last time that this project was "synced" with LLVM.

Currently that sync point is 20.1.8, commit [87f0227](https://github.com/llvm/llvm-project/tree/87f0227cb60147a26a1eeb4fb06e3b505e9c7261).

These files were originally located at:
* `llvm/include/llvm/BinaryFormat/COFF.h`
* `llvm/include/llvm/Object/Archive.h`
* `llvm/include/llvm/Object/ArchiveWriter.h`
* `llvm/include/llvm/Object/COFFImportFile.h`
* `llvm/include/llvm/Support/Alignment.h`
* `llvm/include/llvm/Support/MathExtras.h`
* `llvm/lib/IR/Mangler.cpp`
* `llvm/lib/Object/ArchiveWriter.cpp`
* `llvm/lib/Object/COFFImportFile.cpp`
* `llvm/unittests/IR/ManglerTest.cpp`

When syncing, make sure to update these files and the commit above.

Additionally, `ar_archive_writer` has removed some options, so you can assume:
* `deterministic` is always `true`.
* `write_symtab` is always `true`.
