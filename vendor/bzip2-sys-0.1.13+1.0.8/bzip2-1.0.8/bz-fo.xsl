<?xml version="1.0" encoding="UTF-8"?> <!-- -*- sgml -*- -->
<xsl:stylesheet xmlns:xsl="http://www.w3.org/1999/XSL/Transform" 
     xmlns:fo="http://www.w3.org/1999/XSL/Format" version="1.0">

<xsl:import href="http://docbook.sourceforge.net/release/xsl/current/fo/docbook.xsl"/>
<xsl:import href="bz-common.xsl"/>

<!-- set indent = yes while debugging, then change to NO -->
<xsl:output method="xml" indent="yes"/>

<!-- ensure only passivetex extensions are on -->
<xsl:param name="stylesheet.result.type" select="'fo'"/>
<!-- fo extensions: PDF bookmarks and index terms -->
<xsl:param name="use.extensions" select="'1'"/>
<xsl:param name="xep.extensions" select="0"/>      
<xsl:param name="fop.extensions" select="0"/>     
<xsl:param name="saxon.extensions" select="0"/>   
<xsl:param name="passivetex.extensions" select="1"/>
<xsl:param name="tablecolumns.extension" select="'1'"/>

<!-- ensure we are using single sided -->
<xsl:param name="double.sided" select="'0'"/> 

<!-- insert cross references to page numbers -->
<xsl:param name="insert.xref.page.number" select="1"/>

<!-- <?custom-pagebreak?> inserts a page break at this point -->
<xsl:template match="processing-instruction('custom-pagebreak')">
  <fo:block break-before='page'/>
</xsl:template>

<!-- show links in color -->
<xsl:attribute-set name="xref.properties">
  <xsl:attribute name="color">blue</xsl:attribute>
</xsl:attribute-set>

<!-- make pre listings indented a bit + a bg colour -->
<xsl:template match="programlisting | screen">
  <fo:block start-indent="0.25in" wrap-option="no-wrap" 
            white-space-collapse="false" text-align="start" 
            font-family="monospace" background-color="#f2f2f9"
            linefeed-treatment="preserve" 
            xsl:use-attribute-sets="normal.para.spacing">
    <xsl:apply-templates/>
  </fo:block>
</xsl:template>
<!-- make verbatim output prettier -->
<xsl:template match="literallayout">
  <fo:block start-indent="0.25in" wrap-option="no-wrap" 
            white-space-collapse="false" text-align="start" 
            font-family="monospace" background-color="#edf7f4"
            linefeed-treatment="preserve" 
            space-before="0em" space-after="0em">
    <xsl:apply-templates/>
  </fo:block>
</xsl:template>

<!-- workaround bug in passivetex fo output for itemizedlist -->
<xsl:template match="itemizedlist/listitem">
  <xsl:variable name="id">
  <xsl:call-template name="object.id"/></xsl:variable>
  <xsl:variable name="itemsymbol">
    <xsl:call-template name="list.itemsymbol">
      <xsl:with-param name="node" select="parent::itemizedlist"/>
    </xsl:call-template>
  </xsl:variable>
  <xsl:variable name="item.contents">
    <fo:list-item-label end-indent="label-end()">
      <fo:block>
        <xsl:choose>
          <xsl:when test="$itemsymbol='disc'">&#x2022;</xsl:when>
          <xsl:when test="$itemsymbol='bullet'">&#x2022;</xsl:when>
          <xsl:otherwise>&#x2022;</xsl:otherwise>
        </xsl:choose>
      </fo:block>
    </fo:list-item-label>
    <fo:list-item-body start-indent="body-start()">
      <xsl:apply-templates/>    <!-- removed extra block wrapper -->
    </fo:list-item-body>
  </xsl:variable>
  <xsl:choose>
    <xsl:when test="parent::*/@spacing = 'compact'">
      <fo:list-item id="{$id}" 
          xsl:use-attribute-sets="compact.list.item.spacing">
        <xsl:copy-of select="$item.contents"/>
      </fo:list-item>
    </xsl:when>
    <xsl:otherwise>
      <fo:list-item id="{$id}" xsl:use-attribute-sets="list.item.spacing">
        <xsl:copy-of select="$item.contents"/>
      </fo:list-item>
    </xsl:otherwise>
  </xsl:choose>
