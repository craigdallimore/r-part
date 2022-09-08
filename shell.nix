let
  rust_overlay = import (builtins.fetchTarball "https://github.com/oxalica/rust-overlay/archive/master.tar.gz");
  pkgs = import (fetchTarball "https://github.com/NixOS/nixpkgs/archive/nixos-22.05.tar.gz") { overlays = [ rust_overlay ]; };
  rust_channel = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain;
in pkgs.mkShell {

    name = "particles";

    # Required for rust-analyzer?
    RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";

    nativeBuildInputs = [
      rust_channel # Full rust from overlay, includes cargo
      pkgs.rustc
      pkgs.cargo
      pkgs.wasm-pack          # A utility that builds rust-generated WebAssembly
    ];

    buildInputs = [
      pkgs.pkg-config # Required for compiling on linux
      pkgs.openssl    # Required for compiling on linux
      pkgs.rustfmt
      pkgs.simple-http-server # For development
    ];
    shellHook = ''
      alias gst='git status'
      alias gd='git diff'
      alias gc='git commit'
      :q() {
        exit
      }
    '';
  }
