# /usr/bin/bash
cargo run --bin sem
for f in *.dot; do
  dot "$f" -Tpng > "$f".png
done
ffmpeg -framerate 1 -start_number 1 -i 'out%d.dot.png' -vf format=gray -y -r 10 out.gif
