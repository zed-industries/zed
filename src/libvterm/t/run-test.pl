#!/usr/bin/perl

use strict;
use warnings;
use Getopt::Long;
use IO::Handle;
use IPC::Open2 qw( open2 );
use POSIX qw( WIFEXITED WEXITSTATUS WIFSIGNALED WTERMSIG );

my $VALGRIND = 0;
my $EXECUTABLE = "t/harness";
GetOptions(
   'valgrind|v+' => \$VALGRIND,
   'executable|e=s' => \$EXECUTABLE,
   'fail-early|F' => \(my $FAIL_EARLY),
) or exit 1;

my ( $hin, $hout, $hpid );
{
   my @command = $EXECUTABLE;
   unshift @command, "valgrind", "--tool=memcheck", "--leak-check=yes", "--num-callers=25", "--log-file=valgrind.out", "--error-exitcode=126" if $VALGRIND;

   $hpid = open2 $hout, $hin, @command or die "Cannot open2 harness - $!";
}

my $exitcode = 0;

my $command;
my @expect;

my $linenum = 0;

sub do_onetest
{
   $hin->print( "$command\n" );
   undef $command;

   my $fail_printed = 0;

   while( my $outline = <$hout> ) {
      last if $outline eq "DONE\n" or $outline eq "?\n";

      chomp $outline;

      if( !@expect ) {
         print "# line $linenum: Test failed\n" unless $fail_printed++;
         print "#    expected nothing more\n" .
               "#   Actual:   $outline\n";
         next;
      }

      my $expectation = shift @expect;

      next if $expectation eq $outline;

      print "# line $linenum: Test failed\n" unless $fail_printed++;
      print "#   Expected: $expectation\n" .
            "#   Actual:   $outline\n";
   }

   if( @expect ) {
      print "# line $linenum: Test failed\n" unless $fail_printed++;
      print "#   Expected: $_\n" .
            "#    didn't happen\n" for @expect;
   }

   $exitcode = 1 if $fail_printed;
   exit $exitcode if $exitcode and $FAIL_EARLY;
}

