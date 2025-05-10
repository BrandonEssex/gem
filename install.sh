#!/bin/bash

echo "Installing gemx..."
mkdir -p ~/.config/gemx
cp -r ./themes ~/.config/gemx/
cp -r ./plugins ~/.config/gemx/
cp ./keymap.json ~/.config/gemx/
cp ./settings.json ~/.config/gemx/
echo "alias gemx='cargo run --release'" >> ~/.bashrc
echo "Installed. Reload your shell or run 'source ~/.bashrc'"
