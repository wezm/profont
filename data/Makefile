# Note this is a BSD Makefile, run with bmake on non-BSD, non-macOS systems
# https://www.crufty.net/help/sjg/bmake.html
# https://archlinux.org/packages/community/x86_64/bmake/

PNG!= find . -type f -name '*.png'
RAW = ${PNG:.png=.raw}

all: ${RAW} ../src/main.rs

pngs:
	cargo build --release --features exe
	../target/release/profont 9 7
	../target/release/profont 10 9
	../target/release/profont 11 10
	../target/release/profont 14 12
	../target/release/profont 16 14
	../target/release/profont 21 18
	../target/release/profont 28 24

.SUFFIXES: .png .raw

.png.raw:
	convert ${.IMPSRC} -depth 1 gray:${.TARGET}

.PHONY: pngs
