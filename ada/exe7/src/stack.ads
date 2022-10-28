package Stack is
    type Stack is private;
    ReachSizeMax, EmptyStack: exception;

    procedure Push(S: in out Stack; Val: Integer); 
    procedure Pop(S: in out Stack; Val: out Integer);

private

    type Stack_Index is range 1..10;
    type Stack_Values is Array (Stack_Index) of Integer;

    type Stack is record
        values: Stack_Values := (others => 0);
        index: Stack_Index := 1;
    end record;

end Stack;