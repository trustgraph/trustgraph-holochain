{
  holonixPath ?  builtins.fetchTarball { url = "https://github.com/holochain/holonix/archive/d15633710a8d4349dc0ff03b7b47ad01eb9f2433.tar.gz"; }
}:

let
  holonix = import (holonixPath) {
    rustVersion = {
      track = "stable";
      version = "1.57.0";  # outside of nix = 1.58
    };
  };
  nixpkgs = holonix.pkgs;
in nixpkgs.mkShell {
  inputsFrom = [ holonix.main ];
  buildInputs = with nixpkgs; [
    binaryen
    cargo-release
    # cargo-watch # broken on M1
    nodejs-16_x
  ];
}
