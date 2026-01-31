int foo(int x, int y){
/*@ \result == \old(x) + \at(y, L1); */
return x + y;
}
