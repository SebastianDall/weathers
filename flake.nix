{
  description = "A flake for the ordinator-scheduler-frontend repo";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
        };
      in {
        devShells.default = pkgs.mkShell {
          buildInputs = [
            pkgs.openssl
            pkgs.pkg-config
            pkgs.cargo
            pkgs.rustc
          ];

          shellHook = ''
            # Point OPENSSL_DIR to the "dev" output (headers, etc.).
            export OPENSSL_DIR=${pkgs.openssl.dev}

            # Also ensure pkg-config sees the .pc files for OpenSSL
            export PKG_CONFIG_PATH=${pkgs.openssl.dev}/lib/pkgconfig:$PKG_CONFIG_PATH

            echo "OPENSSL_DIR  = $OPENSSL_DIR"
            echo "PKG_CONFIG_PATH = $PKG_CONFIG_PATH"
          '';
        };
    });
}
