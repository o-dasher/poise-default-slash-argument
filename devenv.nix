{ pkgs, ... }:
{
  packages = with pkgs; [
    nixfmt-rfc-style
  ];


  languages.rust = {
    enable = true;
    channel = "nightly";
  };
}
