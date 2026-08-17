#!/usr/bin/perl -w
#
# ------------------------------------------------------------------
# This file is part of bzip2/libbzip2, a program and library for
# lossless, block-sorting data compression.
#
# bzip2/libbzip2 version 1.0.8 of 13 July 2019
# Copyright (C) 1996-2019 Julian Seward <jseward@acm.org>
#
# Please read the WARNING, DISCLAIMER and PATENTS sections in the 
# README file.
#
# This program is released under the terms of the license contained
# in the file LICENSE.
# ------------------------------------------------------------------
#
use strict;

# get command line values:
if ( $#ARGV !=1 ) {
    die "Usage:  $0 xml_infile xml_outfile\n";
}

my $infile = shift;
# check infile exists
die "Can't find file \"$infile\""
  unless -f $infile;
# check we can read infile
if (! -r $infile) {
    die "Can't read input $infile\n";
}
# check we can open infile
open( INFILE,"<$infile" ) or 
    die "Can't input $infile $!";

#my $outfile = 'fmt-manual.xml';
my $outfile = shift;
#print "Infile: $infile, Outfile: $outfile\n";
# check we can write to outfile
open( OUTFILE,">$outfile" ) or 
    die "Can't output $outfile $! for writing";

my ($prev, $curr, $str);
$prev = ''; $curr = '';
while ( <INFILE> ) {

		print OUTFILE $prev;
    $prev = $curr;
    $curr = $_;
    $str = '';

    if ( $prev =~ /<programlisting>$|<screen>$/ ) {
        chomp $prev;
        $curr = join( '', $prev, "<![CDATA[", $curr );
				$prev = '';
        next;
    }
    elsif ( $curr =~ /<\/programlisting>|<\/screen>/ ) {
        chomp $prev;
        $curr = join( '', $prev, "]]>", $curr );
				$prev = '';
        next;
    }
}
print OUTFILE $curr;
close INFILE;
close OUTFILE;
exit;
