with Ada.Text_IO; use Ada.Text_IO;
with Ada.Strings.Unbounded; use Ada.Strings.Unbounded;

procedure Exe2 is
   procedure test_int_to_string(S1: String; S2: String) is
   begin
      if S1 = S2 then
         put_line("Test Ok");
      else
         put_line("Failed test, " & S1 & " != " & S2);
      end if;
   end test_int_to_string;

   function int_to_string(number: Integer) return String is
      res : Unbounded_String;
      n : Integer := number;
      rest: Integer := 0;
      is_negative: Boolean := False;
   begin
      if n = 0 then
         return "0";
      end if;

      if n < 0 then
         n := n * (-1);
         is_negative := True;
      end if;

      while n /= 0 loop
         rest := n mod 10;
         n := n / 10;
         res := Character'Val(rest + 48) & res;
      end loop;

      if is_negative then
         res := '-' & res;
      end if;

      return To_String(res);
   end int_to_string;
begin
   test_int_to_string(int_to_string(0), "0");
   test_int_to_string(int_to_string(1), "1");
   test_int_to_string(int_to_string(-1), "-1");
   test_int_to_string(int_to_string(42), "42");
   test_int_to_string(int_to_string(9999), "9999");
   test_int_to_string(int_to_string(-9999), "-9999");
   test_int_to_string(int_to_string(-0), "0");
end Exe2;
