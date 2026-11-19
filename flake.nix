{
  description = "keek";

  inputs = {
    nixpkgs.url = "nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    git-hooks.url = "github:cachix/git-hooks.nix";
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
      git-hooks,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs { inherit system; };

        buildCmd =
          { profile }:
          "${pkgs.cargo}/bin/cargo build --profile=${profile} --frozen --offline";
        testCmd =
          { profile }:
          "${pkgs.cargo}/bin/cargo test --profile=${profile} --frozen --offline";
        installCmd =
          { profile, out }:
          "${pkgs.cargo}/bin/cargo install --profile=${profile} --frozen --offline --path . --root ${out}";

        check =
          { profile }:
          git-hooks.lib.${system}.run {
            src = ./.;

            settings = {
              rust = {
                check.cargoDeps = pkgs.rustPlatform.importCargoLock {
                  lockFile = ./Cargo.lock;
                };
                cargoManifestPath = "./Cargo.toml";
              };
            };

            hooks = {
              cargo-check = {
                enable = true;
              };

              cargo-test = {
                enable = true;
                entry = testCmd { inherit profile; };
                pass_filenames = false;
                stages = [ "pre-commit" ];
                verbose = true;
                files = "\\.rs$|Cargo\\.toml$|Cargo\\.lock$";
                excludes = [ "target/" ];
              };

              rustfmt = {
                enable = true;
                settings = {
                  check = true;
                  verbose = true;
                  config = {
                    max_width = 80;
                  };
                };
              };

              clippy = {
                enable = true;
                args = [ "-Dwarnings" ];
              };

              cargo-doc = {
                enable = true;
                entry = "cargo deadlinks";
                extraPackages = [
                  pkgs.cargo
                  pkgs.cargo-deadlinks
                ];
                pass_filenames = false;
                stages = [ "pre-commit" ];
                verbose = true;
                files = "\\.rs$|Cargo\\.toml$|Cargo\\.lock$";
                excludes = [ "target/" ];
              };

              cargo-sort = {
                enable = true;
                args = [
                  "--check"
                  "--no-format"
                ];
              };
            };
          };

        lock =
          let
            lastModifiedDate = self.lastModifiedDate or self.lastModified or "19700101";
            version = "${builtins.substring 0 8 lastModifiedDate}-${self.shortRev or "dirty"}";
          in
          pkgs.rustPlatform.buildRustPackage rec {
            pname = "keek";
            inherit version;

            src = pkgs.nix-gitignore.gitignoreSource [ ".gitignore" ] ./.;

            cargoLock = {
              lockFile = ./Cargo.lock;
              allowBuiltinFetchGit = true;
            };

            profile = "release";

            buildPhase = (buildCmd { inherit profile; });

            doCheck = true;
            checkPhase = (check { inherit profile; }).shellHook;

            installPhase = ''
              mkdir -p $out
              ${installCmd {
                inherit profile;
                out = "$out";
              }}
              rm $out/.crates.toml
            '';
          };

        devShell = pkgs.mkShell {
          packages = with pkgs; [
            cargo
            cargo-sort
            cargo-deadlinks
            lazygit
            perf
          ];
          shellHook = (check { profile = "dev"; }).shellHook;
        };
      in
      {
        checks = {
          pre-commit-check = check { profile = "release"; };
        };

        packages.default = lock;
        devShells.default = devShell;
      }
    );
}
