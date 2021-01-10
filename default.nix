{ system ? builtins.currentSystem
, nixpkgsMozilla ? builtins.fetchGit {
    url = https://github.com/mozilla/nixpkgs-mozilla;
    ref = "master";
    rev = "4521bc61c2332f41e18664812a808294c8c78580";
  }
, cargo2nix ? builtins.fetchGit {
    url = https://github.com/tenx-tech/cargo2nix;
    ref = "master";
    rev = "a0f38b977596c39f4127b67c84a6930b3cbd662a";
  }
}:
let
  rustOverlay = import "${nixpkgsMozilla}/rust-overlay.nix";
  cargo2nixOverlay = import "${cargo2nix}/overlay";

  pkgs = import <nixpkgs> {
    inherit system;
    overlays = [ cargo2nixOverlay rustOverlay ];
  };

  rustPkgs = pkgs.rustBuilder.makePackageSet' {
    rustChannel = "nightly-2021-01-01";
    packageFun = import ./Cargo.nix;
  };
in
rec {
  fuzzr = rustPkgs.workspace.fuzzr { };
}
