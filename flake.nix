{
  description = "Epochal - GTK4 Rust application with Blueprint UI";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, crane, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        
        craneLib = crane.mkLib pkgs;
        
        # Filter source to include Rust files and Blueprint files
        src = pkgs.lib.cleanSourceWith {
          src = craneLib.path ./.;
          filter = path: type:
            (craneLib.filterCargoSources path type)
            || (pkgs.lib.hasSuffix ".blp" path)
            || (pkgs.lib.hasSuffix ".ui" path)
            || (pkgs.lib.hasSuffix ".gresource.xml" path)
            || (pkgs.lib.hasSuffix ".desktop" path);
        };

        # Common build arguments
        commonArgs = {
          inherit src;
          strictDeps = true;

          nativeBuildInputs = with pkgs; [
            pkg-config
            gobject-introspection
            blueprint-compiler
            wrapGAppsHook
            desktop-file-utils
            glib # for glib-compile-resources
          ];

          buildInputs = with pkgs; [
            gtk4
            libadwaita
            glib
            cairo
            pango
            gdk-pixbuf
            graphene
            xdotool
          ];
        };

        # Build dependencies only (for caching)
        cargoArtifacts = craneLib.buildDepsOnly commonArgs;

        # The actual application
        epochal = craneLib.buildPackage (commonArgs // {
          inherit cargoArtifacts;
          
          # Pre-build hook to compile Blueprint files
          preBuild = ''
            # Compile all Blueprint files to UI XML
            for blp in $(find . -name "*.blp"); do
              ui="''${blp%.blp}.ui"
              echo "Compiling $blp to $ui"
              blueprint-compiler compile --output "$ui" "$blp"
            done
            
            # Compile GResource files if present
            if [ -f "data/resources.gresource.xml" ]; then
              echo "Compiling GResource bundle"
              glib-compile-resources --sourcedir=data data/resources.gresource.xml
            fi
          '';

          # Post-install hook for desktop integration
          postInstall = ''
            # Install desktop files
            if [ -d "data" ]; then
              for desktop in data/*.desktop; do
                if [ -f "$desktop" ]; then
                  install -Dm644 "$desktop" -t $out/share/applications
                fi
              done
            fi
            
            # Install icons if present
            if [ -d "data/icons" ]; then
              cp -r data/icons/* $out/share/icons/
            fi
          '';
        });

      in
      {
        packages.default = epochal;
        packages.epochal = epochal;

        devShells.default = craneLib.devShell {
          # Inherit inputs from commonArgs
          inputsFrom = [ commonArgs ];
          
          # Additional dev tools
          packages = with pkgs; [
            # Rust development
            rust-analyzer
            rustfmt
            clippy
            cargo-watch
            
            # GTK development tools
            gtk4.dev
            libadwaita.dev
            blueprint-compiler
            
            # Debugging and inspection
            gdb
            valgrind
            
            # BDD Testing tools
            xvfb-run
            xdotool
            
            # Optional: GNOME development tools
            # gnome.devhelp
            # gnome-builder
          ];

          # Shell hook for development
          shellHook = ''
            export RUST_SRC_PATH="${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}"
            
            # Enable GTK Inspector for debugging
            export GTK_DEBUG=interactive
            
            # Ensure schemas can be found
            export XDG_DATA_DIRS="$XDG_DATA_DIRS:${pkgs.gtk4}/share:${pkgs.libadwaita}/share"
            
            # Set up headless testing environment
            export DISPLAY=:99
            if ! pgrep -x "Xvfb" > /dev/null; then
              echo "Starting Xvfb for headless testing..."
              Xvfb :99 -screen 0 1024x768x24 > /dev/null 2>&1 &
              sleep 1
            fi
            
            echo "🚀 GTK4 + Blueprint development environment ready!"
            echo ""
            echo "Available commands:"
            echo "  cargo build                    - Build the application"
            echo "  cargo run -p epochal-app        - Run the application"
            echo "  cargo test -p bdd-tests         - Run BDD feature tests"
            echo "  cargo watch -x run             - Auto-rebuild and run on file changes"
            echo "  nix build                      - Build with Nix"
            echo ""
            echo "🧪 BDD Testing:"
            echo "  just test-bdd                  - Run BDD tests"
            echo "  just test-bdd-headless         - Run tests headless"
            echo "  just test-smoke                - Run smoke tests only"
            echo ""
            echo "GTK Inspector is enabled (GTK_DEBUG=interactive)"
            echo "Press Ctrl+Shift+I or Ctrl+Shift+D in the running app to open it"
          '';
        };

        # Formatting check
        checks = {
          epochal-fmt = craneLib.cargoFmt {
            inherit src;
          };
          
          epochal-clippy = craneLib.cargoClippy (commonArgs // {
            inherit cargoArtifacts;
            cargoClippyExtraArgs = "--all-targets -- --deny warnings";
          });
        };
      });
}