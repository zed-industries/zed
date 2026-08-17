overview.svg: overview.pdf
	pdf2svg $< $@

overview.pdf: overview.tex
	latexmk $<

overview.png: overview.svg
	svgexport $< $@ 3x
