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
PANDOCBEAMER = \
	       --pdf-engine=xelatex \
	       --from=markdown \
	       --mathjax \
	       --standalone \
	       --listings \
	       -H listings.tex \
	       -t beamer


FIGURES = $(shell find . -name '*.svg')

#all: $(patsubst %.md, %.pdf, $(wildcard *.md))

proposal.pdf: proposal.md Makefile
	pandoc $< -o $@ $(PANDOCFLAGS)

presentation.pdf: presentation.md Makefile
	pandoc $< -o $@ $(PANDOCBEAMER) 
