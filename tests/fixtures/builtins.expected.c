/*@
  requires x >= 0;
  ensures \result == \old(x) + \at(y, L1);
*/
int foo(int x, int y){
return x + y;
}
