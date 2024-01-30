#!/bin/dash
# Test file to test 'for do done' loops.
# You can start this script like: $0 {-ne -gt -le ...} (all numeric operators 
# are allowed!

# All this works and should be OK

################################################################################
#
# For loop without 'in list'. Uses $1 $2 ... This is a special case!
# This 'for Var, do, done' is a very handy solution AND no real replacement 
# available!
#
Function1 () {

echo "Function1: for loop inside a function:\t\c"
[ "$*" ] || echo "none\c"

for Var
do
    [ 1 $Var 2 ] && echo "OK \c" || echo "no \c"
done
echo

} # End of Function1

################################################################################
#
# For loop with 'in list' $*
#
Function2 () {

echo "Function2: for loop inside a function:\t\c"
for Var in $*
do
    [ 1 $Var 2 ] && echo "OK \c" || echo "no \c"
done ; echo

} # End of Function2

################################################################################
#
# For loop with 'in list' $@. Works the same way as $*
#
Function3 () {

echo "Function3: for loop inside a function:\t\c"
for Var in $@
do
    [ 1 $Var 2 ] && echo "OK \c" || echo "no \c"
done ; echo

} # End of Function3

################################################################################
#
# For loop with 'in list' "$@". Special case. Works like "$1" "$2" ...
#
Function4 () {

echo "Function4: for loop inside a function:\t\c"
for Var in "$@"
do
    [ 1 $Var 2 ] && echo "OK \c" || echo "no \c"
done ; echo

} # End of Function4


################################################################################
# main ### main ### main ### main ### main ### main ### main ### main ### main #
################################################################################
#
# Here is the heart of this script:
#
echo "Processing the following command line arguments: ${*:-none}"
echo "Script:    for loop outside a function:\t\c"
for Var
do
    [ 1 $Var 2 ] && echo "OK \c" || echo "no \c"
done ; echo

# Same as function calls
Function1 -eq -ne -gt -ge -le -lt
Function2 -eq -ne -gt -ge -le -lt
Function3 -eq -ne -gt -ge -le -lt
Function4 -eq -ne -gt -ge -le -lt '-ge 1 -a 2 -ge'

# Now the same call like Function4 but with Function1
Function1 -eq -ne -gt -ge -le -lt '-ge 1 -a 2 -ge'
Function1

exit $?
