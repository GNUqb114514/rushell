{
  description = "A flake-based Rust dev env";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
  };

  outputs = { self, nixpkgs, ... }: let
      system = "x86_64-linux";
  in {
    devShells."${system}".default = let
      pkgs = import nixpkgs {
        inherit system;
      };
    in pkgs.mkShell {
      packages = with pkgs; [
        rustc
	cargo
        rust-analyzer
        rustfmt
      ];
      shellhook = ''
        echo "Rustc $(${pkgs.rustc}/bin/rustc --version)"
        echo "Cargo $(${pkgs.cargo}/bin/rustc --version)"
	exec fish
      '';
    };
  };
}
