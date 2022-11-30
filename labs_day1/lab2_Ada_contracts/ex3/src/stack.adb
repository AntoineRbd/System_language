package body Stack is
   procedure Reset (S : out Stack_T) is
   begin
      S.Length := 0;
   end Reset;

   function  Pop (S : in out Stack_T) return Element_T is
      E : Element_T := S.Content (S.Length);
   begin
      S.Length := S.Length - 1;
      return E;
   end Pop;

   procedure Push (S : in out Stack_T; E : Element_T) is
   begin
      S.Length := S.Length + 1;
      S.Content (S.Length) := E;
   end Push;
end Stack;
