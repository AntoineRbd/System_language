#include <stdio.h>
#include "stack.h"

int main() 
{
  t_stack s = init();
  int v;
  int err = 0;

  print(s);
  push(&s, 42, &err);
  push(&s, 43, &err);
  push(&s, 44, &err);
  push(&s, 45, &err); 
  print(s);

  v = pop(&s, &err);
  print(s);
  printf("%d\n", v);
 
  v = pop(&s, &err);
  print(s);
  printf("%d\n", v);

  v = pop(&s, &err);
  print(s);
  printf("%d\n", v);

  v = pop(&s, &err);
  print(s);
  printf("%d\n", v); 

}

