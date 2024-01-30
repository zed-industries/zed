#!/bin/ksh
# Shall we debug this script? If so, remove the '#' before '#DebugScript=set'
DebugScript=set

# Show that we are busy.
[ -t 0 ] && echo "Please wait ... \c" >`tty`

################################################################################
#
# Display some Help
#
Usage () {

# does this comment work?
VariableName="${BasicConfigName}_*"

# Echo some ksh special variables
echo "CDPATH="${CDPATH}

# Get also a short description of the backuptype/method
eval BackupMethod=\$mess09${BackupType}B

case $BackupType in
  3)   DefaultDevice=$MountDevice    ;;
  1|2) DefaultDevice=$TapeDrive      ;;
esac

# If we have more the 53 characters in the variables below split them up
# into several lines and add 3 tabs before them
for Variable in DefaultExclude DefaultFindOption DoNotBackupList
do
    eval VarValue=\$$Variable
    VarValue=`echo $VarValue | FoldS 53 | sed "2,\\$s/^/$Tab$Tab$Tab/"`
    eval $Variable=\$VarValue
done

echo "
Usage:  $ScriptName [-Options]

Options List:
        -v              The current version of '$ScriptName'
        -h  | -H | ?    Display this list

"

} # End of Usage


################################################################################
#
# Create a backup using fbackup/frecover
#
ExecuteFbackup () { # TESTING

[ "$DebugScript" ]    && set -x || set +x

cd $cwd

} # End of ExecuteFbackup


################################################################################
# main ### main ### main ### main ### main ### main ### main ### main ### main #
################################################################################
#
# Here is the heart of this script:
#
Usage

# And exit
Exit $Result
