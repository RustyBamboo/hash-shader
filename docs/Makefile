.PHONY: phony

PANDOCFLAGS = \
	      --pdf-engine=xelatex 		\
	      --from=markdown 			\
	      --number-sections 		\
	      --highlight-style=monochrome 	\
	      --mathjax 			\
	      --standalone 			\
	      -V documentclass=article 		\
	      -V geometry:margin=0.5in 		\
		  -V urlcolor=blue


FIGURES = $(shell find . -name '*.svg')

all: $(patsubst %.md, %.pdf, $(wildcard *.md))

%.pdf: %.md
	pandoc $< -o $@ $(PANDOCFLAGS)
