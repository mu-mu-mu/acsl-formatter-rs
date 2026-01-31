int loop_sum(int n){
int i = 0;
int sum = 0;
/*@
  loop invariant (0 <= i) && (i <= n);
*/
while(i < n){
/*@
  assert sum >= 0;
*/
sum = sum + i;
i = i + 1;
}
return sum;
}
