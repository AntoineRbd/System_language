with Ada.Text_IO; use Ada.Text_IO;

with stack;

procedure Exe8 is
   procedure test_int(N1: Integer; N2: Integer) is
   begin
      if N1 = N2 then
         put_line("Test Ok");
      else
         put_line("Failed test, " & N1'Image & " != " & N2'Image);
      end if;
   end test_int;

   procedure stack_test is
      stack_test: stack.Stack := Stack.New_Stack(10);
      val: Integer := 0;
   begin
      put_line("---Stack Test---");

      stack.Push(stack_test, 34);
      stack.Push(stack_test, 35);
      stack.Push(stack_test, 36);
      stack.Push(stack_test, 37);
      stack.Push(stack_test, 38);
      stack.Push(stack_test, 39);

      stack.Pop(stack_test, val);
      test_int(val, 39);

      stack.Pop(stack_test, val);
      test_int(val, 38);

      stack.Pop(stack_test, val);
      test_int(val, 37);

      stack.Pop(stack_test, val);
      test_int(val, 36);

      stack.Pop(stack_test, val);
      test_int(val, 35);

      stack.Pop(stack_test, val);
      test_int(val, 34);
   end stack_test;

   procedure push_limit is
      stack_test: stack.Stack := Stack.New_Stack(2);
   begin
      put_line("---Push Limit Test---");
      stack.Push(stack_test, 34);
      stack.Push(stack_test, 35);
      stack.Push(stack_test, 36);
      put_line("Failed test");
   exception
      when Stack.ReachSizeMax => put_line("Test Error succedd");
   end push_limit;

   procedure pop_empty is
      stack_test: stack.Stack := Stack.New_Stack(100);
      val: Integer := 0;
   begin
      put_line("---Pop Empty Test---");
      stack.Push(stack_test, 90);
      stack.Pop(stack_test, val);
      stack.Pop(stack_test, val);
      put_line("Failed test");
   exception
      when Stack.EmptyStack => put_line("Test Error succedd");
   end pop_empty;
begin
   stack_test;
   push_limit;
   pop_empty;
end Exe8;