</xsl:template>

<!-- workaround bug in passivetex fo output for orderedlist -->
<xsl:template match="orderedlist/listitem">
  <xsl:variable name="id">
  <xsl:call-template name="object.id"/></xsl:variable>
  <xsl:variable name="item.contents">
    <fo:list-item-label end-indent="label-end()">
      <fo:block>
        <xsl:apply-templates select="." mode="item-number"/>
      </fo:block>
    </fo:list-item-label>
    <fo:list-item-body start-indent="body-start()">
      <xsl:apply-templates/>    <!-- removed extra block wrapper -->
    </fo:list-item-body>
  </xsl:variable>
  <xsl:choose>
    <xsl:when test="parent::*/@spacing = 'compact'">
      <fo:list-item id="{$id}" 
          xsl:use-attribute-sets="compact.list.item.spacing">
        <xsl:copy-of select="$item.contents"/>
      </fo:list-item>
    </xsl:when>
    <xsl:otherwise>
      <fo:list-item id="{$id}" xsl:use-attribute-sets="list.item.spacing">
        <xsl:copy-of select="$item.contents"/>
      </fo:list-item>
    </xsl:otherwise>
  </xsl:choose>
</xsl:template>

<!-- workaround bug in passivetex fo output for variablelist -->
<xsl:param name="variablelist.as.blocks" select="1"/>
<xsl:template match="varlistentry" mode="vl.as.blocks">
  <xsl:variable name="id">
    <xsl:call-template name="object.id"/></xsl:variable>
  <fo:block id="{$id}" xsl:use-attribute-sets="list.item.spacing"  
      keep-together.within-column="always" 
      keep-with-next.within-column="always">
    <xsl:apply-templates select="term"/>
  </fo:block>
  <fo:block start-indent="0.5in" end-indent="0in" 
            space-after.minimum="0.2em" 
            space-after.optimum="0.4em" 
            space-after.maximum="0.6em">
    <fo:block>
      <xsl:apply-templates select="listitem"/>
    </fo:block>
  </fo:block>
</xsl:template>


<!-- workaround bug in footers: force right-align w/two 80|30 cols -->
<xsl:template name="footer.table">
  <xsl:param name="pageclass" select="''"/>
  <xsl:param name="sequence" select="''"/>
  <xsl:param name="gentext-key" select="''"/>
  <xsl:choose>
    <xsl:when test="$pageclass = 'index'">
      <xsl:attribute name="margin-left">0pt</xsl:attribute>
    </xsl:when>
  </xsl:choose>
  <xsl:variable name="candidate">
    <fo:table table-layout="fixed" width="100%">
      <fo:table-column column-number="1" column-width="80%"/>
      <fo:table-column column-number="2" column-width="20%"/>
      <fo:table-body>
        <fo:table-row height="14pt">
          <fo:table-cell text-align="left" display-align="after">
            <xsl:attribute name="relative-align">baseline</xsl:attribute>
            <fo:block> 
              <fo:block> </fo:block><!-- empty cell -->
            </fo:block>
          </fo:table-cell>
          <fo:table-cell text-align="center" display-align="after">
            <xsl:attribute name="relative-align">baseline</xsl:attribute>
            <fo:block>
              <xsl:call-template name="footer.content">
                <xsl:with-param name="pageclass" select="$pageclass"/>
                <xsl:with-param name="sequence" select="$sequence"/>
                <xsl:with-param name="position" select="'center'"/>
                <xsl:with-param name="gentext-key" select="$gentext-key"/>
              </xsl:call-template>
            </fo:block>
          </fo:table-cell>
        </fo:table-row>
      </fo:table-body>
    </fo:table>
  </xsl:variable>
  <!-- Really output a footer? -->
  <xsl:choose>
    <xsl:when test="$pageclass='titlepage' and $gentext-key='book'
                    and $sequence='first'">
      <!-- no, book titlepages have no footers at all -->
    </xsl:when>
    <xsl:when test="$sequence = 'blank' and $footers.on.blank.pages = 0">
      <!-- no output -->
    </xsl:when>
    <xsl:otherwise>
      <xsl:copy-of select="$candidate"/>
    </xsl:otherwise>
  </xsl:choose>
