#!/bin/bash
# see the README file for usage etc.
#
# ------------------------------------------------------------------
#  This file is part of bzip2/libbzip2, a program and library for
#  lossless, block-sorting data compression.
#
#  bzip2/libbzip2 version 1.0.8 of 13 July 2019
#  Copyright (C) 1996-2019 Julian Seward <jseward@acm.org>
#
#  Please read the WARNING, DISCLAIMER and PATENTS sections in the 
#  README file.
#
#  This program is released under the terms of the license contained
#  in the file LICENSE.
# ----------------------------------------------------------------


usage() {
  echo '';
  echo 'Usage: xmlproc.sh -[option] <filename.xml>';
  echo 'Specify a target from:';
  echo '-v      verify xml file conforms to dtd';
  echo '-html   output in html format (single file)';
  echo '-ps     output in postscript format';
  echo '-pdf    output in pdf format';
  exit;
}

if test $# -ne 2; then
  usage
fi
# assign the variable for the output type
action=$1; shift
# assign the output filename
xmlfile=$1; shift
# and check user input it correct
if !(test -f $xmlfile); then
  echo "No such file: $xmlfile";
  exit;
fi
# some other stuff we will use
OUT=output
xsl_fo=bz-fo.xsl
xsl_html=bz-html.xsl

basename=$xmlfile
basename=${basename//'.xml'/''}

fofile="${basename}.fo"
htmlfile="${basename}.html"
pdffile="${basename}.pdf"
psfile="${basename}.ps"
xmlfmtfile="${basename}.fmt"

# first process the xmlfile with CDATA tags
./format.pl $xmlfile $xmlfmtfile
# so the shell knows where the catalogs live
export XML_CATALOG_FILES=/etc/xml/catalog

# post-processing tidy up
cleanup() {
  echo "Cleaning up: $@" 
  while [ $# != 0 ]
  do
    arg=$1; shift;
    echo "  deleting $arg";
    rm $arg
  done
}

case $action in
  -v)
   flags='--noout --xinclude --noblanks --postvalid'
   dtd='--dtdvalid http://www.oasis-open.org/docbook/xml/4.2/docbookx.dtd'
   xmllint $flags $dtd $xmlfmtfile 2> $OUT 
   egrep 'error' $OUT 
   rm $OUT
  ;;

  -html)
   echo "Creating $htmlfile ..."
   xsltproc --nonet --xinclude  -o $htmlfile $xsl_html $xmlfmtfile
   cleanup $xmlfmtfile
  ;;

  -pdf)
   echo "Creating $pdffile ..."
   xsltproc --nonet --xinclude -o $fofile $xsl_fo $xmlfmtfile
   pdfxmltex $fofile >$OUT </dev/null
   pdfxmltex $fofile >$OUT </dev/null
   pdfxmltex $fofile >$OUT </dev/null
   cleanup $OUT $xmlfmtfile *.aux *.fo *.log *.out
  ;;

  -ps)
   echo "Creating $psfile ..."
   xsltproc --nonet --xinclude -o $fofile $xsl_fo $xmlfmtfile
   pdfxmltex $fofile >$OUT </dev/null
   pdfxmltex $fofile >$OUT </dev/null
   pdfxmltex $fofile >$OUT </dev/null
   pdftops $pdffile $psfile
   cleanup $OUT $xmlfmtfile $pdffile *.aux *.fo *.log *.out
#  passivetex is broken, so we can't go this route yet.
#   xmltex $fofile >$OUT </dev/null
#   xmltex $fofile >$OUT </dev/null
#   xmltex $fofile >$OUT </dev/null
#   dvips -R -q -o bzip-manual.ps *.dvi
  ;;

  *)
  usage
  ;;
esac
