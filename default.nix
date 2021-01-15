{ system ? builtins.currentSystem
, nixpkgsMozilla ? builtins.fetchGit {
    url = https://github.com/mozilla/nixpkgs-mozilla;
    ref = "master";
    rev = "8c007b60731c07dd7a052cce508de3bb1ae849b4";
  }
, cargo2nix ? builtins.fetchGit {
    url = https://github.com/positron-solutions/cargo2nix;
    ref = "dont-output-nulls-from-jq";
    rev = "a47a344b865c399a773e257e1ef1fc052c27ee86";
  }
}:
let
  rustOverlay = import "${nixpkgsMozilla}/rust-overlay.nix";
  rust = (pkgs.latest.rustChannels.nightly.rust.override {
    targets = [ "x86_64-unknown-linux-musl" ];
  });
  cargo2nixOverlay = import "${cargo2nix}/overlay";

  pkgs = import <nixpkgs> {
    inherit system;
    overlays = [ cargo2nixOverlay rustOverlay ];
  };

  # target = "x86_64-unknown-linux-musl";

  rustPkgs = pkgs.rustBuilder.makePackageSet' {
    rustChannel = "1.49.0";
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
            propagatedBuildInputs = drv.propagatedBuildInputs or [ ] ++ [ pkgs.cmake pkgs.vulkan-tools pkgs.vulkan-loader pkgs.vulkan-headers pkgs.mesa ];
          };
        };
        servo-fontconfig-sys = pkgs.rustBuilder.rustLib.makeOverride {
          name = "servo-fontconfig-sys";
          overrideAttrs = drv: {
            propagatedBuildInputs = drv.propagatedBuildInputs or [ ] ++ [ pkgs.file pkgs.cmake pkgs.expat pkgs.freetype pkgs.vulkan-tools pkgs.vulkan-loader pkgs.vulkan-headers pkgs.mesa ];
          };
        };
        x11 = pkgs.rustBuilder.rustLib.makeOverride {
          name = "x11";
          overrideAttrs = drv: {
            propagatedBuildInputs = drv.propagatedBuildInputs or [ ] ++ [ pkgs.x11 pkgs.vulkan-tools pkgs.vulkan-loader pkgs.vulkan-headers pkgs.mesa ];
          };
        };
        glow = pkgs.rustBuilder.rustLib.makeOverride {
          name = "glow";
          overrideAttrs = drv: {
            propagatedBuildInputs = drv.propagatedBuildInputs or [ ] ++ [ pkgs.vulkan-tools pkgs.vulkan-loader pkgs.vulkan-headers pkgs.mesa ];
          };
        };
        wgpu = pkgs.rustBuilder.rustLib.makeOverride {
          name = "wgpu";
          overrideAttrs = drv: {
            propagatedBuildInputs = drv.propagatedBuildInputs or [ ] ++ [ pkgs.vulkan-tools pkgs.vulkan-loader pkgs.vulkan-headers pkgs.mesa ];
          };
        };
        iced = pkgs.rustBuilder.rustLib.makeOverride {
          name = "iced";
          overrideAttrs = drv: {
            propagatedBuildInputs = drv.propagatedBuildInputs or [ ] ++ [ pkgs.vulkan-tools pkgs.vulkan-loader pkgs.vulkan-headers pkgs.mesa ];
          };
        };
        iced-wgpu = pkgs.rustBuilder.rustLib.makeOverride {
          name = "iced-wgpu";
          overrideAttrs = drv: {
            propagatedBuildInputs = drv.propagatedBuildInputs or [ ] ++ [ pkgs.vulkan-tools pkgs.vulkan-loader pkgs.vulkan-headers pkgs.mesa ];
          };
        };
        iced-glow = pkgs.rustBuilder.rustLib.makeOverride {
          name = "iced-glow";
          overrideAttrs = drv: {
            propagatedBuildInputs = drv.propagatedBuildInputs or [ ] ++ [ pkgs.vulkan-tools pkgs.vulkan-loader pkgs.vulkan-headers pkgs.mesa ];
          };
        };
        gfx-backend-vulkan = pkgs.rustBuilder.rustLib.makeOverride {
          name = "gfx-backend-vulkan";
          overrideAttrs = drv: {
            propagatedBuildInputs = drv.propagatedBuildInputs or [ ] ++ [ pkgs.vulkan-tools pkgs.vulkan-loader pkgs.vulkan-headers pkgs.mesa ];
          };
        };
      in
      pkgs: pkgs.rustBuilder.overrides.all ++ [ expat-sys servo-freetype-sys servo-fontconfig-sys x11 glow wgpu iced iced-wgpu iced-glow gfx-backend-vulkan ];
  };
in
rec {
  fuzzr = rustPkgs.workspace.fuzzr { };
}
