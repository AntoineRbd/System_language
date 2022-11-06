with Ada.Strings.Hash;
with Ada.Strings.Fixed.Hash;
with Ada.Strings.Unbounded;
use Ada.Strings.Unbounded;
with Ada.Strings.Unbounded.Hash;
with Ada.Containers.Indefinite_Hashed_Maps;

package Expr_Eval is
    type NameString is new String(1..256);

    package Binder_Hashed_Maps is new
    Ada.Containers.Indefinite_Hashed_Maps
        (Key_Type        => Unbounded_String,
        Element_Type    => Integer,
        Hash            => Ada.Strings.Unbounded.Hash,
        Equivalent_Keys => "=");
    use Binder_Hashed_Maps;


    DivisionByZero, UnknowVariableName: exception;

    type Expr is abstract tagged null record;

    function Eval (E: Expr) return Integer is abstract;

    type Op_Kind is (Add, Sub, Mul, Div, Logic_And, Logic_Or);
    type Expr_Access is access Expr'Class;

    type Bin_Op is new Expr with record
        L, R: Expr_Access;
        Op: Op_Kind;
    end record;
    type Bin_Op_Access is access Bin_Op'Class;

    type If_Expr is new Expr with record
        Cond, Then_Expr, Else_Expr: Expr_Access;
    end record;
    type If_Expr_Access is access If_Expr'Class;

    type Literal is new Expr with record
        val: Integer;
    end record;
    type Literal_Access is access Literal'Class;

    type Let_Expr is new Expr with record
        name: Unbounded_String;
        value, next: Expr_Access;
    end record;
    type Let_Expr_Access is access Let_Expr'Class;

    type Ref_Expr is new Expr with record
        name: Unbounded_String;
    end record;
    type Ref_Expr_Access is access Ref_Expr'Class;

    overriding function Eval (L : Literal) return Integer;
    overriding function Eval (B : Bin_Op) return Integer;
    overriding function Eval (I : If_Expr) return Integer;
    overriding function Eval (let : Let_Expr) return Integer;
    overriding function Eval (ref : Ref_Expr) return Integer;

    procedure Clear_Binder;

end Expr_Eval;
