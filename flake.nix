{
  description = "rust devshell and package, created by scaffolder";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    naersk = {
      url = "github:nix-community/naersk";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
      naersk,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs { inherit system; };

        naersk' = pkgs.callPackage naersk {};
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

        packages.haal = naersk'.buildPackage {
          src = ./.;

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
