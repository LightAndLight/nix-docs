{
  description = "nix-docs generator";
  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
    cargo2nix.url = "github:cargo2nix/cargo2nix";
  };
  outputs = { self, nixpkgs, flake-utils, cargo2nix, rust-overlay }: 
    flake-utils.lib.eachSystem [ "x86_64-linux" ] (system:
      let 
        pkgs = import nixpkgs { 
          inherit system; 
          overlays = [ 
            (import "${cargo2nix}/overlay")
            rust-overlay.overlay 
          ]; 
        };

	rustChannel = "latest";
        
        rustPkgs = pkgs.rustBuilder.makePackageSet' {
          inherit rustChannel;
          packageFun = import "${self}/Cargo.nix";
          release = false;
        };
      in {
        devShell =
          pkgs.mkShell {
            buildInputs = [
              cargo2nix.defaultPackage.${system}
              (pkgs.rust-bin.stable.${rustChannel}.default.override {
                extensions = [
                  "cargo"
                  "clippy"
                  "rustc"
                  "rust-src"
                  "rustfmt"
                ];
              })
            ];
          };
      }
    );
}
