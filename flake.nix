{
  description = "A very basic flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
  };

  outputs =
    { self, nixpkgs }:
    let
      system = "x86_64-linux";
      pkgs = nixpkgs.legacyPackages.${system};
    in
    {

      devShells.${system}.default = pkgs.mkShell rec {
        nativeBuildInputs = with pkgs; [
          pkg-config
          nushell
        ];
        buildInputs = with pkgs; [
          udev
          alsa-lib
          vulkan-loader
          # xorg.libX11
          # xorg.libXcursor
          # xorg.libXi
          libxkbcommon
          wayland # To use the wayland feature
        ];
        LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath buildInputs;
        shellHook = ''
          exec nu
        '';
      };
    };
}
