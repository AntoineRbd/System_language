with Ada.Text_IO; use Ada.Text_IO;

procedure Exe5 is
   procedure triangle(n: Integer) is 
   begin
      for i in 0 .. n loop
         for j in reverse 0 .. n loop
            if i < j then
               put(' ');
            else
               put('*');
            end if;
         end loop;
         put_line("");
      end loop;
   end triangle;
begin
   triangle(5);
   triangle(42);
end Exe5;
