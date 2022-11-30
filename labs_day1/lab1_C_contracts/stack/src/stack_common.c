#include <stdio.h>
#include "stack.h"

t_stack init()
{
  t_stack s;

  s.length = 0;
  return(s);
}

void print(t_stack s)
{
  int i;

  printf(">>>>>>length = %d\n", s.length);
  for (i = 0; i < s.length; i++)
    {
      printf("%d => %d\n", i, s.stack[i]);
    }
  printf("<<<<<<\n");
}

int length(t_stack s)
{
  return(s.length);
};

int is_empty(t_stack s)
{
  return(s.length == 0);
};
int is_full(t_stack s)
{
  return(s.length == STACK_SIZE);
};
