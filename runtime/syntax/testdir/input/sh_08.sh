#!/bin/ksh -p

# Note that this is special test file for ksh. sh is an extra file.
# Note too, that this file contains ONLY things which works for ksh BUT NOT
# for sh

# This all should be OK

# Several keywords without any quotes!
# Case 1a. Several Constants 
[ -t 0 ] && date
Variable1=${VariableA:-This is a Text}
Variable2=${VariableA:=This is a Text}
Variable3=${VariableA:?This is a Text}
echo "$Variable1" ; echo "$Variable2" ; echo "$Variable3"

# Case 1b. Variable and Constant
[ -t 0 ] && echo "\n`date`" && unset VariableA
Variable1=${VariableA:-$HOME This is a Text}
Variable2=${VariableA:=$HOME This is a Text}
Variable3=${VariableA:?$HOME This is a Text}
echo "$Variable1" ; echo "$Variable2" ; echo "$Variable3"

# Case 1c. Constant and Variable
[ -t 0 ] && echo "\n`date`" && unset VariableA
Variable1=${VariableA:-This is a Text in $HOME}
Variable2=${VariableA:=This is a Text in $HOME}
Variable3=${VariableA:+This is a Text in $HOME}       #! :+ is bash-only, error here expected
Variable1=${VariableA:-This is a Text in $HOME too}
Variable2=${VariableA:=This is a Text in $HOME too}
Variable3=${VariableA:+This is a Text in $HOME too}
echo "$Variable1" ; echo "$Variable2" ; echo "$Variable3"

# Case 1d. More Variables and Constants. Starting with a Variable.
[ -t 0 ] && echo "\n`date`" && unset VariableA
Variable1=${VariableA:-$SHELL}
Variable1=${VariableA:-$SHELL This is a Text in $HOME}
Variable2=${VariableA:=$SHELL This is a Text in $HOME}
Variable3=${VariableA:+$SHELL This is a Text in $HOME}
echo "$Variable1" ; echo "$Variable2" ; echo "$Variable3"

# Case 1e. More Constants and Variables. Starting with a Constant.
[ -t 0 ] && echo "\n`date`" && unset VariableA
Variable1=${VariableA:-"This is a Text in $HOME $SHELL"}
Variable1=${VariableA:-This is a Text in $HOME $SHELL}
Variable2=${VariableA:=This is a Text in $HOME $SHELL}
Variable3=${VariableA:+This is a Text in $HOME $SHELL}
echo "$Variable1" ; echo "$Variable2" ; echo "$Variable3"

# Case 1x. The same with ':'
[ -t 0 ] && echo "\n`date`" && unset VariableA
: ${VariableA:-This is a Text}
: ${VariableA:-$HOME This is a Text}
: ${VariableA:-This is a Text in $HOME}
: ${VariableA:-$SHELL This is a Text in $HOME}
: ${VariableA:-This is a Text in $HOME $SHELL}

# Case 1y. The same with ':' and without the ':' in the parameter substitution
[ -t 0 ] && echo "\n`date`" && unset VariableA
: ${VariableA-This is a Text}
: ${VariableA-$HOME This is a Text}
: ${VariableA-This is a Text in $HOME}
: ${VariableA-$SHELL This is a Text in $HOME}
: ${VariableA-This is a Text in $HOME $SHELL}

################################################################################
#
# This are valid usages for ${Var:?} in ksh!
#
Variable4=${Variable4:?This is an Error Message}
Variable4=${Variable4:?This is an Error Message from `date`}

: ${Variable4:?This is an Error Message}
: ${Variable4:?This is an Error Message from `date`}

exit $?

# Michael Soulier
if [ $# -ne 1 ]; then
	echo whatever
	exit 1
fi
