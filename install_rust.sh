#!/bin/sh

# From https://www.rust-lang.org/tools/install
echo "NOTE: Please choose a complete installation of stable releases after selecting option 2"
echo "NOTE: After that, select Option 1 to proceed with installation and setup cargo in your PATH"
echo "Sleeping for 10 seconds..."
sleep 10
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
