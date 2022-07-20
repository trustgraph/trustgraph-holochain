let
  holonixRev = "8dea7f5e572128fde79cb939ad048531fae2d436";

  holonixPath = builtins.fetchTarball "https://github.com/holochain/holonix/archive/${holonixRev}.tar.gz";
  holonix = import (holonixPath) {
    rustVersion = {
      track = "stable";
      version = "1.63.0";
    };
    holochainVersionId = "v0_0_162";
  };
  nixpkgs = holonix.pkgs;
in nixpkgs.mkShell {
  inputsFrom = [ holonix.main ];
  packages = with nixpkgs; [
    cargo-release
    cargo-watch
    niv
  ];
}
