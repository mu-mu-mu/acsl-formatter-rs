int choose(int a, int b, int c, int d, int e){
/*@ (a && b) ? (c) : (d + e); */
return a ? c : d;
}
