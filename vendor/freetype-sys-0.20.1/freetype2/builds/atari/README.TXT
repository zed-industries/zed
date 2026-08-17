Compiling FreeType 2 with PureC compiler
========================================

[See below for a German version.]

To compile FreeType 2 as a library the following changes must be applied:

- All *.c files must start with an uppercase letter.
  (In case GEMSCRIPT is available:
  Simply drag the whole FreeType 2 directory to the file `FNames.SIC'.)

- You have to change the INCLUDE directory in PureC's compiler options
  to contain both the `INCLUDE' and `freetype2\include' directory.
  Example:

    INCLUDE;E:\freetype2\include

- The file `freetype/include/Ft2build.h' must be patched as follows to
  include ATARI.H:

    #ifndef FT2_BUILD_GENERIC_H_
    #define FT2_BUILD_GENERIC_H_

    #include "ATARI.H"



Compilieren von FreeType 2 mit PureC
====================================

Um FreeType 2 als eine Bibliothek (library) zu compilieren, muss folgendes
geÑndert werden:

- Alle *.c-files mÅssen mit einem GROSSBUCHSTABEN beginnen.
  (Falls GEMSCRIPT zur VerfÅgung steht:
  Den kompletten Ordner freetype2 auf die Datei `FNames.SIC' draggen.)

- In den Compiler-Optionen von PureC muss das INCLUDE directory auf INCLUDE
  und freetype2\include verweisen. Z.B.:

    INCLUDE;E:\freetype2\include

- In der Datei freetype/include/Ft2build.h muss zu Beginn
  ein #include "ATARI.H" wie folgt eingefÅgt werden:

    #ifndef FT2_BUILD_GENERIC_H_
    #define FT2_BUILD_GENERIC_H_

    #include "ATARI.H"

--- end of README.TXT ---
