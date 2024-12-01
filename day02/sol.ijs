data =: _".&.> 'b' freads './data.txt'

NB. @> fucks ngl
NB. check and a were written before i knew about infixes.
check =: [: */ [: ((>&0) * (<:&3)) [: ([ * *&{.) [: }. (- |.!.0)
a =: [: +/ check@>
b =: [: +/ {{>./ check"1 (1]\. y)}}@>

echo a
echo b
exit ''
