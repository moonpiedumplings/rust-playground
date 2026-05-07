{ pkgs,
  fenix,
  mkNixPak,
  ...
 }:

  let
      toolchain = fenix.combine [
        fenix.minimal.toolchain
        fenix.stable.rustc
        fenix.stable.cargo
        fenix.targets.wasm32-unknown-unknown.latest.rust-std
        fenix.targets.x86_64-unknown-linux-gnu.latest.rust-std 
        fenix.targets.x86_64-unknown-linux-musl.latest.rust-std     
        ];

      sandboxed-env = pkgs.mkShell {
        buildInputs = with pkgs; [
        rustfmt
        clippy
        rust-analyzer
        toolchain
        trunk
        amp-cli
          ];
      };


      amp-sandboxed = mkNixPak {
        config = { sloth, ...} : {
          app.package = sandboxed-env;
          app.binPath = "bin/bash";
          dbus.enable = true;
          bubblewrap = {
            network = true;
            bind.rw = [
              (sloth.env "PWD")
              [
                (sloth.concat' sloth.homeDir "/.local/share/amp")
                (sloth.concat' sloth.homeDir "/.config/amp")
              ]
            ];

            bind.ro = [
              "/nix"
            ];
          };
        };
      };

      packages = with pkgs; [

        # Main rust toolchain
        rustfmt
        clippy
        rust-analyzer
        toolchain # rust toolchain from fenix

        # Build libraries
        freetype
        fontconfig
        cmake
        expat
        pkg-config

        # X11 libraries
        libX11
        libXcursor
        libXi
        libXrandr
        libXinerama
        libxkbcommon
        wayland

        # Mesa libraries
        libGL
        libGLU
        mesa
        libglvnd
      ];
      
  in

pkgs.mkShell {


  buildInputs = packages;

  LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath packages;
  # LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath [
  #   pkgs.freetype
  #   pkgs.fontconfig
  #   pkgs.glibc
  #   pkgs.xorg.libX11
  # ];

  CMAKE_POLICY_VERSION_MINIMUM = "3.5";

  WINIT_UNIX_BACKEND = "x11";

  RUST_BACKTRACE = 1;
}
