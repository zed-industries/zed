#! /usr/bin/perl -w
# -*- Perl -*-
#
# afblue.pl
#
# Process a blue zone character data file.
#
# Copyright (C) 2013-2023 by
# David Turner, Robert Wilhelm, and Werner Lemberg.
#
# This file is part of the FreeType project, and may only be used,
# modified, and distributed under the terms of the FreeType project
# license, LICENSE.TXT.  By continuing to use, modify, or distribute
# this file you indicate that you have read the license and
# understand and accept it fully.

use strict;
use warnings;
use English '-no_match_vars';
use open ':std', ':encoding(UTF-8)';


my $prog = $PROGRAM_NAME;
$prog =~ s| .* / ||x;      # Remove path.

die "usage: $prog datafile < infile > outfile\n" if $#ARGV != 0;


my $datafile = $ARGV[0];

my %diversions;        # The extracted and massaged data from `datafile'.
my @else_stack;        # Booleans to track else-clauses.
my @name_stack;        # Stack of integers used for names of aux. variables.

my $curr_enum;         # Name of the current enumeration.
my $curr_array;        # Name of the current array.
my $curr_max;          # Name of the current maximum value.

my $curr_enum_element; # Name of the current enumeration element.
my $curr_offset;       # The offset relative to current aux. variable.
my $curr_elem_size;    # The number of non-space characters in the current string or
                       # the number of elements in the current block.

my $have_sections = 0; # Boolean; set if start of a section has been seen.
my $have_strings;      # Boolean; set if current section contains strings.
my $have_blocks;       # Boolean; set if current section contains blocks.

my $have_enum_element; # Boolean; set if we have an enumeration element.
my $in_string;         # Boolean; set if a string has been parsed.

my $num_sections = 0;  # Number of sections seen so far.

my $last_aux;          # Name of last auxiliary variable.


# Regular expressions.

# [<ws>] <enum_name> <ws> <array_name> <ws> <max_name> [<ws>] ':' [<ws>] '\n'
my $section_re = qr/ ^ \s* (\S+) \s+ (\S+) \s+ (\S+) \s* : \s* $ /x;

# [<ws>] <enum_element_name> [<ws>] '\n'
my $enum_element_re = qr/ ^ \s* ( [A-Za-z0-9_]+ ) \s* $ /x;

# '#' <preprocessor directive> '\n'
my $preprocessor_re = qr/ ^ \# /x;

# [<ws>] '/' '/' <comment> '\n'
my $comment_re = qr| ^ \s* // |x;

# empty line
my $whitespace_only_re = qr/ ^ \s* $ /x;

# [<ws>] '"' <string> '"' [<ws>] '\n'  (<string> doesn't contain newlines)
my $string_re = qr/ ^ \s*
                       " ( (?> (?: (?> [^"\\]+ ) | \\. )* ) ) "
                       \s* $ /x;

