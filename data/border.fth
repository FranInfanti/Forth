: foo 1 ;
: foo foo foo ;
: bar 1 foo ;
: foo 8 ;
: gar foo ;
: par bar ;
bar foo gar par
: = * ;
8 5 =