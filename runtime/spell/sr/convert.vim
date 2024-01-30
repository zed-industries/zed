:e sr.aff
:normal gg
:normal wgu$
:3d
:4d
:normal G
:normal o
:r sr-Latn.aff
:%s#^\(SFX\|PFX\).*[а-џa-ž]\zs$# .#g
:normal G
?SET
:.,+5d
:.,$s#^\(SFX\|PFX\) \zs\(\d\+\)#\= eval(submatch(2) .. ' + 1903')#
:w ../sr.aff
:bd!
:e sr.dic
:%s#a#а#g
:%s#e#е#g
:normal G
:normal o
:r sr-Latn.dic
:normal 201dd
:.,$s#/\zs\(\d\+\)\(,\(\d\+\)\)\?$#\=(submatch(2) == '') ? eval(submatch(1) + '1903') : eval(submatch(1) + '1903') .. ',' .. eval(submatch(3) + '1903')#
:normal {
:normal dd
:normal gg
:normal C502898
:w ../sr.dic
:bd!
:q!
