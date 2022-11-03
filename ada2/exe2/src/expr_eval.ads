package Expr_Eval is

    type Expr;

    type Expr_Kind is (Bin_Op, Literal, If_Expr);
    type Op_Kind is (Add, Sub, Mul, Div, Logic_And, Logic_Or);
    type Expr_Access is access Expr;
    type Ada.Containers.Indefinite_Holders(Integer) is access Expr;


    type Expr (Kind : Expr_Kind) is record
      case Kind is
         when Bin_Op =>
            L, R : Holder(Integer);
            Op   : Op_Kind;
         when If_Expr =>
            Cond, Then_Expr, Else_Expr : Expr_Access;
         when Literal =>
            Val : Integer;
      end case;
    end record;

    function Eval(E: Expr) return Integer;
    function BinOp(L: Integer; R: Integer; Op: Expr_Eval.Op_Kind) return Expr;
end Expr_Eval;
