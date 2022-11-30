#include <stdio.h>
#include "stack.h"

void push(t_stack *s, int v, int *err)
{
  // check if the stack is full using is_full function before pushing the value
  // v on the stack if the stack is full set *err to 1 otherwise set it to 0 to
  // indicate no error
  if (!is_full(*s))
  {
    s->stack[s->length] = v;
    s->length++;
    *err = 0;
  }
  else
    *err = 1;
}

int pop(t_stack *s, int *err)
{
  int ret;

  // check if the stack is empty using is_empty function before extracting the value from the stack
  // if the stack is empty set *err to 1 otherwise set it to 0 to indicate no errors
  // return 0 if *err is set to 1
  if (!is_empty (*s))
  {
    ret = s->stack[s->length - 1];
    s->length--;
    *err = 0;
  }
  else
  {
      ret = 0;
     *err = 1; 
  }
  return ret;
}

