PNG!= find . -type f -name '*.png'
RAW = ${PNG:.png=.raw}

all: ${RAW}

.SUFFIXES: .png .raw

.png.raw:
	convert ${.IMPSRC} -depth 1 gray:${.TARGET}
