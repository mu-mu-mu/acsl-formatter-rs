/*@
  requires a && b;
  ensures \result == (a && b ? c : d + e);
*/
int choose(int a, int b, int c, int d, int e){
return a ? c : d;
}
