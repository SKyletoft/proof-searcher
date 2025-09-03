{
	inputs = {
		nixpkgs.url      = "github:nixos/nixpkgs/nixpkgs-unstable";
		flake-utils.url  = "github:numtide/flake-utils";
		rust-overlay.url = "github:oxalica/rust-overlay";
	};

	outputs = { self, nixpkgs, flake-utils, rust-overlay }:
		flake-utils.lib.eachDefaultSystem(system:
			let
				pkgs = import nixpkgs {
					inherit system;
					overlays = [ (import rust-overlay) ];
				};
				toolchain = pkgs.rust-bin.nightly.latest.default;
			in {
				packages.default = (pkgs.makeRustPlatform {
					cargo = toolchain;
					rustc = toolchain;
				}).buildRustPackage {
					pname = "proof-search";
					version = "0.0.1";
					src = ./.;
					cargoLock.lockFile = ./Cargo.lock;
				};
				devShells.default = pkgs.mkShell {
					nativeBuildInputs = [ toolchain ];
				};
			}
		);
}
