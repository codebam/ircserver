{
  inputs = {
    utils.url = "github:numtide/flake-utils";
  };
  outputs = { self, nixpkgs, utils }: utils.lib.eachDefaultSystem (system:
    let
      pkgs = nixpkgs.legacyPackages.${system};
    in
    {
      packages.default = pkgs.rustPlatform.buildRustPackage rec {
        pname = "hello";
        version = "0.1.0";
        src = ./.;
        cargoHash = "sha256-nZimqEgcrWzy1wOdepGjNET9J7iwfhZ8WG7a85/XRbw=";
      };
      devShell = pkgs.mkShell {
        buildInputs = with pkgs; [
          cargo
          rust-analyzer
          rustc
          rustfmt
        ];
      };
    }
  );
}
