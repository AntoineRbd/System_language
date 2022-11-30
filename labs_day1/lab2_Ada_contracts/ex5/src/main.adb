with Stack; use Stack;

procedure Main is

   S : Stack_T;
   E : Element_T;
begin
   
   Reset (S);
   Push (S, 1);
   E := Pop (S);
   
end Main;
