#!/bin/ksh
#  Test variable modifiers
# Variable=value
Variable='value'
Variable="value"
VariableA="pat1xxpat2"
VariableB="pat2xxpat1"
echo ${#}
echo ${#VariableA}
echo ${VariableA#pat1}
echo ${VariableA##pat1}
echo ${VariableB%pat1}
echo ${VariableB%%pat1}

# This gets marked as an error
Variable=${VariableB:+${VariableC:=eng}}	# :+ seems to work for ksh as well as bash
Variable=${VariableB:-${VariableC:-eng}}	# :- is ksh and bash

# This is OK
Variable='${VariableB:+${VariableC:=eng}}'
Variable='${VariableB:-${VariableC:-eng}}'
Variable="${VariableB:+${VariableC:=eng}}"	# :+ seems to work for ksh as well as bash
Variable="${VariableB:-${VariableC:-eng}}"  # :- is ksh and bash

# These are OK
: ${VariableB:-${VariableC:-eng}}
: "${VariableB:-${VariableC:-eng}}"
: '${VariableB:-${VariableC:-eng}}'

# Another test
Variable=${VariableB:-${VariableC:-${VariableD:-${VariableE:=eng}}}}
       :        ${VariableB:=${VariableC:-${VariableD:-${VariableE:=eng}}}}

