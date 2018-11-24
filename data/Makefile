PNG!= find . -type f -name '*.png'
RAW = ${PNG:.png=.raw}

all: ${RAW}

pngs:
	cargo run --release -- 7
	cargo run --release -- 9
	cargo run --release -- 10
	cargo run --release -- 12
	cargo run --release -- 14
	cargo run --release -- 18
	cargo run --release -- 24

.SUFFIXES: .png .raw

.png.raw:
	convert ${.IMPSRC} -depth 1 gray:${.TARGET}

.PHONY: pngs
