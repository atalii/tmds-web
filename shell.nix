{
  pkgs ? import <nixpkgs> { },
}:

pkgs.mkShell {
  buildInputs = with pkgs; [
    cargo
    rustc
    wasm-pack

    lld
    clang

    nodejs
  ];
}
