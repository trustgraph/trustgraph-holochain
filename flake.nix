{
  inputs = {

    holochain = {
      url = "github:holochain/holochain";
      inputs.versions.url = "github:holochain/holochain?dir=versions/0_1";
    };

    nixpkgs.follows = "holochain/nixpkgs";
  };

  outputs = inputs @ { ... }:
    inputs.holochain.inputs.flake-parts.lib.mkFlake
      {
        inherit inputs;
      }
      {
        systems = builtins.attrNames inputs.holochain.devShells;
        perSystem =
          { config
          , pkgs
          , system
          , ...
          }: {
            devShells.default = pkgs.mkShell {
              inputsFrom = [ inputs.holochain.devShells.${system}.holonix ];
              packages = [
                pkgs.cargo-nextest pkgs.sqlite # temporary workaround, should be fixed in holochain-0.1.5
              ];

              shellHook = ''
                unset CARGO_TARGET_DIR
                unset CARGO_HOME
              '';
            };
          };
      };
}
