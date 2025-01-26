{
  description = "devshell for a Rust project";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

  inputs.crane.url = "github:ipetkov/crane";

  inputs.devshell.url = "github:numtide/devshell";
  inputs.devshell.inputs.nixpkgs.follows = "nixpkgs";

  inputs.fenix.url = "github:nix-community/fenix";
  inputs.fenix.inputs.nixpkgs.follows = "nixpkgs";

  inputs.flake-compat.url = "github:nix-community/flake-compat";
  inputs.flake-compat.flake = false;

  inputs.flake-utils.url = "github:numtide/flake-utils";

  outputs =
    {
      nixpkgs,
      crane,
      devshell,
      fenix,
      flake-utils,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ devshell.overlays.default ];
        };

        rustStable = (import fenix { inherit pkgs; }).fromToolchainFile {
          file = ./toolchain.toml;
          sha256 = "sha256-lMLAupxng4Fd9F1oDw8gx+qA0RuF7ou7xhNU8wgs0PU=";
        };

        craneLib = (crane.mkLib pkgs).overrideToolchain rustStable;

        spark-rs = craneLib.buildPackage {
          src = craneLib.cleanCargoSource (craneLib.path ./.);
          buildInputs = [ ];
          nativeBuildInputs = [ ];
          cargoClippyExtraArgs = "--all-targets -- --deny warnings";
        };
      in
      {
        checks = {
          inherit spark-rs;
        };

        packages.default = spark-rs;

        apps.default = flake-utils.lib.mkApp { drv = spark-rs; };

        devShells.default = pkgs.devshell.mkShell {
          env = [
            {
              name = "DEVSHELL_NO_MOTD";
              value = 1;
            }
          ];

          packages = with pkgs; [
            just
            rustStable
            sqlx-cli
          ];
        };
      }
    );
}
