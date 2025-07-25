{
  description = "git-gardener - A powerful Git worktree management tool";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        
        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rust-src" "rust-analyzer" ];
        };

        buildInputs = with pkgs; [
          pkg-config
          openssl
          libiconv
        ] ++ lib.optionals stdenv.isDarwin [
          darwin.apple_sdk.frameworks.Security
          darwin.apple_sdk.frameworks.SystemConfiguration
        ];

        nativeBuildInputs = with pkgs; [
          rustToolchain
          pkg-config
        ];

      in
      {
        packages.default = pkgs.rustPlatform.buildRustPackage {
          pname = "git-gardener";
          version = "0.1.0";
          
          src = ./.;
          
          cargoLock = {
            lockFile = ./Cargo.lock;
          };
          
          inherit buildInputs;
          nativeBuildInputs = nativeBuildInputs ++ [ pkgs.installShellFiles ];
          
          # Skip tests during build (they require a git repository setup)
          doCheck = false;
          
          # Install shell completions
          postInstall = ''
            # Install completions
            installShellCompletion --cmd git-gardener \
              --bash completions/git-gardener.bash \
              --zsh completions/git-gardener.zsh \
              --fish completions/git-gardener.fish
          '';
          
          meta = with pkgs.lib; {
            description = "A powerful Git worktree management tool with TUI interface";
            homepage = "https://github.com/mei28/git-gardener";
            license = licenses.mit;
            maintainers = [ ];
            platforms = platforms.unix;
          };
        };

        packages.git-gardener = self.packages.${system}.default;

        devShells.default = pkgs.mkShell {
          inherit buildInputs;
          nativeBuildInputs = nativeBuildInputs ++ (with pkgs; [
            # Development tools
            cargo-watch
            cargo-edit
            just
            
            # Additional tools for development
            git
            pre-commit
          ]);
          
          RUST_BACKTRACE = "1";
          
          shellHook = ''
            echo "🌱 git-gardener development environment"
            echo "Available commands:"
            echo "  cargo build    - Build the project"
            echo "  cargo test     - Run tests"
            echo "  cargo run      - Run git-gardener"
            echo "  just --list    - Show available just commands"
          '';
        };

        # For backwards compatibility
        defaultPackage = self.packages.${system}.default;
        devShell = self.devShells.${system}.default;
      }
    );
}