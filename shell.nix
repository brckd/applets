{ pkgs ? import <nixpkgs> {}}:

pkgs.mkShell {
  packages = with pkgs; [
    rustc
    cargo
    pkg-config
    gtk4
    libadwaita
    librsvg
  ];
}
