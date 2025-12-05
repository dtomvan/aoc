{
  description = "Generic devshell flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
    systems.url = "github:nix-systems/default";
    flake-parts.url = "github:hercules-ci/flake-parts";

    treefmt-nix = {
      url = "github:numtide/treefmt-nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    inputs@{ flake-parts, ... }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      systems = import inputs.systems;

      imports = [ inputs.treefmt-nix.flakeModule ];

      perSystem =
        { pkgs, lib, ... }:
        let
          mergeSources =
            sources:
            lib.pipe sources [
              lib.fileset.unions
              (
                fileset:
                lib.fileset.toSource {
                  inherit fileset;
                  root = ./.;
                }
              )
            ];
        in
        {
          treefmt = {
            programs.nixfmt.enable = true;
            programs.rustfmt = {
              enable = true;
              excludes = [ "year/*/src/_match_days.rs" ];
            };
            programs.fourmolu.enable = true;
          };

          packages = rec {
            haskell = pkgs.stdenv.mkDerivation (finalAttrs: {
              pname = "aoc";
              version = "0";
              src = ./haskell;

              nativeBuildInputs = [ pkgs.ghc ];

              installPhase = "make PREFIX=$out install";
            });
            default = haskell;

            rust = pkgs.rustPlatform.buildRustPackage (finalAttrs: {
              pname = "aoc";
              version = "0";

              src = mergeSources [
                ./Cargo.toml
                ./Cargo.lock
                ./util/common
                ./year
              ];

              nativeBuildInputs = with pkgs; [ pkg-config ];

              buildInputs = with pkgs; [ openssl ];

              doCheck = false;

              cargoDeps = pkgs.rustPlatform.importCargoLock {
                lockFile = ./Cargo.lock;
              };

              RUSTC_BOOTSTRAP = 1;
            });
          };

          devShells.default = pkgs.mkShell {
            packages = with pkgs; [
              cargo
              rustc
              clippy
              rust-analyzer
              pkg-config

              (ghc.withPackages (hpkgs: [ ]))
              haskell-language-server
            ];

            buildInputs = with pkgs; [
              openssl
            ];

            RUSTC_BOOTSTRAP = 1;
          };
        };
    };
}
