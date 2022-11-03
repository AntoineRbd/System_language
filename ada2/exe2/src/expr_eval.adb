package body Expr_Eval is
    function Eval(E: Expr) return Integer is
        begin
            case E.Kind is
                when Bin_Op => return Eval(BinOp(E.L.Val, E.R.Val, E.Op));
                when If_Expr => 
                    if Eval(E.Cond.all) /= 0 then
                        return Eval(E.Then_Expr.all);
                    else
                        return Eval(E.Else_Expr.all);
                    end if;
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
                when Logic_And =>
                    if L /= 0 and R /= 0 then
                        return Expr'(Kind => Literal, Val => 1);
                    else
                        return Expr'(Kind => Literal, Val => 0);
                    end if; 
                when Logic_Or => 
                    if L /= 0 or R /= 0 then
                        return Expr'(Kind => Literal, Val => 1);
                    else
                        return Expr'(Kind => Literal, Val => 0);
                    end if; 
            end case;
            return Expr'(Kind => Literal, Val => 0);
        end BinOp;
end Expr_Eval;

