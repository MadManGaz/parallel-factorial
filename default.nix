{ sources ? import ./nix/sources.nix
, pkgs ? import sources.nixpkgs {}
}:

pkgs.rustPlatform.buildRustPackage rec {
  pname = "parallel-factorial";
  version = "0.2.2-dev";

  nativeBuildInputs = [
    pkgs.niv
  ];

  src = ./.;

  cargoSha256 = "sha256-lXyfdQEbLTyqkdcPiyTvl1ltKvMU3qNJsBJ0mLC4KJM=";
}

