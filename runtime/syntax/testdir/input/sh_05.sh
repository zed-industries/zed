#!/bin/dash -x
# sh5
# Note that this is special for sh. ksh will be an extra file later.
# Note too, that sh and ksh allow ${var:-sub} as well as ${var-sub}!
# The ':' is optional!

# This all should be OK
# Case 0a
[ -t 0 ] && date
Variable1=value1
Variable2='value2'
Variable3="value3"
echo "$Variable1" "$Variable2" "$Variable3"

# Case 0b
[ -t 0 ] && echo "\ndate"
Variable1=$HOME
Variable2='$HOME'
Variable3="$HOME"
echo "$Variable1" "$Variable2" "$Variable3"

# Case 0c
[ -t 0 ] && echo "\ndate"
Variable1=$HOME$SHELL
Variable2=$HOME.$SHELL
Variable3=$HOME.$SHELL+$HOME-$SHELL/$HOME
echo "$Variable1" "$Variable2" "$Variable3"

# Case 0d
[ -t 0 ] && echo "\ndate"
Variable1=`date`
Variable2=`id -ng`
Variable3=`id -ng | wc -c`
echo "$Variable1" "$Variable2" "$Variable3"

################################################################################
#
# Case 1a with constants
[ -t 0 ] && echo "\ndate"
Variable1=${VariableA:=eng1}
Variable2=${VariableA:-eng2}
Variable3=${VariableA:?eng3}
Variable3=${VariableA:+eng3}
echo "$Variable1" "$Variable2" "$Variable3"

# Case 1b with constants in single quotes
[ -t 0 ] && echo "\ndate"
Variable1=${VariableA:='eng1'}
Variable2=${VariableA:-'eng2'}
Variable3=${VariableA:?'eng3'}
Variable3=${VariableA:+'eng3'}
echo "$Variable1" "$Variable2" "$Variable3"

# Case 1c with constants in double quotes
[ -t 0 ] && echo "\ndate"
Variable1=${VariableA:="eng1"}
Variable2=${VariableA:-"eng2"}
Variable3=${VariableA:?"eng3"}
Variable3=${VariableA:+"eng3"}
echo "$Variable1" "$Variable2" "$Variable3"

# Case 1d: constants but missing colons
Variable1=${VariableA=eng1}
Variable2=${VariableA-eng2}
Variable3=${VariableA?eng3}
Variable3=${VariableA+eng3}
Variable1=${VariableA='eng1'}
Variable2=${VariableA-'eng2'}
Variable3=${VariableA?'eng3'}
Variable3=${VariableA+'eng3'}
Variable1=${VariableA="eng1"}
Variable2=${VariableA-"eng2"}
Variable3=${VariableA?"eng3"}
Variable3=${VariableA+"eng3"}

# Case 2a with a variable
[ -t 0 ] && echo "\ndate"
Variable1=${VariableA:=$HOME}
Variable2=${VariableA:-$HOME}
Variable3=${VariableA:?$HOME}
Variable3=${VariableA:+$HOME}
echo "$Variable1" "$Variable2" "$Variable3"

# Case 2b with a variable in single quotes
[ -t 0 ] && echo "\ndate"
Variable1=${VariableA:='$HOME'}
Variable2=${VariableA:-'$HOME'}
Variable3=${VariableA:?'$HOME'}
Variable3=${VariableA:+'$HOME'}
echo "$Variable1" "$Variable2" "$Variable3"

# Case 2c with a variable in double quotes
[ -t 0 ] && echo "\ndate"
Variable1=${VariableA:="$HOME"}
Variable2=${VariableA:-"$HOME"}
Variable3=${VariableA:?"$HOME"}
Variable3=${VariableA:+"$HOME"}
echo "$Variable1" "$Variable2" "$Variable3"

# Case 3a with a command substitution
[ -t 0 ] && echo "\ndate"
Variable1=${VariableA:=`date`}
Variable2=${VariableA:-`date`}
Variable3=${VariableA:?`date`}
Variable3=${VariableA:+`date`}
echo "$Variable1" "$Variable2" "$Variable3"

# Case 3b with a command + option substitution
[ -t 0 ] && echo "\ndate"
Variable1=${VariableA:=`id -ng`}
Variable2=${VariableA:-`id -ng`}
Variable3=${VariableA:?`id -ng`}
Variable3=${VariableA:+`id -ng`}
echo "$Variable1" "$Variable2" "$Variable3"

# Case 3c with a command + pipe substitution
[ -t 0 ] && echo "\ndate"
Variable1=${VariableA:=`id -ng | wc -c`}
Variable2=${VariableA:-`id -ng | wc -c`}
Variable3=${VariableA:?`id -ng | wc -c`}
Variable3=${VariableA:+`id -ng | wc -c`}
echo "$Variable1" "$Variable2" "$Variable3"

