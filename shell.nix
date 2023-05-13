{ pkgs ? import <nixpkgs> {} }:
let
  flake-expression = import ./flake.nix;
  # HACK! but due to https://github.com/NixOS/nix/issues/3966
  # it is not straightforward to extract attributes from the imported flake
  flake-utils-repo = import (pkgs.fetchFromGitHub {
    owner = "numtide";
    repo = "flake-utils";
    rev = "main";
    hash = "sha256-H+Rh19JDwRtpVPAWp64F+rlEtxUWBAQW28eAi3SRSzg=";
  });
  flake-derivation = (flake-expression.outputs {
    self = null;
    nixpkgs = {
      legacyPackages = {
        ${pkgs.system} = pkgs;
      };
    };
    flake-utils = {
      lib = flake-utils-repo;
    };
    whacked-setup = ~/setup;
  }).devShell.${pkgs.system};

  helpers = import (
    builtins.fetchurl {
      url = "https://raw.githubusercontent.com/whacked/setup/5a0b52059f19151e15d44ec773c17dcc1540f3c2/nix/helpers.nix";
      sha256 = "1vby29hnzhannl75kjc55169pbdglfxvsmqszj7krzik60lhgyfq";
    });
in helpers.mkShell [
] {
  buildInputs = flake-derivation.buildInputs;

  nativeBuildInputs = [
    (builtins.fetchurl {
      url = "https://raw.githubusercontent.com/whacked/setup/5a0b52059f19151e15d44ec773c17dcc1540f3c2/bash/nix_shortcuts.sh";
      sha256 = "12lqrmwdvxn54vy5h72h6q5a9zfa2hphz6j3vlcxsbmnqsghgg4s";
    })
  ];

  shellHook = flake-derivation.shellHook;
}
