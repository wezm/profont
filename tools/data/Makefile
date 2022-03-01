# Note this is a BSD Makefile, run with bmake on non-BSD, non-macOS systems
# https://www.crufty.net/help/sjg/bmake.html
# https://archlinux.org/packages/community/x86_64/bmake/

PROFONT=../../target/debug/profont
PNG!= find . -type f -name '*.png'
RAW = ${PNG:.png=.raw}

all: ${RAW}

.SUFFIXES: .png .raw

.png.raw:
	convert ${.IMPSRC} -depth 1 gray:../../font/data/${.TARGET}

pngs:
	cargo build
	$(PROFONT) generate --strike 9 --font-size 7
	$(PROFONT) generate --strike 10 --font-size 9
	$(PROFONT) generate --strike 11 --font-size 10
	$(PROFONT) generate --strike 14 --font-size 12
	$(PROFONT) generate --strike 16 --font-size 14
	$(PROFONT) generate --strike 21 --font-size 18
	$(PROFONT) generate --strike 28 --font-size 24

.PHONY: pngs
