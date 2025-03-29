: f if 2 then ;
-1 f
: g if 2 else 3 then ;
-1 g
0 g
: h
    if
    if 1 else 2 then
    else
    drop 3
    then ;
-1 -1 h
0 -1 h
0 0 h
: l
    dup 0 = if
    drop 2
    else dup 1 = if
    drop 3
    else
    drop 4
    then then ;
0 l
1 l
2 l
: z if 10 then ;
5 z