################################################################################
#
# The same with one nestet ${} level
# Case 1a with constants
[ -t 0 ] && echo "\ndate"
Variable1=${VariableA:-${VarB:=eng1}}
Variable2=${VariableA:-${VarB:-eng2}}
Variable3=${VariableA:-${VarB:?eng3}}
Variable3=${VariableA:-${VarB:+eng3}}
echo "$Variable1" "$Variable2" "$Variable3"

# Case 1b with constants in single quotes
[ -t 0 ] && echo "\ndate"
Variable1=${VariableA:-${VarB:='eng1'}}
Variable2=${VariableA:-${VarB:-'eng2'}}
Variable3=${VariableA:-${VarB:?'eng3'}}
Variable3=${VariableA:-${VarB:+'eng3'}}
echo "$Variable1" "$Variable2" "$Variable3"

# Case 1c with constants in double quotes
[ -t 0 ] && echo "\ndate"
Variable1=${VariableA:-${VarB:="eng1"}}
Variable2=${VariableA:-${VarB:-"eng2"}}
Variable3=${VariableA:-${VarB:?"eng3"}}
Variable3=${VariableA:-${VarB:+"eng3"}}
echo "$Variable1" "$Variable2" "$Variable3"

# Case 2a with a variable
[ -t 0 ] && echo "\ndate"
Variable1=${VariableA:-${VarB:=$HOME}}
Variable2=${VariableA:-${VarB:-$HOME}}
Variable3=${VariableA:-${VarB:?$HOME}}
Variable3=${VariableA:-${VarB:+$HOME}}
echo "$Variable1" "$Variable2" "$Variable3"

# Case 2b with a variable in single quotes
[ -t 0 ] && echo "\ndate"
Variable1=${VariableA:-${VarB:='$HOME'}}
Variable2=${VariableA:-${VarB:-'$HOME'}}
Variable3=${VariableA:-${VarB:?'$HOME'}}
Variable3=${VariableA:-${VarB:+'$HOME'}}
echo "$Variable1" "$Variable2" "$Variable3"

# Case 2c with a variable in double quotes
[ -t 0 ] && echo "\ndate"
Variable1=${VariableA:-${VarB:="$HOME"}}
Variable2=${VariableA:-${VarB:-"$HOME"}}
Variable3=${VariableA:-${VarB:?"$HOME"}}
Variable3=${VariableA:-${VarB:+"$HOME"}}
echo "$Variable1" "$Variable2" "$Variable3"

# Case 3a with a command substitution
[ -t 0 ] && echo "\ndate"
Variable1=${VariableA:-${VarB:=`date`}}
Variable2=${VariableA:-${VarB:-`date`}}
Variable3=${VariableA:-${VarB:?`date`}}
Variable3=${VariableA:-${VarB:+`date`}}
echo "$Variable1" "$Variable2" "$Variable3"

# Case 3b with a command + option substitution
[ -t 0 ] && echo "\ndate"
Variable1=${VariableA:-${VarB:=`id -ng`}}
Variable2=${VariableA:-${VarB:-`id -ng`}}
Variable3=${VariableA:-${VarB:?`id -ng`}}
Variable3=${VariableA:-${VarB:+`id -ng`}}
echo "$Variable1" "$Variable2" "$Variable3"

# Case 3c with a command + pipe substitution
[ -t 0 ] && echo "\ndate"
Variable1=${VariableA:-${VarB:=`id -ng | wc -c`}}
Variable2=${VariableA:-${VarB:-`id -ng | wc -c`}}
Variable3=${VariableA:-${VarB:?`id -ng | wc -c`}}
Variable3=${VariableA:-${VarB:+`id -ng | wc -c`}}
echo "$Variable1" "$Variable2" "$Variable3"

################################################################################
#
# The same with two nestet ${} level
# Case 1a with constants
[ -t 0 ] && echo "\ndate"
Variable1=${VariableA:-${VarB:-${VarC:=eng1}}}
Variable2=${VariableA:-${VarB:-${VarC:-eng2}}}
Variable3=${VariableA:-${VarB:-${VarC:?eng3}}}
Variable3=${VariableA:-${VarB:-${VarC:+eng3}}}
echo "$Variable1" "$Variable2" "$Variable3"

# Case 1b with constants in single quotes
[ -t 0 ] && echo "\ndate"
Variable1=${VariableA:-${VarB:-${VarC:='eng1'}}}
Variable2=${VariableA:-${VarB:-${VarC:-'eng2'}}}
Variable3=${VariableA:-${VarB:-${VarC:?'eng3'}}}
Variable3=${VariableA:-${VarB:-${VarC:+'eng3'}}}
echo "$Variable1" "$Variable2" "$Variable3"

