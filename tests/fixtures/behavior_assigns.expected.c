/*@
  assigns \nothing;
  ensures \result >= 0;
  behavior positive:
  assumes x > 0;
  assigns \nothing;
  ensures \result > 0;
  behavior any:
  assigns \nothing;
  ensures \result >= 0;
*/
int abs_val(int x){
if (x < 0) return -x;
return x;
}
