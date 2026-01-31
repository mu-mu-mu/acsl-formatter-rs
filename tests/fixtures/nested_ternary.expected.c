/*@
  ensures \result == a ? b : c ? d : e;
*/
int choose3(int a, int b, int c, int d, int e){
return a ? b : c;
}
