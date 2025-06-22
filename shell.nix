{ pkgs ? import <nixpkgs> {} }:
  pkgs.mkShell {
    # nativeBuildInputs is usually what you want -- tools you need to run
    nativeBuildInputs = with pkgs.buildPackages; [ rustc cargo pkg-config  ];
    packages = with pkgs; [ pciutils usbutils cpuid ];

}
