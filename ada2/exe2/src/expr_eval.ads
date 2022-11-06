package Expr_Eval is

    DivisionByZero: exception;

    type Expr is abstract tagged null record;

    function Eval (E: Expr) return Integer is abstract;

    type Op_Kind is (Add, Sub, Mul, Div, Logic_And, Logic_Or);
    type Expr_Access is access Expr'Class;

    type Bin_Op is new Expr with record
        L, R: Expr_Access;
        Op: Op_Kind;
    end record;
    type Bin_Op_Access is access Bin_Op'Class;

    type If_Expr is new Expr with record
        Cond, Then_Expr, Else_Expr: Expr_Access;
    end record;
    type If_Expr_Access is access If_Expr'Class;

    type Literal is new Expr with record
        val: Integer;
    end record;
    type Literal_Access is access Literal'Class;

    overriding function Eval (L : Literal) return Integer;
    overriding function Eval (B : Bin_Op) return Integer;
    overriding function Eval (I : If_Expr) return Integer;

end Expr_Eval;

