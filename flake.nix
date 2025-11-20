{
  description = "Rust Dev Environment";

  inputs = {
    flake-utils.url = "github:numtide/flake-utils";

    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

    crane.url = "github:ipetkov/crane";
  };

  outputs = { self, nixpkgs, crane, flake-utils }:
    flake-utils.lib.eachDefaultSystem
      (system:
        let
          pkgs = nixpkgs.legacyPackages.${system};

          craneLib = crane.mkLib pkgs;

          # Common arguments can be set here to avoid repeating them later
          # Note: changes here will rebuild all dependency crates
          commonArgs = {
            src = craneLib.cleanCargoSource ./.;
            strictDeps = true;

            buildInputs = [
              # Add additional build inputs here
            ]
            ++ pkgs.lib.optionals pkgs.stdenv.isDarwin [
              # Additional darwin specific inputs can be set here
              pkgs.libiconv
            ];

            nativeBuildInputs = [
              pkgs.installShellFiles
            ];

            postInstall = ''
              installManPage target/release/build/todo-rs-*/out/todo.1
              installManPage target/release/build/todo-rs-*/out/todo-rs.1
              # This is cursed, but for some reason, I cannot
              # refer to the directory I want by name, I have to
              # do it indirectly like this
              installShellCompletion target/release/build/todo-rs*/out/todo.{bash,fish}
              installShellCompletion --zsh target/release/build/todo-rs*/out/_todo
            '';
          };

          todo-rs = craneLib.buildPackage (
            commonArgs
            // {
              cargoArtifacts = craneLib.buildDepsOnly commonArgs;

              # Additional environment variables or build phases/hooks can be set
              # here *without* rebuilding all dependency crates
              # MY_CUSTOM_VAR = "some value";
            }
          );
        in
        {
          checks = {
            inherit todo-rs;
          };

          packages.default = todo-rs;

          apps.default = flake-utils.lib.mkApp {
            drv = todo-rs;
          };

          devShells.default = craneLib.devShell {
            # Inherit inputs from checks.
            checks = self.checks.${system};

            inputsFrom = [ todo-rs ];

            shellHook = ''
              rustc -V
              cargo -V
              export PATH=./target/debug/:./target/release/:$PATH
              echo "Added 'todo' binary to \$PATH"
            '';
          };
        }
      );
}