# [<ws>] '{' <block> '}' [<ws>] '\n'  (<block> can contain newlines)
my $block_start_re = qr/ ^ \s* \{ /x;

# We need the capturing group for `split' to make it return the separator
# tokens (i.e., the opening and closing brace) also.
my $brace_re = qr/ ( [{}] ) /x;


sub Warn
{
  my $message = shift;
  warn "$datafile:$INPUT_LINE_NUMBER: warning: $message\n";
}


sub Die
{
  my $message = shift;
  die "$datafile:$INPUT_LINE_NUMBER: error: $message\n";
}


my $warned_before = 0;

sub warn_before
{
  Warn("data before first section gets ignored") unless $warned_before;
  $warned_before = 1;
}


sub strip_newline
{
  chomp;
  s/ \x0D $ //x;
}


sub end_curr_string
{
  # Append final null byte to string.
  if ($have_strings)
  {
    push @{$diversions{$curr_array}}, "    '\\0',\n" if $in_string;

    $curr_offset++;
    $in_string = 0;
  }
}


sub update_max_elem_size
{
  if ($curr_elem_size)
  {
    my $max = pop @{$diversions{$curr_max}};
    $max = $curr_elem_size if $curr_elem_size > $max;
    push @{$diversions{$curr_max}}, $max;
  }
}


sub convert_non_ascii_char
{
  # A UTF-8 character outside of the printable ASCII range, with possibly a
  # leading backslash character.
  my $s = shift;

  # Here we count characters, not bytes.
  $curr_elem_size += length $s;

  utf8::encode($s);
  $s = uc unpack 'H*', $s;

  $curr_offset += $s =~ s/\G(..)/'\\x$1', /sg;

  return $s;
}


sub convert_ascii_chars
{
  # A series of ASCII characters in the printable range.
  my $s = shift;

  # We reduce multiple space characters to a single one.
  $s =~ s/ +/ /g;

  # Count all non-space characters.  Note that `()' applies a list context
  # to the capture that is used to count the elements.
  $curr_elem_size += () = $s =~ /[^ ]/g;

  $curr_offset += $s =~ s/\G(.)/'$1', /g;

  return $s;
}


sub convert_literal
{
  my $s = shift;
  my $orig = $s;

  # ASCII printables and space
  my $safe_re = '\x20-\x7E';
  # ASCII printables and space, no backslash
  my $safe_no_backslash_re = '\x20-\x5B\x5D-\x7E';

  $s =~ s{
           (?: \\? ( [^$safe_re] )
               | ( (?: [$safe_no_backslash_re]
                       | \\ [$safe_re] )+ ) )
         }
         {
           defined($1) ? convert_non_ascii_char($1)
                       : convert_ascii_chars($2)
         }egx;

   # We assume that `$orig' doesn't contain `*/'
   return $s . " /* $orig */";
}


sub aux_name
{
  return "af_blue_" . $num_sections. "_" . join('_', @name_stack);
}


sub aux_name_next
{
  $name_stack[$#name_stack]++;
  my $name = aux_name();
  $name_stack[$#name_stack]--;

  return $name;
}


sub enum_val_string
{
  # Build string that holds code to save the current offset in an
  # enumeration element.
  my $aux = shift;

  my $add = ($last_aux eq "af_blue_" . $num_sections . "_0" )
              ? ""
              : "$last_aux + ";

  return "    $aux = $add$curr_offset,\n";
}



# Process data file.

open(DATA, $datafile) || die "$prog: can't open \`$datafile': $OS_ERROR\n";

while (<DATA>)
{
  strip_newline();

  next if /$comment_re/;
  next if /$whitespace_only_re/;

  if (/$section_re/)
  {
    Warn("previous section is empty") if ($have_sections
                                          && !$have_strings
                                          && !$have_blocks);

    end_curr_string();
    update_max_elem_size();

    # Save captured groups from `section_re'.
    $curr_enum = $1;
    $curr_array = $2;
    $curr_max = $3;

    $curr_enum_element = "";
    $curr_offset = 0;

    Warn("overwriting already defined enumeration \`$curr_enum'")
      if exists($diversions{$curr_enum});
    Warn("overwriting already defined array \`$curr_array'")
      if exists($diversions{$curr_array});
    Warn("overwriting already defined maximum value \`$curr_max'")
      if exists($diversions{$curr_max});

    $diversions{$curr_enum} = [];
    $diversions{$curr_array} = [];
    $diversions{$curr_max} = [];

    push @{$diversions{$curr_max}}, 0;

    @name_stack = ();
    push @name_stack, 0;

    $have_sections = 1;
    $have_strings = 0;
    $have_blocks = 0;

    $have_enum_element = 0;
    $in_string = 0;

    $num_sections++;
    $curr_elem_size = 0;

    $last_aux = aux_name();

    next;
  }

  if (/$preprocessor_re/)
  {
    if ($have_sections)
    {
      # Having preprocessor conditionals complicates the computation of
      # correct offset values.  We have to introduce auxiliary enumeration
      # elements with the name `af_blue_<s>_<n1>_<n2>_...' that store
      # offsets to be used in conditional clauses.  `<s>' is the number of
      # sections seen so far, `<n1>' is the number of `#if' and `#endif'
      # conditionals seen so far in the topmost level, `<n2>' the number of
      # `#if' and `#endif' conditionals seen so far one level deeper, etc.
      # As a consequence, uneven values are used within a clause, and even
      # values after a clause, since the C standard doesn't allow the
      # redefinition of an enumeration value.  For example, the name
      # `af_blue_5_1_6' is used to construct enumeration values in the fifth
      # section after the third (second-level) if-clause within the first
      # (top-level) if-clause.  After the first top-level clause has
      # finished, `af_blue_5_2' is used.  The current offset is then
      # relative to the value stored in the current auxiliary element.

      if (/ ^ \# \s* if /x)
      {
        push @else_stack, 0;

        $name_stack[$#name_stack]++;

        push @{$diversions{$curr_enum}}, enum_val_string(aux_name());
        $last_aux = aux_name();

        push @name_stack, 0;

        $curr_offset = 0;
      }
      elsif (/ ^ \# \s* elif /x)
      {
        Die("unbalanced #elif") unless @else_stack;

        pop @name_stack;

        push @{$diversions{$curr_enum}}, enum_val_string(aux_name_next());
        $last_aux = aux_name();

        push @name_stack, 0;

        $curr_offset = 0;
      }
      elsif (/ ^ \# \s* else /x)
      {
        my $prev_else = pop @else_stack;
        Die("unbalanced #else") unless defined($prev_else);
        Die("#else already seen") if $prev_else;
        push @else_stack, 1;

        pop @name_stack;

        push @{$diversions{$curr_enum}}, enum_val_string(aux_name_next());
        $last_aux = aux_name();

        push @name_stack, 0;

        $curr_offset = 0;
      }
      elsif (/ ^ (\# \s*) endif /x)
      {
        my $prev_else = pop @else_stack;
        Die("unbalanced #endif") unless defined($prev_else);

        pop @name_stack;

        # If there is no else-clause for an if-clause, we add one.  This is
        # necessary to have correct offsets.
        if (!$prev_else)
        {
          # Use amount of whitespace from `endif'.
          push @{$diversions{$curr_enum}}, enum_val_string(aux_name_next())
                                           . $1 . "else\n";
          $last_aux = aux_name();

          $curr_offset = 0;
        }

        $name_stack[$#name_stack]++;

        push @{$diversions{$curr_enum}}, enum_val_string(aux_name());
        $last_aux = aux_name();

        $curr_offset = 0;
      }

      # Handle (probably continued) preprocessor lines.
    CONTINUED_LOOP:
      {
        do
        {
          strip_newline();

          push @{$diversions{$curr_enum}}, $ARG . "\n";
          push @{$diversions{$curr_array}}, $ARG . "\n";

          last CONTINUED_LOOP unless / \\ $ /x;

        } while (<DATA>);
      }
    }
    else
    {
      warn_before();
    }

    next;
  }

  if (/$enum_element_re/)
  {
    end_curr_string();
    update_max_elem_size();

    $curr_enum_element = $1;
    $have_enum_element = 1;
    $curr_elem_size = 0;

    next;
  }

  if (/$string_re/)
  {
    if ($have_sections)
    {
      Die("strings and blocks can't be mixed in a section") if $have_blocks;

      # Save captured group from `string_re'.
      my $string = $1;

      if ($have_enum_element)
      {
        push @{$diversions{$curr_enum}}, enum_val_string($curr_enum_element);
        $have_enum_element = 0;
      }

      $string = convert_literal($string);

      push @{$diversions{$curr_array}}, "    $string\n";

      $have_strings = 1;
      $in_string = 1;
    }
    else
    {
      warn_before();
    }

    next;
  }

  if (/$block_start_re/)
  {
    if ($have_sections)
    {
      Die("strings and blocks can't be mixed in a section") if $have_strings;

      my $depth = 0;
      my $block = "";
      my $block_end = 0;

      # Count braces while getting the block.
    BRACE_LOOP:
      {
        do
        {
          strip_newline();

          foreach my $substring (split(/$brace_re/))
          {
            if ($block_end)
            {
              Die("invalid data after last matching closing brace")
                if $substring !~ /$whitespace_only_re/;
            }

            $block .= $substring;

            if ($substring eq '{')
            {
              $depth++;
            }
            elsif ($substring eq '}')
            {
              $depth--;

              $block_end = 1 if $depth == 0;
            }
          }

          # If we are here, we have run out of substrings, so get next line
          # or exit.
          last BRACE_LOOP if $block_end;

          $block .= "\n";

        } while (<DATA>);
      }

      if ($have_enum_element)
      {
        push @{$diversions{$curr_enum}}, enum_val_string($curr_enum_element);
        $have_enum_element = 0;
      }

      push @{$diversions{$curr_array}}, $block . ",\n";

      $curr_offset++;
      $curr_elem_size++;

      $have_blocks = 1;
    }
    else
    {
      warn_before();
    }

    next;
  }

  # Garbage.  We weren't able to parse the data.
  Die("syntax error");
}

# Finalize data.
end_curr_string();
update_max_elem_size();


# Filter stdin to stdout, replacing `@...@' templates.

sub emit_diversion
{
  my $diversion_name = shift;
  return (exists($diversions{$1})) ? "@{$diversions{$1}}"
                                   : "@" . $diversion_name . "@";
}


$LIST_SEPARATOR = '';

my $s1 = "This file has been generated by the Perl script \`$prog',";
my $s1len = length $s1;
my $s2 = "using data from file \`$datafile'.";
my $s2len = length $s2;
my $slen = ($s1len > $s2len) ? $s1len : $s2len;

print "/* " . $s1 . " " x ($slen - $s1len) . " */\n"
      . "/* " . $s2 . " " x ($slen - $s2len) . " */\n"
      . "\n";

while (<STDIN>)
{
  s/ @ ( [A-Za-z0-9_]+? ) @ / emit_diversion($1) /egx;
  print;
}

# EOF
