{
  inputs = {
    versions.url = "github:holochain/holochain?dir=versions/0_2";
    holonix.url = "github:holochain/holochain";
    holonix.inputs.versions.follows = "versions";
    holonix.inputs.holochain.url = "github:holochain/holochain/holochain-0.2.2";

    nixpkgs.follows = "holonix/nixpkgs";
  };

  outputs = inputs@{ holonix, ... }:
    holonix.inputs.flake-parts.lib.mkFlake { inherit inputs; } {
      # provide a dev shell for all systems that the holonix flake supports
      systems = builtins.attrNames holonix.devShells;

      perSystem = { config, system, pkgs, ... }:
        {
          devShells.default = pkgs.mkShell {
            inputsFrom = [ holonix.devShells.${system}.holonix ];
            packages = with pkgs; [
              # add further packages from nixpkgs
              cargo-watch
              cargo-nextest
              nodejs
              sqlite # temporary workaround, should be fixed in holochain-0.1.5
            ];
            shellHook = ''
              unset CARGO_TARGET_DIR
              unset CARGO_HOME
            '';
          };
        };
    };
}