</xsl:template>


<!-- fix bug in headers: force right-align w/two 40|60 cols -->
<xsl:template name="header.table">
  <xsl:param name="pageclass" select="''"/>
  <xsl:param name="sequence" select="''"/>
  <xsl:param name="gentext-key" select="''"/>
  <xsl:choose>
    <xsl:when test="$pageclass = 'index'">
      <xsl:attribute name="margin-left">0pt</xsl:attribute>
    </xsl:when>
  </xsl:choose>
  <xsl:variable name="candidate">
    <fo:table table-layout="fixed" width="100%">
      <xsl:call-template name="head.sep.rule">
        <xsl:with-param name="pageclass" select="$pageclass"/>
        <xsl:with-param name="sequence" select="$sequence"/>
        <xsl:with-param name="gentext-key" select="$gentext-key"/>
      </xsl:call-template>
      <fo:table-column column-number="1" column-width="40%"/>
      <fo:table-column column-number="2" column-width="60%"/>
      <fo:table-body>
        <fo:table-row height="14pt">
          <fo:table-cell text-align="left" display-align="before">
            <xsl:attribute name="relative-align">baseline</xsl:attribute>
            <fo:block>
              <fo:block> </fo:block><!-- empty cell -->
            </fo:block>
          </fo:table-cell>
          <fo:table-cell text-align="center" display-align="before">
            <xsl:attribute name="relative-align">baseline</xsl:attribute>
            <fo:block>
              <xsl:call-template name="header.content">
                <xsl:with-param name="pageclass" select="$pageclass"/>
                <xsl:with-param name="sequence" select="$sequence"/>
                <xsl:with-param name="position" select="'center'"/>
                <xsl:with-param name="gentext-key" select="$gentext-key"/>
              </xsl:call-template>
            </fo:block>
          </fo:table-cell>
        </fo:table-row>
      </fo:table-body>
    </fo:table>
  </xsl:variable>
  <!-- Really output a header? -->
  <xsl:choose>
    <xsl:when test="$pageclass = 'titlepage' and $gentext-key = 'book'
                    and $sequence='first'">
      <!-- no, book titlepages have no headers at all -->
    </xsl:when>
    <xsl:when test="$sequence = 'blank' and $headers.on.blank.pages = 0">
      <!-- no output -->
    </xsl:when>
    <xsl:otherwise>
      <xsl:copy-of select="$candidate"/>
    </xsl:otherwise>
  </xsl:choose>
</xsl:template>


<!-- Bug-fix for Suse 10 PassiveTex version -->
<!-- Precompute attribute values 'cos PassiveTex is too stupid: -->
<xsl:attribute-set name="component.title.properties">
  <xsl:attribute name="keep-with-next.within-column">always</xsl:attribute>
  <xsl:attribute name="space-before.optimum">
    <xsl:value-of select="concat($body.font.master, 'pt')"/>
  </xsl:attribute>
  <xsl:attribute name="space-before.minimum">
    <xsl:value-of select="$body.font.master * 0.8"/>
    <xsl:text>pt</xsl:text>
  </xsl:attribute>
  <xsl:attribute name="space-before.maximum">
    <xsl:value-of select="$body.font.master * 1.2"/>
    <xsl:text>pt</xsl:text>
  </xsl:attribute>
  <xsl:attribute name="hyphenate">false</xsl:attribute>
</xsl:attribute-set>


</xsl:stylesheet>
