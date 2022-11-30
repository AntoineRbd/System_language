#include <stdio.h>
#include "stack.h"

void push(t_stack *s, int v, int *err)
{
  s->stack[s->length] = v;
  s->length++;
}

int pop(t_stack *s, int *err)
{
  int ret;

  ret = s->stack[s->length];
  s->length--;
  return ret;  
}

