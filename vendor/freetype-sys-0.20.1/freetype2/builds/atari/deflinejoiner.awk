#!/usr/bin/env awk


function shift( array, \
                junk, elm0, l )
{
  elm0 = array[0]
  for ( l = 0; l < asorti( array, junk ) - 1; l++ )
    array[l] = array[l+1];
  delete array[l]
  return elm0
}


function init_cpp_src_line()
{
  logical_line = ""
  delete break_pos
}


function shift_valid_bp( array, \
                         junk, elm )
{
  elm = -1

  if ( 0 < asorti( array, junk ) )
    do {
      elm = shift( array )
    } while ( 0 > elm );

  return elm
}


function check_cpp_src_line_break_pos( \
                                       i, junk )
{
  printf( "break_pos:" )
  for ( i = 0; i < asorti( break_pos, junk ); i++ )
    printf( " %d", break_pos[i] );
  printf( "\n" )
}


function check_cpp_src_line()
{
  printf( "logical_line[%s]\n", logical_line )
  check_cpp_src_line_break_pos()
}


function append_line( phys_line, \
                      filt_line, bp_len )
{
  filt_line = phys_line
  sub( /\\$/, " ", filt_line )
  logical_line    = logical_line filt_line
  bp_len = asorti( break_pos, junk )
  break_pos[bp_len] = length( logical_line ) - 1
}


function print_line( \
                     c0, c1, i, junk, part_str )
{
  c0 = 0

  while( asorti( break_pos, junk ) > 1 )
  {
    if ( ( c1 = shift_valid_bp( break_pos ) ) < 1 )
    {
      part_str = substr( logical_line, c0 + 1 )
      printf( "%s\n", part_str )
      return
    }

    part_str = substr( logical_line, c0 + 1, c1 - c0 + 1 )
    gsub( / $/, "\\", part_str )
    printf( "%s\n", part_str )
    c0 = c1 + 1
  }

  part_str = substr( logical_line, c0 + 1 )
  printf( "%s\n", part_str )
}


function shrink_spaces( pos, \
                        tail, removed_length, k )
{
  tail = substr( logical_line, pos )
  sub( /^[ \t]+/, " ", tail )
  removed_length = length( logical_line ) - pos - length( tail ) + 1
  logical_line = substr( logical_line, 0, pos - 1 ) tail


  for ( k = 0; k < asorti( break_pos, junk ); k++ )
    if ( ( pos + removed_length ) <= break_pos[k] )
      break_pos[k] = break_pos[k] - removed_length;
    else if ( pos <= break_pos[k] )
      break_pos[k] = -1;

  return removed_length
}


function shrink_spaces_to_linebreak( pos, \
                                     junk, part_str, removed_length, i )
{
  for ( i = 0; i < asorti( break_pos, junk ) && break_pos[i] < pos ; i++ )
    ;

  if ( break_pos[i] < 1 )
    return;

  part_str = substr( logical_line, pos, break_pos[i] - pos + 1 )
  sub( /^[ \t]+/, " ", part_str )
  removed_length = ( break_pos[i] - pos + 1 ) - length( part_str )

  tail = substr( logical_line, pos + removed_length )
  logical_line = substr( logical_line, 0, pos - 1 ) tail

  for ( ; i < asorti( break_pos, junk ); i++ )
    break_pos[i] -= removed_length;

  return removed_length
}


function delete_linebreaks_in_2nd_token( \
                                           tail, paren_depth, junk, i, j, k, l )
{
  if ( logical_line ~ /^[ \t]*#[ \t]*define[ \t]+[0-9A-Za-z_]+\(/ )
  {
    tail = logical_line
    sub( /^[ \t]*#[ \t]*define[ \t]+[0-9A-Za-z_]+/, "", tail )

    paren_depth = 0
    l = 0
    i = length( logical_line ) - length( tail ) + 1 # seek to the 1st op paren
    j = i
    do {
      if ( substr( logical_line, j, 2 ) ~ /[ \t][ \t]/ )
        l = shrink_spaces( j );
      else if ( substr( logical_line, j, 1 ) == "(" )
        paren_depth += 1;
      else if ( substr( logical_line, j, 1 ) == ")" )
        paren_depth -= 1;
      j += 1
    } while ( j < length( logical_line ) && paren_depth != 0 )

    for ( k = 0; k < asorti( break_pos, junk ); k++ )
      if ( i <= break_pos[k] && break_pos[k] < j )
        break_pos[k] = -1;

    if ( l > 0 )
      shrink_spaces_to_linebreak( j );
  }
}


BEGIN{
  init_cpp_src_line()
}
{
  append_line( $0 )
  if ( $0 !~ /\\$/ )
  {
    delete_linebreaks_in_2nd_token()
    print_line()
    init_cpp_src_line()
  }
}
END{
  if ( 0 < length( logical_line ) )
  {
    delete_linebreaks_in_2nd_token()
    print_line()
  }
}
