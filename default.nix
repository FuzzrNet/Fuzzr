{ system ? builtins.currentSystem
, nixpkgsMozilla ? builtins.fetchGit {
    url = https://github.com/mozilla/nixpkgs-mozilla;
    ref = "master";
    rev = "8c007b60731c07dd7a052cce508de3bb1ae849b4";
  }
, cargo2nix ? builtins.fetchGit {
    url = https://github.com/tenx-tech/cargo2nix;
    ref = "master";
    rev = "433cd5b53d91a9577e7bfaa910df6b8eb8528bbc";
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
    rustChannel = "stable";
    packageFun = import ./Cargo.nix;
    packageOverrides =
      let
        expat-sys = pkgs.rustBuilder.rustLib.makeOverride {
          name = "expat-sys";
          overrideAttrs = drv: {
            propagatedBuildInputs = drv.propagatedBuildInputs or [ ] ++ [ pkgs.cmake ];
          };
        };
        servo-freetype-sys = pkgs.rustBuilder.rustLib.makeOverride {
          name = "servo-freetype-sys";
          overrideAttrs = drv: {
            propagatedBuildInputs = drv.propagatedBuildInputs or [ ] ++ [ pkgs.cmake ];
          };
        };
        servo-fontconfig-sys = pkgs.rustBuilder.rustLib.makeOverride {
          name = "servo-fontconfig-sys";
          overrideAttrs = drv: {
            propagatedBuildInputs = drv.propagatedBuildInputs or [ ] ++ [ pkgs.file pkgs.cmake pkgs.expat pkgs.freetype ];
          };
        };
        x11 = pkgs.rustBuilder.rustLib.makeOverride {
          name = "x11";
          overrideAttrs = drv: {
            propagatedBuildInputs = drv.propagatedBuildInputs or [ ] ++ [ pkgs.x11 ];
          };
        };
        # iced = pkgs.rustBuilder.rustLib.makeOverride {
        #   name = "iced";
        #   overrideAttrs = drv: {
        #     propagatedBuildInputs = drv.propagatedBuildInputs or [ ] ++ [ pkgs.vulkan-tools pkgs.mesa ];
        #   };
        # };
        # wgpu = pkgs.rustBuilder.rustLib.makeOverride {
        #   name = "wgpu";
        #   overrideAttrs = drv: {
        #     propagatedBuildInputs = drv.propagatedBuildInputs or [ ] ++ [ pkgs.vulkan-tools pkgs.mesa ];
        #   };
        # };
        # gfx-backend-vulkan = pkgs.rustBuilder.rustLib.makeOverride {
        #   name = "gfx-backend-vulkan";
        #   overrideAttrs = drv: {
        #     propagatedBuildInputs = drv.propagatedBuildInputs or [ ] ++ [ pkgs.vulkan-tools pkgs.mesa ];
        #   };
        # };
      in
      pkgs: pkgs.rustBuilder.overrides.all ++ [ expat-sys servo-freetype-sys servo-fontconfig-sys x11 ];
  };
in
rec {
  fuzzr = rustPkgs.workspace.fuzzr { };
}
