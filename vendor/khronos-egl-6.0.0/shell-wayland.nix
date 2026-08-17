{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
	buildInputs = [
		pkgs.gcc
		pkgs.libGL
		pkgs.pkg-config
		pkgs.wayland
	];
	LD_LIBRARY_PATH="${pkgs.libGL}/lib";
}
