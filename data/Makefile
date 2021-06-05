PNG!= find . -type f -name '*.png'
RAW = ${PNG:.png=.raw}

all: ${RAW}

pngs:
	cargo run --release --features exe -- 7
	cargo run --release --features exe -- 9
	cargo run --release --features exe -- 10
	cargo run --release --features exe -- 12
	cargo run --release --features exe -- 14
	cargo run --release --features exe -- 18
	cargo run --release --features exe -- 24

.SUFFIXES: .png .raw

.png.raw:
	convert ${.IMPSRC} -depth 1 gray:${.TARGET}

.PHONY: pngs
