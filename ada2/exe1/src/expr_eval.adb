package body Expr_Eval is
    function Eval(E: Expr) return Integer is
        begin
            case E.Kind is
                when Bin_Op => return Eval(BinOp(E.L, E.R, E.Op));
                when If_Expr => return Eval(IfExpr(E.Cond, E.Then_Expr, E.Else_Expr));
                when Literal => return E.Val;
            end case;
            return 0;
        end Eval;

    function BinOp(L: Expr.Val; R: Expr.Val; Op: Expr.Op_Kind) return Expr is
        l: Integer := Eval(L);
        r: Integer := Eval(R);
        begin
            case Op is
                when Add => return new Expr'(Kind => Literal, Val => l + r);
                when Sub => return new Expr'(Kind => Literal, Val => l - r);
                when Mul => return new Expr'(Kind => Literal, Val => l * r);
                when Div =>
                    if r /= 0 then
                        return new Expr'(Kind => Literal, Val => l / r);
                    else
                        return new Expr'(Kind => Literal, Val => -1);
                    end if;
                when Logic_And => return new Expr'(Kind => Literal, Val => l AND r);
                when Logic_Or => return new Expr'(Kind => Literal, Val => l OR r);
            end case;
            return new Expr'(Kind => Literal, Val => 0);
        end BinOp;

    function IfExpr(Cond: Expr.Expr_Access; Then_Expr: Expr.Expr_Access; Else_Expr: Expr.Expr_Access) return Expr is
    begin
        if Eval(Cond) /= 0 then
            return new Expr'(Kind => Literal, Val => 0);
        else
            return new Expr'(Kind => Literal, Val => -1);
        end if;
    end IfExpr;

end Expr_Eval;

