/*@
  requires i >= 0;
  ensures \result == f(a, (b + c), d[e]) + g()[i + 1];
*/
int use(int a, int b, int c, int i){
return a + b + c + i;
}
