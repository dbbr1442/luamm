{
  description = "A very basic flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
  };

  outputs = { self, nixpkgs }: 
  let
    system = "x86_64-linux";
    pkgs = nixpkgs.legacyPackages.${system};

    nativeBuildInputs = with pkgs; [ rustup lua-language-server ];
    buildInputs = with pkgs; [ pkg-config alsa-lib xorg.libX11 xorg.libXi libxkbcommon libGL ]; 
    
    cargoTOML = builtins.fromTOML (builtins.readFile ./Cargo.toml);
    pname = cargoTOML.package.name;
    version = cargoTOML.package.version;
  in {
    devShells.${system}.default = pkgs.mkShell {
      inherit nativeBuildInputs buildInputs;  
      LD_LIBRARY_PATH = "${pkgs.lib.makeLibraryPath buildInputs}";
      C_INCLUDE_PATH = pkgs.lib.makeIncludePath [ ];
      EDITOR = "nvim";
    };
  };
}