sub do_line
{
   my ( $line ) = @_;

   if( $line =~ m/^!(.*)/ ) {
      do_onetest if defined $command;
      print "> $1\n";
   }

   # Commands have capitals
   elsif( $line =~ m/^([A-Z]+)/ ) {
      # Some convenience formatting
      if( $line =~ m/^(PUSH|ENCIN) (.*)$/ ) {
         # we're evil
         my $string = eval($2);
         $line = "$1 " . unpack "H*", $string;
      }
      elsif( $line =~ m/^(SELECTION \d+) +(\[?)(.*?)(\]?)$/ ) {
         # we're evil
         my $string = eval($3);
         $line = "$1 $2 " . unpack( "H*", $string ) . " $4";
      }

      do_onetest if defined $command;

      $command = $line;
      undef @expect;
   }
   # Expectations have lowercase
   elsif( $line =~ m/^([a-z]+)/ ) {
      # Convenience formatting
      if( $line =~ m/^(text|encout) (.*)$/ ) {
         $line = "$1 " . join ",", map sprintf("%x", $_), eval($2);
      }
      elsif( $line =~ m/^(output) (.*)$/ ) {
         $line = "$1 " . join ",", map sprintf("%x", $_), unpack "C*", eval($2);
      }
      elsif( $line =~ m/^control (.*)$/ ) {
         $line = sprintf "control %02x", eval($1);
      }
      elsif( $line =~ m/^csi (\S+) (.*)$/ ) {
         $line = sprintf "csi %02x %s", eval($1), $2; # TODO
      }
      elsif( $line =~ m/^(osc) (\[\d+)? *(.*?)(\]?)$/ ) {
         my ( $cmd, $initial, $data, $final ) = ( $1, $2, $3, $4 );
         $initial //= "";
         $initial .= ";" if $initial =~ m/\d+/;

         $line = "$cmd $initial" . join( "", map sprintf("%02x", $_), unpack "C*", length $data ? eval($data) : "" ) . "$final";
      }
      elsif( $line =~ m/^(escape|dcs|apc|pm|sos) (\[?)(.*?)(\]?)$/ ) {
         $line = "$1 $2" . join( "", map sprintf("%02x", $_), unpack "C*", length $3 ? eval($3) : "" ) . "$4";
      }
      elsif( $line =~ m/^putglyph (\S+) (.*)$/ ) {
         $line = "putglyph " . join( ",", map sprintf("%x", $_), eval($1) ) . " $2";
      }
      elsif( $line =~ m/^(?:movecursor|scrollrect|moverect|erase|damage|sb_pushline|sb_popline|sb_clear|settermprop|setmousefunc|selection-query) ?/ ) {
         # no conversion
      }
      elsif( $line =~ m/^(selection-set) (.*?) (\[?)(.*?)(\]?)$/ ) {
         $line = "$1 $2 $3" . join( "", map sprintf("%02x", $_), unpack "C*", eval($4) ) . "$5";
      }
      else {
         warn "Unrecognised test expectation '$line'\n";
      }

      push @expect, $line;
   }
   # ?screen_row assertion is emulated here
   elsif( $line =~ s/^\?screen_row\s+(\d+)\s*=\s*// ) {
      my $row = $1;
      my $want;

      if( $line =~ m/^"/ ) {
         $want = eval($line);
      }
      else {
         # Turn 0xDD,0xDD,... directly into bytes
         $want = pack "C*", map { hex } split m/,/, $line;
      }

      do_onetest if defined $command;

      $hin->print( "\?screen_chars $row\n" );
      my $response = <$hout>;
      chomp $response;

      $response = pack "C*", map { hex } split m/,/, $response;
      if( $response ne $want ) {
         print "# line $linenum: Assert ?screen_row $row failed:\n" .
               "# Expected: $want\n" .
               "# Actual:   $response\n";
         $exitcode = 1;
         exit $exitcode if $exitcode and $FAIL_EARLY;
      }
   }
   # Assertions start with '?'
   elsif( $line =~ s/^\?([a-z]+.*?=)\s*// ) {
      do_onetest if defined $command;

      my ( $assertion ) = $1 =~ m/^(.*)\s+=/;
      my $expectation = $line;

      $hin->print( "\?$assertion\n" );
      my $response = <$hout>; defined $response or wait, die "Test harness failed - $?\n";
      chomp $response; $response =~ s/^\s+|\s+$//g;

      # Some convenience formatting
      if( $assertion =~ m/^screen_chars/ and $expectation =~ m/^"/ ) {
         $expectation = join ",", map sprintf("0x%02x", ord $_), split m//, eval($expectation);
      }

      if( $response ne $expectation ) {
         print "# line $linenum: Assert $assertion failed:\n" .
               "# Expected: $expectation\n" .
               "# Actual:   $response\n";
         $exitcode = 1;
         exit $exitcode if $exitcode and $FAIL_EARLY;
      }
   }
   # Test controls start with '$'
   elsif( $line =~ s/\$SEQ\s+(\d+)\s+(\d+):\s*// ) {
      my ( $low, $high ) = ( $1, $2 );
      foreach my $val ( $low .. $high ) {
         ( my $inner = $line ) =~ s/\\#/$val/g;
         do_line( $inner );
      }
   }
   elsif( $line =~ s/\$REP\s+(\d+):\s*// ) {
      my $count = $1;
      do_line( $line ) for 1 .. $count;
   }
   else {
      die "Unrecognised TEST line $line\n";
   }
}

open my $test, "<", $ARGV[0] or die "Cannot open test script $ARGV[0] - $!";

while( my $line = <$test> ) {
   $linenum++;
   $line =~ s/^\s+//;
   chomp $line;

   next if $line =~ m/^(?:#|$)/;
   last if $line eq "__END__";

   do_line( $line );
}

do_onetest if defined $command;

close $hin;
close $hout;

waitpid $hpid, 0;
if( $? ) {
   printf STDERR "Harness exited %d\n", WEXITSTATUS($?)   if WIFEXITED($?);
   printf STDERR "Harness exit signal %d\n", WTERMSIG($?) if WIFSIGNALED($?);
   $exitcode = WIFEXITED($?) ? WEXITSTATUS($?) : 125;
}

exit $exitcode;
