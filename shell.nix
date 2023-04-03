{ pkgs ? import <nixpkgs> {} }:
let
  helpers = import ~/setup/nix/helpers.nix;
in helpers.mkShell [
] {
  buildInputs = [
    pkgs.cargo
  ];  # join lists with ++

  nativeBuildInputs = [
    ~/setup/bash/nix_shortcuts.sh
  ];

  shellHook = ''
    echo-shortcuts ${__curPos.file}
  '';  # join strings with +
}
