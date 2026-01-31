int use(int a, int b, int c, int i){
/*@ f(a, b + c, d[e]) == g()[i + 1]; */
return a + b + c + i;
}
