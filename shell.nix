{ nixpkgs ? import <nixpkgs> { }}:

let
    rust_overlay = import (builtins.fetchTarball "https://github.com/oxalica/rust-overlay/archive/master.tar.gz");
    pkgs = import <nixpkgs> { overlays = [ rust_overlay ]; };
    unstable = import <nixos-unstable> { config = { allowUnfree = true; }; };
    rust = pkgs.rust-bin.stable.latest.default.override {
      extensions = [ "rust-src" ];
    };

in pkgs.mkShell {
    name = "rust-shell";

    buildInputs = [
      rust

      pkgs.cmake
      pkgs.evcxr
      pkgs.jupyter
      
      unstable.clippy
      unstable.rustfmt
      unstable.rust-analyzer

      pkgs.exercism
      pkgs.vscode
    ];

    shellHook = ''
      if [ -e ~/.nixpkgs/shellhook.sh ]; then . ~/.nixpkgs/shellhook.sh; fi

      export MAGIC_DB=${pkgs.file}/share/misc/magic.mgc;

      exercism configure --token=6bbd960a-851c-43e5-9052-014e38e76f04 --workspace=/home/christoph/Projects/exercism/

      {pkgs.evcxr}/bin/evcxr_jupyter --install
    '';
}
