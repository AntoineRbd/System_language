with Ada.Text_IO;
use Ada.Text_IO;


with Expr_Eval;
use Expr_Eval;

procedure Exo2 is
    procedure Unit_Test_Eq_String(S1: String; S2: String) is
    begin
        if S1 = S2 then
            Put_Line("Test Passed...");
        else
            Put_Line("/!\ Fail test: '" & S1 & "' is not equal to '" & S2 & "'");
        end if;
    end Unit_Test_Eq_String;

    procedure Unit_Test_Eq_Int(I1: Integer; I2: Integer) is
    begin
        if I1 = I2 then
            Put_Line("Test Passed...");
        else
            Put_Line("/!\ Fail test: " & I1'Image & " is not equal to " & I2'Image);
        end if;
    end Unit_Test_Eq_Int;

    procedure Simple_Literal_Test is
        e: Literal := (val => 12);
    begin
        Unit_Test_Eq_Int(Eval(e), 12);
    end Simple_Literal_Test;

    procedure Simple_Bin_Op_Test is
        e: Bin_Op := (Op => Add, 
                        L => new Literal'(val => 1),
                        R => new Literal'(val => 1));
    begin
        Unit_Test_Eq_Int(Eval(e), 2);
    end Simple_Bin_Op_Test;

    procedure Simple_If_Expr_Test is
        false_branch: Expr_Access := new Literal'(val => -1);
        true_branch: Expr_Access := new Literal'(val => 1);
        cond: Expr_Access := new Literal'(val => 1);

        e: If_Expr := (Cond => cond,
                   Then_Expr => true_branch,
                   Else_Expr => false_branch);

    begin
        Unit_Test_Eq_Int(Eval(e), 1);
    end Simple_If_Expr_Test;

begin
    Simple_Literal_Test;
    Simple_Bin_Op_Test;
    Simple_If_Expr_Test;
end Exo2;

