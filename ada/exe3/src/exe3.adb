with Ada.Text_IO; use Ada.Text_IO;

procedure Exe3 is
   procedure test_string_to_int(I1: Integer; I2: Integer) is
   begin
      if I1 = I2 then
         put_line("Test Ok");
      else
         put_line("Failed test, " & I1'Image & " != " & I2'Image);
      end if;
   end test_string_to_int;

   function String_to_int(S1: String) return Integer is
      res: Integer := 0;
      number: Integer := 0;
      is_negative: Boolean := False;
      s: String := S1;

   begin
      if s(1) = '-' then
         is_negative := True;
         s(1) := '0';
      end if;

      for val of s loop
         number := Character'Pos(val) - 48;
         res := res * 10;
         res := res + number;
      end loop;
      
      if is_negative then
         res := res * (-1);
      end if;

      return res;
   end String_to_int;

begin
   test_string_to_int(String_to_int("0"), 0);
   test_string_to_int(String_to_int("-0"), 0);
   test_string_to_int(String_to_int("1"), 1);
   test_string_to_int(String_to_int("-1"), -1);
   test_string_to_int(String_to_int("9999"), 9999);
   test_string_to_int(String_to_int("-9999"), -9999);
   test_string_to_int(String_to_int("00000125"), 125);
end Exe3;
