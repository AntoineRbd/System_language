package Stack is
   Max : constant Natural := 2;

   type Element_T is new Natural;
   type Stack_T is private;
   type Model_Array_T is array (Positive range 1 .. Max) of Element_T;

   function Is_Full (S : in Stack_T) return Boolean;
   function Is_Empty (S : in Stack_T) return Boolean;
   function Size (S : in Stack_T) return Natural;
   function Get_Model (S : in out Stack_T) return Model_Array_T;

   procedure Reset (S : out Stack_T)
     with Post => Is_Empty(S);
   function  Pop (S : in out Stack_T) return Element_T
     with Pre => not Is_Empty(S),
          Post => not Is_Full(S) and then Size(S) = Size(S'Old) - 1 and then Pop'Result = Get_Model(S)(Size(S'Old));
   procedure Push (S : in out Stack_T; E : Element_T)
     with Pre => not Is_Full(S),
          Post => not Is_Empty(S) and then Size(S) = Size(S'Old) + 1 and then E = Get_Model(S)(Size(S));

private
   type Element_Array_T is array (Positive range 1 .. Max) of Element_T;
   type Stack_T is record
      Content : Element_Array_T;
      Length  : Natural := 0;
   end record;

   function Is_Full (S : in Stack_T) return Boolean is (S.Length = Max);
   function Is_Empty (S : in Stack_T) return Boolean is (S.Length = 0);
   function Size (S : in Stack_T) return Natural is (S.Length);
   function Get_Model (S : in out Stack_T) return Model_array_T is (Model_array_T(S.Content));
end Stack;
