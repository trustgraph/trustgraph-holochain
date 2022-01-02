{
  holonixPath ?  builtins.fetchTarball { url = "https://github.com/holochain/holonix/archive/d15633710a8d4349dc0ff03b7b47ad01eb9f2433.tar.gz"; }
}:

let
  holonix = import (holonixPath) {
    holochainVersionId = "v0_0_119";
    rustVersion = {
      track = "stable";
      version = "1.57.0";
    };
  };
  nixpkgs = holonix.pkgs;
in nixpkgs.mkShell {
  inputsFrom = [ holonix.main ];
  buildInputs = with nixpkgs; [
    binaryen
    nodejs-16_x
    cargo-watch
  ];
}
