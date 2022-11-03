with Expr_Eval; use Expr_Eval;
with Ada.Text_IO; use Ada.Text_IO;

procedure Exe1 is
    Add_test : CONSTANT Expr := (Kind => Bin_Op,
                 L => new Expr'(Kind => Literal, Val => 12),
                 R => new Expr'(Kind => Literal, Val => 15),
                 Op => Add);
    Sub_test : CONSTANT Expr := (Kind => Bin_Op,
                 L => new Expr'(Kind => Literal, Val => 12),
                 R => new Expr'(Kind => Literal, Val => 2),
                 Op => Sub);
    Sub_neg_test : CONSTANT Expr := (Kind => Bin_Op,
                 L => new Expr'(Kind => Literal, Val => 10),
                 R => new Expr'(Kind => Literal, Val => 15),
                 Op => Sub);
    Mul_test : CONSTANT Expr := (Kind => Bin_Op,
                 L => new Expr'(Kind => Literal, Val => 10),
                 R => new Expr'(Kind => Literal, Val => 2),
                 Op => Mul);
    Mul_0_test : CONSTANT Expr := (Kind => Bin_Op,
                 L => new Expr'(Kind => Literal, Val => 10),
                 R => new Expr'(Kind => Literal, Val => 0),
                 Op => Mul);
    Div_test : CONSTANT Expr := (Kind => Bin_Op,
                 L => new Expr'(Kind => Literal, Val => 10),
                 R => new Expr'(Kind => Literal, Val => 2),
                 Op => Div);
    Div_0_test : CONSTANT Expr := (Kind => Bin_Op,
                 L => new Expr'(Kind => Literal, Val => 10),
                 R => new Expr'(Kind => Literal, Val => 0),
                 Op => Div);
    Logic_And_1_test : CONSTANT Expr := (Kind => Bin_Op,
                 L => new Expr'(Kind => Literal, Val => 10),
                 R => new Expr'(Kind => Literal, Val => 12),
                 Op => Logic_And);
    Logic_And_0_test : CONSTANT Expr := (Kind => Bin_Op,
                 L => new Expr'(Kind => Literal, Val => 10),
                 R => new Expr'(Kind => Literal, Val => 0),
                 Op => Logic_And);
    Logic_Or_1_test : CONSTANT Expr := (Kind => Bin_Op,
                 L => new Expr'(Kind => Literal, Val => 0),
                 R => new Expr'(Kind => Literal, Val => 12),
                 Op => Logic_Or);
    Logic_Or_0_test : CONSTANT Expr := (Kind => Bin_Op,
                 L => new Expr'(Kind => Literal, Val => 0),
                 R => new Expr'(Kind => Literal, Val => 0),
                 Op => Logic_Or);

    If_Expr_then_test : CONSTANT Expr := (Kind => If_Expr,
                 Cond => new Expr'(Kind => Literal, Val => 1),
                Then_Expr => new Expr'(Kind => Literal, Val => 1),
                 Else_Expr => new Expr'(Kind => Literal, Val => 0));
    If_Expr_else_test : CONSTANT Expr := (Kind => If_Expr,
                 Cond => new Expr'(Kind => Literal, Val => 0),
                Then_Expr => new Expr'(Kind => Literal, Val => 1),
                 Else_Expr => new Expr'(Kind => Literal, Val => 0));

    procedure test(a: Integer; b: Integer) is
    begin
        if a = b then 
            put_line("Test Ok");
        else
            put_line("Test failed... " & a'Image & " != " & b'Image);
        end if;
    end test;

begin
    test(Eval(Add_test), 27);
    test(Eval(Sub_test), 10);
    test(Eval(Sub_neg_test), -5);
    test(Eval(Mul_test), 20);
    test(Eval(Mul_0_test), 0);
    test(Eval(Div_test), 5);
    test(Eval(Div_0_test), -1);

    test(Eval(Logic_And_1_test), 1);
    test(Eval(Logic_And_0_test), 0);
    test(Eval(Logic_Or_1_test), 1);
    test(Eval(Logic_Or_0_test), 0);

    test(Eval(If_Expr_then_test), 1);
    test(Eval(If_Expr_else_test), 0);
end Exe1;
