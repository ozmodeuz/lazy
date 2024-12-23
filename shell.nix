{ pkgs, ... }:

{
  languages.rust = {
    enable = true;
    channel = "stable";
    components = [
      "rustc"
      "cargo"
      "clippy"
      "llvm-tools"
      "rustfmt"
      "rust-analyzer"
    ]
  };
}
