#!/bin/dash
# sh4
Variable=${VariableB:-{VariableC}}
Variable=${VariableB:-${VariableC:-{Var3:=eng}}}

# This gets marked as an error while its ok
Variable=${VariableB:-${VariableC:-{Var3:=eng}}}
Variable=${VariableB:=${VariableC:={Var3:=${Var4:-eng}}}}
Variable=${VariableB:=${VariableC:={Var3:=${Var4:-${Var5:-eng}}}}}
Variable=${VariableB:=${VariableC:={Var3:=${Var4:-${Var5:-$Var6}}}}}

# These are OK
Variable="${VariableB:-${VariableC:-{Var3:=eng}}}"
Variable="${VariableB:=${VariableC:={Var3:=${Var4:-eng}}}}"

# This gets marked as an error too
: ${VariableB:-${VariableC:-{Var3:=eng}}}
: ${VariableB:=${VariableC:={Var3:=${Var4:-eng}}}}

# This is OK
: ${VariableB:-${VariableC:-eng}}
: "${VariableB:-${VariableC:-eng}}"

# First line is OK except its missing a closing "}",
# so second line should have some error highlighting
Variable=${VariableB:=${VariableC:={Var3:=${Var4:-eng}}}
Variable=${VariableB:-${VariableC:-{Var3:=eng}}
