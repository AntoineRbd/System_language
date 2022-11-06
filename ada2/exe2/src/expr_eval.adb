package body Expr_Eval is
    overriding function Eval (L : Literal) return Integer is
    begin
        return L.val;
    end Eval;

    overriding function Eval (B : Bin_Op) return Integer is
        L_evaluated: Integer := Eval(B.L.all);
        R_evaluated: Integer := Eval(B.R.all);
    begin
        if B.Op = Div and R_evaluated = 0 then 
            raise DivisionByZero;
        end if;
        return (case B.Op is
        when Add => L_evaluated + R_evaluated,
        when Sub => L_evaluated - R_evaluated,
        when Mul => L_evaluated * R_evaluated,
        when Div =>  L_evaluated / R_evaluated,
        when Logic_And => (if (L_evaluated /= 0 And R_evaluated /= 0) then 1 else 0),
        when Logic_Or => (if (L_evaluated /= 0 or R_evaluated /= 0) then 1 else 0),
        when others => 0);
    end Eval;

    overriding function Eval (I : If_Expr) return Integer is
    begin
        return (if Eval(I.Cond.all) /= 0 then Eval(I.Then_Expr.all) else Eval(I.Else_Expr.all));
    end Eval;
end Expr_Eval;

