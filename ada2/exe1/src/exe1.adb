package Expr_Eval is

    type Expr;

    type Expr_Kind is (Bin_Op, Literal, If_Expr);
    type Op_Kind is (Add, Sub, Mul, Div, Logic_And, Logic_Or);
    type Expr_Access is access Expr;

    type Expr (Kind : Expr_Kind) is record
      case Kind is
         when Bin_Op =>
            L, R : Expr_Access;
            Op   : Op_Kind;
         when If_Expr =>
            Cond, Then_Expr, Else_Expr : Expr_Access;
         when Literal =>
            Val : Integer;
      end case;
    end record;

    function Eval(E: Expr) return Integer is
        begin
            case E.Expr_Kind is
                when BinOp => return Eval(BinOp(E.L, E.R, E.Op));
                when If_Expr => return Eval(IfExpr(E.Cond, E.Then_Expr, E.Else_Expr));
                when Literal => return E.Val;
            end case;
        end Eval;

    function BinOp(L: Expr.Val, R: Expr.Val, Op: Expr.Op_Kind) return Expr is
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
        end BinOp;

    function IfExpr(Cond: Expr.Expr_Access, Then_Expr: Expr.Expr_Access, Else_Expr: Expr.Expr_Access) return Expr is
    begin
        if Eval(Cond) /= 0 then
            return new Expr'(Kind => Literal, Val => 0);
        else
            return new Expr'(Kind => Literal, Val => -1);
        end if;
    end IfExpr;

end Expr_Eval;


with Expr_Eval; use Expr_Eval;



procedure Test is
    E : Expr := (Kind => Bin_Op,
                 L => new Expr'(Kind => Literal, Val => 12),
                 R => new Expr'(Kind => Literal, Val => 15),
                 Op => Add);

begin
    Put_Line (Eval (E)'Image);
end Test;
