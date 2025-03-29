: meter 100 * ;
: decimeter 10 * ;
: centimeter 1 * ;
1 meter 5 decimeter 2 centimeter + +
: seconds 1 * ;
: minutes 60 * seconds ;
: hours 60 * minutes ;
2 hours 13 minutes 5 seconds + +
: one1 1 ;
: one2  one1 one1 ;
: one4  one2 one2 ;
: one8  one4 one4 ;
: one16 one8 one8 ;
: add1 + ;
: add2  add1 add1 ;
: add4  add2 add2 ;
: add8  add4 add4 ;
: add16 add8 add8 ;
0
one16
add16
: next1 dup 1 + ;
: next2  next1 next1 ;
: next4  next2 next2 ;
: next8  next4 next4 ;
: next16 next8 next8 ;
: add1 + ;
: add2  add1 add1 ;
: add4  add2 add2 ;
: add8  add4 add4 ;
: add16 add8 add8 ;
0
next16
add16
: next1 dup 2 * ;
: next2  next1 next1 ;
: next4  next2 next2 ;
: next8  next4 next4 ;
: add1 + ;
: add2  add1 add1 ;
: add4  add2 add2 ;
: add8  add4 add4 ;
1
next8
add8
: next1 dup 2 * ;
: next2  next1 next1 ;
: next4  next2 next2 ;
: mul1 * ;
: mul2  mul1 mul1 ;
: mul4  mul2 mul2 ;
1
next4
mul4
: foo 5 ;
: bar foo ;
: foo 6 ;
bar foo
: SWAP DUP Dup dup ;
1 swap
: + * ;
3 4 +
: dup-twice dup dup ;
1 dup-twice