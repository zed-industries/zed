<?xml version="1.0"?> <!-- -*- sgml -*- -->
<xsl:stylesheet 
     xmlns:xsl="http://www.w3.org/1999/XSL/Transform" version="1.0">

<!-- we like '1.2 Title' -->
<xsl:param name="section.autolabel" select="'1'"/> 
<xsl:param name="section.label.includes.component.label" select="'1'"/>

<!-- Do not put 'Chapter' at the start of eg 'Chapter 1. Doing This' -->
<xsl:param name="local.l10n.xml" select="document('')"/> 
<l:i18n xmlns:l="http://docbook.sourceforge.net/xmlns/l10n/1.0"> 
  <l:l10n language="en"> 
    <l:context name="title-numbered">
      <l:template name="chapter" text="%n.&#160;%t"/>
    </l:context> 
  </l:l10n>
</l:i18n>

<!-- don't generate sub-tocs for qanda sets -->
<xsl:param name="generate.toc">
set       toc,title
book      toc,title,figure,table,example,equation
chapter   toc,title
section   toc
sect1     toc
sect2     toc
sect3     toc
sect4     nop
sect5     nop
qandaset  toc
qandadiv  nop
appendix  toc,title
article/appendix  nop
article   toc,title
preface   toc,title
reference toc,title
</xsl:param>

</xsl:stylesheet>
