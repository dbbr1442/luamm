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
    buildInputs = with pkgs; [ pkg-config alsa-lib libX11 libXi libxkbcommon libGL stdenv.cc.cc.lib ]; 
    
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

    packages.${system}.default = pkgs.rustPlatform.buildRustPackage {
      inherit buildInputs pname version;
      #preBuild = ''
      #  export HOME=$(mktemp -d)
      #'';

      nativeBuildInputs = [
        pkgs.rustc
        pkgs.cargo
      ];

      postFixup = '' 
        patchelf --set-rpath ${pkgs.lib.makeLibraryPath buildInputs} $out/bin/${pname}
      '';

      LD_LIBRARY_PATH = "${pkgs.lib.makeLibraryPath buildInputs}";#:${pkgs.stdenv.cc.cc.lib}/lib" ;

      cargoLock = {
        outputHashes = {
         "lli-0.1.0" = "sha256-EiL6Eqh7KbrWwpLnFk+OuaLmdyRppV+Lsaq/hB81QNU=";
       };
       lockFile = ./Cargo.lock;
      };

      src = ./.;
    };
  };
}
