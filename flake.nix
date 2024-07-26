{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = {
        nixpkgs.follows = "nixpkgs";
      };
    };
  };
  outputs = { self, nixpkgs, flake-utils, rust-overlay }:
    flake-utils.lib.eachDefaultSystem
      (system:
        let
          overlays = [ (import rust-overlay) ];
          pkgs = import nixpkgs {
            inherit system overlays;
          };
        in
        with pkgs;
        {
          devShells.default = with pkgs; mkShell {
            buildInputs = [
                openssl
                pkg-config
                # rust-bin.beta.latest.default
                (pkgs.rust-bin.stable.latest.rust.override {
                  extensions = ["rust-src"];
                })
                protobuf_25
            ];
          };
        }
      );
}
