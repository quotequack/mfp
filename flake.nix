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
            cargoHash = "sha256-eWZTrM+eeqJx64HNkPLKAYMmzUSrCg/mU8ypwqbCZNw=";
            postInstall = ''
              mkdir -p $out/share/applications/
              install -Dm644 mfpCreate.desktop $out/share/applications/mfpCreate.desktop
              install -Dm644 mfpRead.desktop $out/share/applications/mfpRead.desktop
            '';
        };
        devShells."x86_64-linux".default = pkgs.mkShell {
            buildInputs = with pkgs; [
                cargo
                rustc 
                just
            ];
            nativeBuildInputs = [ pkgs.pkg-config ];
            env.RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
        };
    };
}