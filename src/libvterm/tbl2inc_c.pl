#!/usr/bin/perl

use strict;
use warnings;

my ( $encname ) = $ARGV[0] =~ m{/([^/.]+).tbl}
   or die "Cannot parse encoding name out of $ARGV[0]\n";

print <<"EOF";
static const struct StaticTableEncoding encoding_$encname = {
  {
    NULL, /* init */
    &decode_table /* decode */
  },
  {
EOF

my $row = 0;
while( <> ) {
   s/\s*#.*//; # strip comment

   if ($_ =~ m{^\d+/\d+}) {
     my ($up, $low) = ($_ =~ m{^(\d+)/(\d+)});
     my $thisrow = $up * 16 + $low;
     while ($row < $thisrow) {
	print "    0x0, /* $row */\n";
	++$row;
     }
   }

   s{^(\d+)/(\d+)}{""}e;                     # Remove 3/1
   s{ = }{""}e;                            # Remove " = "
   s{"(.)"}{sprintf "0x%04x", ord $1}e;      # Convert "A" to 0x41
   s{U\+}{0x};                               # Convert U+0041 to 0x0041

   s{$}{, /* $row */}; # append comma and index

   print "    $_";

   ++$row;
}

while ($row < 128) {
   print "    0x0, /* $row */\n";
   ++$row;
}

print <<"EOF";
  }
};
EOF
