$! test77a - help file creating checksum on VMS
$! Created by Zoltan Arpadffy
$
$ IF P1 .NES. ""
$ THEN
$    checksum 'P1'
$    show symb CHECKSUM$CHECKSUM
$ ENDIF
