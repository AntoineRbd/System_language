package Stack is
    type Stack is private;
    ReachSizeMax, EmptyStack: exception;

    function New_Stack(size: Integer) return Stack;
    procedure Push(S: in out Stack; Val: Integer); 
    procedure Pop(S: in out Stack; Val: out Integer);

private
    type Stack_Values is Array (Integer range <>) of Integer;

    type Stack is record
        values: access Stack_Values;
        index: Integer := 1;
    end record;
end Stack;