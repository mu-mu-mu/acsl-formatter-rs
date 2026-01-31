int foo(int x){
/*@ \at(\old(x), L1) == \at(x, L2); */
return x;
}
