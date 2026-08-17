{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
	buildInputs = [
		pkgs.gcc
		pkgs.libGL
		pkgs.pkg-config
	];
	LD_LIBRARY_PATH="${pkgs.libGL}/lib";
}