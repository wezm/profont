# Note this is a BSD Makefile, run with bmake on non-BSD, non-macOS systems
# https://www.crufty.net/help/sjg/bmake.html
# https://archlinux.org/packages/community/x86_64/bmake/

PNG!= find ../../tools/data -type f -name '*.png'
RAW = ${PNG:.png=.raw}

all: ${RAW}

.SUFFIXES: .png .raw

.png.raw:
	convert ${.IMPSRC} -depth 1 gray:../../font/data/${.TARGET}
