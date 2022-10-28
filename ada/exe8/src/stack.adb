package body Stack is
    procedure Push (S: in out Stack; Val: Integer) is
    begin
        S.values(S.index) := Val;
        S.index := S.index + 1;
    exception
        when CONSTRAINT_ERROR => raise ReachSizeMax;
    end Push;

    procedure Pop (S: in out Stack; Val: out Integer) is
    begin
        S.index := S.index - 1;
        Val := S.values(S.index);
    exception
        when CONSTRAINT_ERROR => raise EmptyStack;
    end Pop;

    function New_Stack(size: Integer) return Stack is
        res: Stack;
    begin
        res.values := new Stack_Values(1 .. size);
        return res;
    exception
        when CONSTRAINT_ERROR => raise EmptyStack;
    end New_Stack;
end Stack;