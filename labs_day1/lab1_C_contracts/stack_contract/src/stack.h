#define STACK_SIZE 3

typedef struct s_stack{
  int stack[STACK_SIZE];
  int length;
}t_stack;

t_stack init();
void print(t_stack s);
void push(t_stack *s, int v, int *err);
int pop(t_stack *s, int *err);
int is_full(t_stack s);
int is_empty(t_stack s);
int length(t_stack s);
