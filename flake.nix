{
  description = "Development environment for Rust project with OpenSSL";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs"; # Use the latest stable version or pin it to a specific commit
  };

  outputs = { self, nixpkgs }:
    let
      pkgs = import nixpkgs {
        system = "x86_64-linux"; # Adjust for your system if needed
      };
    in {
      devShells.default = pkgs.mkShell {
        buildInputs = [
          pkgs.openssl
          pkgs.pkg-config
          pkgs.cargo
          pkgs.rustc
        ];

        # Export the OpenSSL environment variable
        shellHook = ''
          export OPENSSL_DIR=${pkgs.openssl.dev}
        '';
      };
    };
}

