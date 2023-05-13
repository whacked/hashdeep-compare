{
  description = "optional description";

  nixConfig.bash-prompt = ''\033[1;32m\[[nix-develop:\[\033[36m\]\w\[\033[32m\]]$\033[0m '';

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/23.05-pre";
    whacked-setup = {
      url = "github:whacked/setup/6b73eed531bdf4fde58a8214a94d11c4e024082e";
      flake = false;
    };
  };
  outputs = { self, nixpkgs, flake-utils, whacked-setup }:
    flake-utils.lib.eachDefaultSystem
    (system:
    let
      pkgs = nixpkgs.legacyPackages.${system};
      whacked-helpers = import (whacked-setup + /nix/flake-helpers.nix) { inherit pkgs; };
    in {
      devShell = whacked-helpers.mkShell {
        flakeFile = __curPos.file;  # used to forward current file to echo-shortcuts
        includeScripts = [
          (whacked-setup + /bash/nix_shortcuts.sh)
        ];
      } {
        buildInputs = [
          pkgs.cargo
          pkgs.rustc
        ];  # join lists with ++

        shellHook = ''
          alias compare='cargo run $*'
        '';  # join strings with +
      };
    }
  );
}
