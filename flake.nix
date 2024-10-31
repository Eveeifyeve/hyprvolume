{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";
    fenix.url = "github:nix-community/fenix";

		pre-commit-hooks = {
      url = "github:cachix/git-hooks.nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = inputs@{ nixpkgs, flake-parts, ... }:
    flake-parts.lib.mkFlake {inherit inputs;} {
      systems = nixpkgs.lib.systems.flakeExposed;
      perSystem = {
        lib,
        pkgs,
        system,
        config,
        ...
      }: 
      {
        _module.args.pkgs = import nixpkgs {
          inherit system;
          overlays = [
            (inputs.fenix.overlays.default)
          ];
        };

				checks = {
					pre-commit-check = inputs.pre-commit-hooks.lib.${system}.run {
            src = ./.;
            hooks = {
							rustfmt.enable = true;
							cargo-check.enable = true;
							clippy.enable = true;
              nixfmt-rfc-style.enable = true;
            };
          };
				};

        devShells.default = with pkgs; let 
        toolchain = pkgs.fenix.stable.withComponents [
          "rustc"
          "cargo"
          "clippy"
        ];
        in mkShell
        {
					inherit (self.checks.${system}.pre-commit-check) shellHook;
					buildInputs = self.checks.${system}.pre-commit-check.enabledPackages;
          packages = with pkgs; [
            openssl
            rust-analyzer
            toolchain
          ];
        };

				# Feel free to make a nixpkg for it long as you maintain it. 
				packages.default = pkgs.rustPlatform.buildRustPackage rec {
					pname = "hyprvolume";
					version = "1.0.0";
					
					src = pkgs.fetchFromGithub {
						owner = "eveeifyeve";
						repo = "hyprvolume";
						rev = "v${version}";
						hash = lib.fakeHash;
					};
					cargoHash = lib.fakeHash;
				};

      };
    };
}
