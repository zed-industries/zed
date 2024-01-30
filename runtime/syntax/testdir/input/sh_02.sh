#! /bin/ksh
# sh2
#  Jul 28, 2018: introduced shCommandSubBQ, which is *not* included in a shSubCommandList (so its not recursive)
ccc=`echo "test"`
	ccc=`echo "test"`

# comment
case $VAR in
# comment
	x|y|z) echo xyz ;;
# comment
	a|b|c) echo abc ;;
# comment
esac

# Jul 26, 2018: why isn't `..` being terminated properly?
# comment
case "$aaa" in
# comment
  	bbb)  ccc=`echo $ddd|cut -b4-`
	echo "test"
# comment
	;;
# comment
	esac
# comment

echo   $VAR abc
export $VAR abc
set    $VAR abc