# Case 1c with constants in double quotes
[ -t 0 ] && echo "\ndate"
Variable1=${VariableA:-${VarB:-${VarC:="eng1"}}}
Variable2=${VariableA:-${VarB:-${VarC:-"eng2"}}}
Variable3=${VariableA:-${VarB:-${VarC:?"eng3"}}}
Variable3=${VariableA:-${VarB:-${VarC:+"eng3"}}}
echo "$Variable1" "$Variable2" "$Variable3"

# Case 2a with a variable
[ -t 0 ] && echo "\ndate"
Variable1=${VariableA:-${VarB:-${VarC:=$HOME}}}
Variable2=${VariableA:-${VarB:-${VarC:-$HOME}}}
Variable3=${VariableA:-${VarB:-${VarC:?$HOME}}}
Variable3=${VariableA:-${VarB:-${VarC:+$HOME}}}
echo "$Variable1" "$Variable2" "$Variable3"

# Case 2b with a variable in single quotes
[ -t 0 ] && echo "\ndate"
Variable1=${VariableA:-${VarB:-${VarC:='$HOME'}}}
Variable2=${VariableA:-${VarB:-${VarC:-'$HOME'}}}
Variable3=${VariableA:-${VarB:-${VarC:?'$HOME'}}}
Variable3=${VariableA:-${VarB:-${VarC:+'$HOME'}}}
echo "$Variable1" "$Variable2" "$Variable3"

# Case 2c with a variable in double quotes
[ -t 0 ] && echo "\ndate"
Variable1=${VariableA:-${VarB:-${VarC:="$HOME"}}}
Variable2=${VariableA:-${VarB:-${VarC:-"$HOME"}}}
Variable3=${VariableA:-${VarB:-${VarC:?"$HOME"}}}
Variable3=${VariableA:-${VarB:-${VarC:?"$HOME"}}}
Variable3=${VariableA:-${VarB:-${VarC:+"$HOME"}}}
echo "$Variable1" "$Variable2" "$Variable3"

# Case 3a with a command substitution
[ -t 0 ] && echo "\ndate"
Variable1=${VariableA:-${VarB:-${VarC:=`date`}}}
Variable2=${VariableA:-${VarB:-${VarC:-`date`}}}
Variable3=${VariableA:-${VarB:-${VarC:?`date`}}}
Variable3=${VariableA:-${VarB:-${VarC:+`date`}}}
echo "$Variable1" "$Variable2" "$Variable3"

# Case 3b with a command + option substitution
[ -t 0 ] && echo "\ndate"
Variable1=${VariableA:-${VarB:-${VarC:=`id -ng`}}}
Variable2=${VariableA:-${VarB:-${VarC:-`id -ng`}}}
Variable3=${VariableA:-${VarB:-${VarC:?`id -ng`}}}
Variable3=${VariableA:-${VarB:-${VarC:+`id -ng`}}}
echo "$Variable1" "$Variable2" "$Variable3"

# Case 3c with a command + pipe substitution
[ -t 0 ] && echo "\ndate"
Variable1=${VariableA:-${VarB:-${VarC:=`id -ng | wc -c`}}}
Variable2=${VariableA:-${VarB:-${VarC:-`id -ng | wc -c`}}}
Variable3=${VariableA:-${VarB:-${VarC:?`id -ng | wc -c`}}}
Variable3=${VariableA:-${VarB:-${VarC:+`id -ng | wc -c`}}}
echo "$Variable1" "$Variable2" "$Variable3"


################################################################################
#
# The same with three nestet ${} level
# Case 1a with constants
[ -t 0 ] && echo "\ndate"
Variable1=${VariableA:-${VarB:-${VarC:-${VarD:=eng1}}}}
Variable2=${VariableA:-${VarB:-${VarC:-${VarD:-eng2}}}}
Variable3=${VariableA:-${VarB:-${VarC:-${VarD:?eng3}}}}
Variable3=${VariableA:-${VarB:-${VarC:-${VarD:+eng3}}}}
echo "$Variable1" "$Variable2" "$Variable3"

# Case 1b with constants in single quotes
[ -t 0 ] && echo "\ndate"
Variable1=${VariableA:-${VarB:-${VarC:-${VarD:='eng1'}}}}
Variable2=${VariableA:-${VarB:-${VarC:-${VarD:-'eng2'}}}}
Variable3=${VariableA:-${VarB:-${VarC:-${VarD:?'eng3'}}}}
Variable3=${VariableA:-${VarB:-${VarC:-${VarD:+'eng3'}}}}
echo "$Variable1" "$Variable2" "$Variable3"

