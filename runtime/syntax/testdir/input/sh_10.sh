#!/bin/ksh

# This script is a test file for ksh93 shared-state
# command substitutions (subshares) and mksh value
# substitutions (valsubs).

# ======
# Below is subshare syntax supported by both ksh93 and mksh.
print ${ echo one }
print ${	echo two
}
print ${
echo three	}
print ${ echo 'four'; }
print ${ echo 'five' ;}
print ${ echo 'six'
}
print ${	echo 'seven'	}
echo ${ print 'eight'	}
typeset nine=${ pwd; }

# ======
# Value substitutions of the form ${|command} are only
# supported by mksh, not ksh93.
if ! command eval '((.sh.version >= 20070703))' 2>/dev/null; then
	valsubfunc() {
		REPLY=$1
	}
	echo ${|valsubfunc ten}
	print "${|valsubfunc eleven;}"
	printf '%s' "${|valsubfunc twelve	}"
	unlucky=${|valsubfunc thirteen
}
	typeset notafloat=${|valsubfunc notanumber	}
	print $unlucky $notanumber
	${|echo foo}
	${|echo bar
}
fi

# ======
# Shared-state command substitutions using the syntax ${<file;}
# are only supported by ksh93, not mksh.
echo ${
	printf %s str
} > /tmp/strfile
echo ${</tmp/strfile;}

exit 0
