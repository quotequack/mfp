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
            cargoHash = "sha256-4BjFaNPUg299Cqo9eVyHO/Hm95TCDUib2KjMXiK/bEQ=";
            RUST_MIN_STACK="33554432";
            postInstall = ''
              mkdir -p $out/share/applications/
              mkdir -p $out/share/mime/packages/
              install -Dm644 mfpCreate.desktop $out/share/applications/mfpCreate.desktop
              install -Dm644 mfpRead.desktop $out/share/applications/mfpRead.desktop
              install -Dm644 mfpExtract.desktop $out/share/application/mfpExtract.desktop
              install -Dm644 image-mfp.xml $out/share/mime/packages/image-mfp.xml
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