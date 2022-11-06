with Ada.Text_IO;
use Ada.Text_IO;
with Ada.Strings.Unbounded;
use Ada.Strings.Unbounded;

with Expr_Eval;
use Expr_Eval;


procedure Exo3 is
    procedure Unit_Test_Eq_Int(I1: Integer; I2: Integer) is
    begin
        if I1 = I2 then
            Put_Line("Test Passed...");
        else
            Put_Line("/!\ Fail test: " & I1'Image & " is not equal to " & I2'Image);
        end if;
    end Unit_Test_Eq_Int;

    procedure Simple_Let_Expr_Test is
        e: Let_Expr := (name => To_Unbounded_String("a"),
                        value => new Literal'(val => 42),
                        next => new Ref_Expr'(name => To_Unbounded_String("a")));
    begin
        Unit_Test_Eq_Int(Eval(e), 42);
    end Simple_Let_Expr_Test;

    procedure Add_Let_Expr_Test is
        e: Let_Expr := (name => To_Unbounded_String("a"),
                        value => new Bin_Op'(Op => Add, 
                            L => new Literal'(val => 1),
                            R => new Literal'(val => 1)),
                        next => new Ref_Expr'(name => To_Unbounded_String("a")));
    begin
        Unit_Test_Eq_Int(Eval(e), 2);
    end Add_Let_Expr_Test;

    procedure Nested_Let_Expr_Test is
        nested: Expr_Access := new Let_Expr'(
            name => To_Unbounded_String("b"),
            value => new Bin_Op'(Op => Add, 
                            L => new Literal'(val => 1),
                            R => new Literal'(val => 1)),
            next => new Bin_Op'(Op => Add, 
                            L => new Ref_Expr'(name => To_Unbounded_String("b")),
                            R => new Ref_Expr'(name => To_Unbounded_String("a")) 
                            ));

        first_let: Let_Expr := (
            name => To_Unbounded_String("a"),
            value => new Bin_Op'(Op => Add, 
                            L => new Literal'(val => 1),
                            R => new Literal'(val => 1)),
            next => nested);
    begin
       Unit_Test_Eq_Int(Eval(first_let), 4);
    end;

    procedure Unknow_Variable_Let_Expr_Test is
        e: Let_Expr := (name => To_Unbounded_String("a"),
                        value => new Literal'(val => 42),
                        next => new Ref_Expr'(name => To_Unbounded_String("b")));
        res: Integer := 0;
   begin
        res := Eval(e);
        Put_Line("Test failed, no exception raised");
   exception 
       when Expr_Eval.UnknowVariableName => Put_Line("Test Passed...");
    end;

    procedure Unknow_Variable_Let_Expr_Test2 is 
            e: Bin_Op := (
            L => new Let_Expr'(name => To_Unbounded_String("a"),
                        value => new Literal'(val => 2),
                        next => new Ref_Expr'(name => To_Unbounded_String("a"))),
            R => new Ref_Expr'(name => To_Unbounded_String("a")),
            Op => Add); 
        res: Integer := 0;
    begin
        res := Eval(e);
        Put_Line("Test failed, no exception raised");
   exception 
       when Expr_Eval.UnknowVariableName => Put_Line("Test Passed...");
    end;

begin
    Simple_Let_Expr_Test;
    Clear_Binder;
    Add_Let_Expr_Test;
    Clear_Binder;
    Nested_Let_Expr_Test;
    Clear_Binder;
    Unknow_Variable_Let_Expr_Test;
    Clear_Binder;
    Unknow_Variable_Let_Expr_Test2;

end Exo3;

