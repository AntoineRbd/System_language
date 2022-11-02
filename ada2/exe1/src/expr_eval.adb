package body Expr_Eval is
    function Eval(E: Expr) return Integer is
        begin
            case E.Kind is
                when Bin_Op => return Eval(BinOp(E.L.Val, E.R.Val, E.Op));
                when If_Expr => return Eval(IfExpr(E.Cond, E.Then_Expr, E.Else_Expr));
                when Literal => return E.Val;
            end case;
            return 0;
        end Eval;

    function BinOp(L: Integer; R: Integer; Op: Expr_Eval.Op_Kind) return Expr is
        begin
            case Op is
                when Add => return Expr'(Kind => Literal, Val => L + R);
                when Sub => return Expr'(Kind => Literal, Val => L - R);
                when Mul => return Expr'(Kind => Literal, Val => L * R);
                when Div =>
                    if r /= 0 then
                        return Expr'(Kind => Literal, Val => L / R);
                    else
                        return Expr'(Kind => Literal, Val => -1);
                    end if;
                when Logic_And => return Expr'(Kind => Literal, Val => L and R);
                when Logic_Or => return Expr'(Kind => Literal, Val => L or R);
            end case;
            return Expr'(Kind => Literal, Val => 0);
        end BinOp;

    function IfExpr(Cond: Expr_Eval.Expr_Access; Then_Expr: Expr_Eval.Expr_Access; Else_Expr: Expr_Eval.Expr_Access) return Expr is
    begin
        return Expr'(Kind => Literal, Val => 0);
    end IfExpr;

end Expr_Eval;

