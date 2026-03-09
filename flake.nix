{
    inputs = {
        nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    };

    outputs = { self, nixpkgs }: let
    pkgs = nixpkgs.legacyPackages."x86_64-linux";
    in {
        packages."x86_64-linux".default = pkgs.rustPlatform.buildRustPackage {
            name = "mfpSuite";
            src = ./.;
            buildInputs = [];
            nativeBuildInputs = [];
            cargoHash = "sha256-ME5DyLKwyqejCm2y4ycEOWizZKVI6vyJRGYzVKEg8Bc=";
        };
        devShells."x86_64-linux".default = pkgs.mkShell {
            buildInputs = with pkgs; [
                cargo
                rustc 
            ];
            nativeBuildInputs = [ pkgs.pkg-config ];
            env.RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
        };
    };
}