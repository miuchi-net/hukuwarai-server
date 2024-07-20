{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = {
    self,
    nixpkgs,
    rust-overlay,
    flake-utils,
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [
            (import rust-overlay)
          ];
        };
        toolchain = pkgs.rust-bin.stable.latest.default;
        buildInputsForBuild = with pkgs;
          pkgs.lib.optional pkgs.stdenv.isDarwin [
            darwin.Security
            darwin.apple_sdk.frameworks.SystemConfiguration
          ];
        nativeBuildInputsForBuild = with pkgs; [pkg-config];
        customBuildRustCrateForPkgs = pkgs:
          pkgs.buildRustCrate.override {
            defaultCrateOverrides =
              pkgs.defaultCrateOverrides
              // {
                hukuwarai = attrs: {
                  buildInputs = buildInputsForBuild;
                  nativeBuildInputs = nativeBuildInputsForBuild;
                };
                sqlx-macros = attrs: {
                  buildInputs = buildInputsForBuild;
                  nativeBuildInputs = nativeBuildInputsForBuild;
                };
              };
          };
        generatedBuild = pkgs.callPackage ./Cargo.nix {
          buildRustCrateForPkgs = customBuildRustCrateForPkgs;
        };
      in rec {
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs;
            buildInputsForBuild
            ++ nativeBuildInputsForBuild
            ++ [
              alejandra
              nil
              toolchain
              rust-analyzer
              crate2nix
              openapi-generator-cli
            ];
        };
        packages.hukuwarai = generatedBuild.workspaceMembers."hukuwarai".build;
        packages.default = packages.hukuwarai;
        apps.${system}.default = {
          type = "app";
          program = "${self.packages.default}/bin/hukuwarai";
        };
        packages.dockerImages = {
          hukuwarai = pkgs.dockerTools.buildImage {
            name = "hukuwarai";
            tag = "latest";
            copyToRoot = pkgs.buildEnv {
              name = "hukuwarai-env";
              paths = [packages.hukuwarai pkgs.sqlx-cli pkgs.cacert];
            };
            config = {
              EntryPoint = [
                "sh"
                "-c"
                "sqlx migrate run && /bin/hukuwarai"
              ];
            };
          };
        };
      }
    );
}