# Case 1c with constants in double quotes
[ -t 0 ] && echo "\ndate"
Variable1=${VariableA:-${VarB:-${VarC:-${VarD:="eng1"}}}}
Variable2=${VariableA:-${VarB:-${VarC:-${VarD:-"eng2"}}}}
Variable3=${VariableA:-${VarB:-${VarC:-${VarD:?"eng3"}}}}
Variable3=${VariableA:-${VarB:-${VarC:-${VarD:+"eng3"}}}}
echo "$Variable1" "$Variable2" "$Variable3"

# Case 2a with a variable
[ -t 0 ] && echo "\ndate"
Variable1=${VariableA:-${VarB:-${VarC:-${VarD:=$HOME}}}}
Variable2=${VariableA:-${VarB:-${VarC:-${VarD:-$HOME}}}}
Variable3=${VariableA:-${VarB:-${VarC:-${VarD:?$HOME}}}}
Variable3=${VariableA:-${VarB:-${VarC:-${VarD:+$HOME}}}}
echo "$Variable1" "$Variable2" "$Variable3"

# Case 2b with a variable in single quotes
[ -t 0 ] && echo "\ndate"
Variable1=${VariableA:-${VarB:-${VarC:-${VarD:='$HOME'}}}}
Variable2=${VariableA:-${VarB:-${VarC:-${VarD:-'$HOME'}}}}
Variable3=${VariableA:-${VarB:-${VarC:-${VarD:?'$HOME'}}}}
Variable3=${VariableA:-${VarB:-${VarC:-${VarD:+'$HOME'}}}}
echo "$Variable1" "$Variable2" "$Variable3"

# Case 2c with a variable in double quotes
[ -t 0 ] && echo "\ndate"
Variable1=${VariableA:-${VarB:-${VarC:-${VarD:="$HOME"}}}}
Variable2=${VariableA:-${VarB:-${VarC:-${VarD:-"$HOME"}}}}
Variable3=${VariableA:-${VarB:-${VarC:-${VarD:?"$HOME"}}}}
Variable3=${VariableA:-${VarB:-${VarC:-${VarD:+"$HOME"}}}}
echo "$Variable1" "$Variable2" "$Variable3"

# Case 3a with a command substitution
[ -t 0 ] && echo "\ndate"
Variable1=${VariableA:-${VarB:-${VarC:-${VarD:=`date`}}}}
Variable2=${VariableA:-${VarB:-${VarC:-${VarD:-`date`}}}}
Variable3=${VariableA:-${VarB:-${VarC:-${VarD:?`date`}}}}
Variable3=${VariableA:-${VarB:-${VarC:-${VarD:+`date`}}}}
echo "$Variable1" "$Variable2" "$Variable3"

# Case 3b with a command + option substitution
[ -t 0 ] && echo "\ndate"
Variable1=${VariableA:-${VarB:-${VarC:-${VarD:=`id -ng`}}}}
Variable2=${VariableA:-${VarB:-${VarC:-${VarD:-`id -ng`}}}}
Variable3=${VariableA:-${VarB:-${VarC:-${VarD:?`id -ng`}}}}
Variable3=${VariableA:-${VarB:-${VarC:-${VarD:+`id -ng`}}}}
echo "$Variable1" "$Variable2" "$Variable3"

# Case 3c with a command + pipe substitution
[ -t 0 ] && echo "\ndate"
Variable1=${VariableA:-${VarB:-${VarC:-${VarD:=`id -ng | wc -c`}}}}
Variable2=${VariableA:-${VarB:-${VarC:-${VarD:-`id -ng | wc -c`}}}}
Variable3=${VariableA:-${VarB:-${VarC:-${VarD:?`id -ng | wc -c`}}}}
Variable3=${VariableA:-${VarB:-${VarC:-${VarD:+`id -ng | wc -c`}}}}
echo "$Variable1" "$Variable2" "$Variable3"


################################################################################
#
# This is also allowed:
Variable1=${VariableA-${VarB-${VarC-${VarD=`id -ng | wc -c`}}}}

################################################################################
#
# All cases with ${Var:?} which works for the sh:
Variable4=${Variable4:?}
Variable4=${Variable4:?OK}
Variable4=${Variable4:?`date`}
Variable4=${Variable4:?'an OK string'}
Variable4=${Variable4:?"an OK string"}
Variable4=${Variable4:?$HOME$SHELL}
Variable4=${Variable4:?$HOME:$SHELL}

# All cases with ${Var:?} which works also for ksh:
Variable4=${Variable4:?This is OK}
Variable4=${Variable4:?This is OK, too: `date`}

# What happens with ${#identifier[*]}:
Variable5=${#identifier[*]}
