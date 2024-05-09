{
  inputs = {
    utils.url = "github:numtide/flake-utils";
  };
  outputs = { self, nixpkgs, utils }: utils.lib.eachDefaultSystem (system:
    let
      pkgs = nixpkgs.legacyPackages.${system};
    in
    {
      packages.default = pkgs.stdenv.mkDerivation
        {
          name = "hello-rust";
          src = ./.;
          nativeBuildInputs = with pkgs; [
          ];
          buildInputs = with pkgs; [
            cargo
          ];
          buildPhase = ''
            cargo build
          '';
          installPhase = ''
            mkdir -p $out/bin
            cp target/debug/hello $out/bin
          '';
        };
      devShell = pkgs.mkShell {
        buildInputs = with pkgs; [
          cargo
        ];
      };
    }
  );
}
