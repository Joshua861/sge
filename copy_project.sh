#!/run/current-system/sw/bin/sh

text=$(tail -n +1 ./src/*.rs ./src/*/*.rs Cargo.toml)
echo "$text" | wl-copy
echo "$text" >output.txt
