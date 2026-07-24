{
  description = "Grimoire";
  
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    # to easily make configs for multiple architectures
    flake-utils.url = "github:numtide/flake-utils";
  };
  
  outputs = { self, nixpkgs, flake-utils }:
    let
      supportedSystems = [
        "x86_64-linux"
        "x86_64-darwin"
        "aarch64-darwin"
        "aarch64-linux"
      ];
    in
    flake-utils.lib.eachSystem supportedSystems(
      system:
      let pkgs = nixpkgs.legacyPackages.${system};
      in {
        devShells.default = pkgs.mkShell {
          name = "Grimoire";

          packages = with pkgs; [
            nodejs_22  # NOTE: Pin to LTS
          ];

          # Fish is nice so use it if ya got it.
          shellHook = ''
            if command -v fish >/dev/null && [ -z "$IN_FISH" ]; then
              exec fish
            fi
          '';
        };
      }
    );
}
