data =: |: ". 'm' freads './data.txt'

NB. can also use `| -/` instead of `(>. - <.)`, though i do like the lil face.
a =: +/ (>. - <.)/ (/:~)"1 data

'left right' =: ;/ data
b =: +/ right * +/ left =/ right

echo a
echo b
exit ''
