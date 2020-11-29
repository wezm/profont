PNG!= find . -type f -name '*.png'
RAW = ${PNG:.png=.raw}

all: ${RAW}

TTFFONT:=ProFontIIx.ttf
FONTPREFIX:=ProFont

CARGO_ARGS:=run --release --features=exe --  $(TTFFONT) $(FONTPREFIX)

pngs:
	cargo  $(CARGO_ARGS) 7
	cargo  $(CARGO_ARGS) 9
	cargo  $(CARGO_ARGS) 10
	cargo  $(CARGO_ARGS) 12
	cargo  $(CARGO_ARGS) 14
	cargo  $(CARGO_ARGS) 18
	cargo  $(CARGO_ARGS) 24

.SUFFIXES: .png .raw

.png.raw:
	convert ${.IMPSRC} -depth 1 gray:${.TARGET}

.PHONY: pngs
