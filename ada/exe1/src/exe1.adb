with Ada.Text_IO; use Ada.Text_IO;

procedure Exe1 is
   test_1: String := "kayak";
   test_2: String := "hello world";
   test_3: String := "123_hello";

   function Invert(S: String) return String is
      res: String := S;
   begin
      for i in S'range loop
         res(res'length - i + 1) := S(i);
      end loop;
      return res;
   end Invert;

   procedure Invert_in_place(S: in out String) is
      buff: Character := '0';
   begin
      for i in 1 .. (S'length / 2) loop
         buff := S(i);
         S(i) := S(S'length - i + 1);
         S(S'length - i + 1) := buff;
      end loop;
   end Invert_in_place;

   procedure test_invert(S1: String; S2: String) is
   begin
      if S1 = S2 then
         put_line("Test ok");
      else
         put_line("Fail test, " & S1 & " != " & S2);
      end if;
   end test_invert;
begin
   test_invert(Invert(test_1), "kayak");
   test_invert(Invert(test_2), "dlrow olleh");
   test_invert(Invert(test_3), "olleh_321");

   Invert_in_place(test_1);
   Invert_in_place(test_2);
   Invert_in_place(test_3);

   test_invert(test_1, "kayak");
   test_invert(test_2, "dlrow olleh");
   test_invert(test_3, "olleh_321");
end Exe1;
