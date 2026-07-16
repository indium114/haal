build:
    nix build .#haal

run:
    cargo run

configure:
    mkdir -p ~/.config/haal
    cp ./config/init.lua ~/.config/haal
    cp ./config/logo.txt ~/.config/haal
