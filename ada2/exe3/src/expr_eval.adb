with Ada.Text_IO;
use Ada.Text_IO;

package body Expr_Eval is
    binding: Binder_Hashed_Maps.Map;

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

    overriding function Eval (let : Let_Expr) return Integer is
        res: Integer := 0;
    begin
        binding.insert(let.name, Eval(let.value.all));
        res := Eval(let.next.all);
        binding.delete(let.name);
        return res;
    end Eval;

    overriding function Eval (ref : Ref_Expr) return Integer is
    begin
        if binding.contains(ref.name) then 
            return binding(ref.name);
        else
            raise UnknowVariableName; 
        end if;
    end Eval;

    procedure Clear_Binder is
    begin
        binding.clear;
    end Clear_Binder;

end Expr_Eval;

