{
  description = "A devShell example";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = {
    self,
    nixpkgs,
    rust-overlay,
    flake-utils,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        overlays = [(import rust-overlay)];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
      in
        with pkgs; {
          devShells.default = mkShell {
            buildInputs = [
              openssl
              pkg-config
              nodejs
              wasm-pack
              eslint_d
              typescript
              trunk
              postgresql
              diesel-cli
              (rust-bin.stable.latest.default.override
                {
                  extensions = ["rust-src"];
                  targets = ["wasm32-unknown-unknown"];
                })
            ];

            shellHook = ''
              export DATABASE_URL=postgres://postgres:mypassword@localhost
              docker start catdex-db
                echo "Environment is ready" | ${pkgs.lolcat}/bin/lolcat;
            '';
          };
        }
    );
}
