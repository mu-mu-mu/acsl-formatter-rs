/*@
  requires i >= 0;
  ensures \result == f(a -> s [ 5 ]);
*/
int use(struct s *a){
return f(a->s[5]);
}
