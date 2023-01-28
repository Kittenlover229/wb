# /usr/bin/bash

cargo run --bin sem
dot out1.dot -Tpng > out1.png
dot out2.dot -Tpng > out2.png
dot out3.dot -Tpng > out3.png
dot out4.dot -Tpng > out4.png
dot out5.dot -Tpng > out5.png
dot out6.dot -Tpng > out6.png
dot out7.dot -Tpng > out7.png
dot out8.dot -Tpng > out8.png

ffmpeg -framerate 1 -start_number 1 -i 'out%d.png' -vf format=gray -y -r 10 out.gif
