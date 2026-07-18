{
  description = "rust devshell and package, created by scaffolder";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs { inherit system; };
      in
      {
        devShells.default = pkgs.mkShell {
          name = "rust-devshell";

          packages = with pkgs; [
            cargo
            rustc
            rustfmt
            rust-analyzer
            clippy
            pkg-config
            pciutils
          ];
        };

        packages.haal = pkgs.rustPlatform.buildRustPackage {
          name = "haal";
          version = "0.1.1";

          src = ./.;

          cargoLock.lockFile = ./Cargo.lock;

          nativeBuildInputs = [
            pkgs.makeBinaryWrapper
          ];

          postInstall = ''
            wrapProgram $out/bin/haal \
              --prefix PATH : ${pkgs.lib.makeBinPath [ pkgs.pciutils ]}
          '';
        };

        apps.haal = {
          type = "app";
          program = "${self.packages.${pkgs.stdenv.hostPlatform.system}.haal}/bin/haal";
        };
      }
    );
}
