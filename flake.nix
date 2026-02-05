{
  description = "bonk - NixOS workflow multitool";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
      rust-overlay,
    }:
    # System-agnostic outputs
    {
      # Home Manager module
      homeManagerModules = {
        bonk = import ./nix/module.nix;
        default = self.homeManagerModules.bonk;
      };

      # Overlay for adding bonk to pkgs
      overlays.default = final: prev: {
        bonk = self.packages.${final.system}.default;
      };
    }
    //
      # System-specific outputs
      flake-utils.lib.eachDefaultSystem (
        system:
        let
          overlays = [ (import rust-overlay) ];
          pkgs = import nixpkgs {
            inherit system overlays;
          };

          # Use stable Rust toolchain
          rustToolchain = pkgs.rust-bin.stable.latest.default.override {
            extensions = [
              "rust-src"
              "rust-analyzer"
              "clippy"
            ];
          };

          # Native build inputs
          nativeBuildInputs = with pkgs; [
            pkg-config
            rustToolchain
            installShellFiles # For installing shell completions
          ];

        in
        {
          # Development shell
          devShells.default = pkgs.mkShell {
            inherit nativeBuildInputs;

            shellHook = ''
              echo "bonk development shell"
              echo "Rust: $(rustc --version)"
              echo ""
              echo "Available commands:"
              echo "  cargo build        - Build the project"
              echo "  cargo build -r     - Build release binary"
              echo "  cargo clippy       - Run linter"
              echo "  cargo fmt          - Format code"
              echo "  cargo test         - Run tests"
              echo ""
            '';
          };

          # Package definition
          packages.default = pkgs.rustPlatform.buildRustPackage {
            pname = "bonk";
            version = "0.1.0";

            src = ./.;

            cargoLock = {
              lockFile = ./Cargo.lock;
            };

            inherit nativeBuildInputs;

            # Install shell completions generated at build time by build.rs.
            # build.rs writes completions to $OUT_DIR/completions/ and records
            # the path in completions_dir.txt for reliable discovery.
            postInstall = ''
              # Read the completions directory path from the file created by build.rs
              completions_path_file=$(find target -name "completions_dir.txt" -type f | head -n1)
              if [ -f "$completions_path_file" ]; then
                completions_dir=$(cat "$completions_path_file")
                if [ -d "$completions_dir" ]; then
                  installShellCompletion --fish "$completions_dir/bonk.fish"
                  installShellCompletion --bash "$completions_dir/bonk.bash"
                  installShellCompletion --zsh "$completions_dir/_bonk"
                fi
              fi
            '';

            meta = with pkgs.lib; {
              description = "NixOS workflow multitool - wraps nh, nix, and nix-store";
              homepage = "https://github.com/tophc7/bonk";
              license = licenses.gpl3;
              maintainers = [ "tophc7" ];
              platforms = platforms.linux;
              mainProgram = "bonk";
            };
          };

          # Convenient alias
          packages.bonk = self.packages.${system}.default;
        }
      );
}
