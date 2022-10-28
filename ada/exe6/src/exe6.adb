with Ada.Text_IO; use Ada.Text_IO;

with stack;

procedure Exe6 is
   procedure test_string(S1: String; S2: String) is 
   begin
      if S1 = S2 then
         put_line("Test Ok");
      else
         put_line("Failed test, " & S1 & " != " & S2);
      end if;
   end test_string;

   procedure test_int(N1: Integer; N2: Integer) is
   begin
      if N1 = N2 then
         put_line("Test Ok");
      else
         put_line("Failed test, " & N1'Image & " != " & N2'Image);
      end if;
   end test_int;

   procedure push_test is 
      push_test: stack.Stack;
   begin
      put_line("---Push Test---");

      stack.Push(push_test, 0);
      test_int(push_test.values(1), 0);

      stack.Push(push_test, 1);
      test_int(push_test.values(1), 0);
      test_int(push_test.values(2), 1);

      stack.Push(push_test, 2);
      test_int(push_test.values(1), 0);
      test_int(push_test.values(2), 1);
      test_int(push_test.values(3), 2);
   end push_test;

   procedure push_limit_test is 
      push_test: stack.Stack;
   begin
      put_line("---Push Limit Test---");
      for i in 0..8 loop
         stack.Push(push_test, 23);
      end loop;
      stack.Push(push_test, 34);
      put_line("Failed test");
   exception
      when Stack.ReachSizeMax => put_line("Test Error succedd");
   end push_limit_test;

   procedure pop_test is
      pop_test: stack.Stack;
      val: Integer := 0;
   begin
      put_line("---Pop Test---");

      stack.Push(pop_test, 34);    
      stack.Push(pop_test, 35);    
      stack.Push(pop_test, 36);    
      stack.Push(pop_test, 37);    
      stack.Push(pop_test, 38);    
      stack.Push(pop_test, 39); 

      Stack.Pop(pop_test, val);
      test_int(val, 39);

      Stack.Pop(pop_test, val);
      test_int(val, 38);

      Stack.Pop(pop_test, val);
      test_int(val, 37);

      Stack.Pop(pop_test, val);
      test_int(val, 36);
      
      Stack.Pop(pop_test, val);
      test_int(val, 35);

      Stack.Pop(pop_test, val);
      test_int(val, 34);
   end pop_test;

   procedure pop_empty_test is
      pop_test: stack.Stack;
      val: Integer := 0;
   begin
      put_line("---Pop Empty Test---");
      stack.Push(pop_test, 90);
      stack.Pop(pop_test, val);
      stack.Pop(pop_test, val);
      put_line("Failed test");
   exception
      when Stack.EmptyStack => put_line("Test Error succedd");
   end pop_empty_test;
begin
   push_test;
   push_limit_test;
   pop_test;
   pop_empty_test;
end Exe6;
