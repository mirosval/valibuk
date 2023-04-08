{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs";
    flake-utils.url = "github:numtide/flake-utils";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };
  
  outputs = { self, nixpkgs, flake-utils, fenix }: 
    flake-utils.lib.eachDefaultSystem (system:
      let 
        pkgs = nixpkgs.legacyPackages.${system};
        packages = with pkgs; [
          cargo-watch
          cargo-expand
        ];
        components = [
          "cargo"
          "rustc"
        ];
        buildInputs = with pkgs; [
          iconv
        ];
      in {
        devShells.default = pkgs.mkShell {
          packages = packages;
          buildInputs = [
            fenix.packages.${system}.complete.toolchain
          ] ++ buildInputs;
        };
        devShells.stable = pkgs.mkShell {
          packages = packages;
          buildInputs = [
            fenix.packages.${system}.stable.toolchain
          ] ++ buildInputs;
        };
      }
    );
}
