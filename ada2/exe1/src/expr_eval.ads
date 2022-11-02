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

    function Eval(E: Expr) return Eval_Result;
    function BinOp(L: Integer; R: Integer; Op: Expr_Eval.Op_Kind) return Expr;
    function IfExpr(Cond: Expr_Eval.Expr_Access; Then_Expr: Expr_Eval.Expr_Access; Else_Expr: Expr_Eval.Expr_Access) return Expr;
end Expr_Eval;

type Eval_Result_Kind is (Bool, Int);
type Eval_Result (K: Eval_Result_Kind) is record
    case K is
        when bool =>
            Bool_Val: Boolean;
        when Int => 
            Int_Val: Integer;
    end case;
end record;
