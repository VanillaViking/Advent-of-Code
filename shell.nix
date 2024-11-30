{ pkgs ? import <nixpkgs> { } }:

with pkgs;

mkShell {
buildInputs = [ rustup ]; # your dependencies here
}
