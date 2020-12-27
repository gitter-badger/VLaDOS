#[derive(Debug, Eq, PartialEq, Clone, EnumString)]
#[strum(serialize_all = "snake_case")]
pub(crate) enum TokenType {
    // Single Character Building Blocks
    #[strum(serialize = "`")]
    Tick,
    #[strum(serialize = "~")]
    Tilde,
    #[strum(serialize = "!")]
    Bang,
    #[strum(serialize = "@")]
    CommercialAt,
    #[strum(serialize = "$")]
    Dollar,
    #[strum(serialize = "%")]
    Parentheses,
    #[strum(serialize = "^")]
    Carat,
    #[strum(serialize = "&")]
    Ampersand,
    #[strum(serialize = "(")]
    LeftParen,
    #[strum(serialize = ")")]
    RightParen,
    #[strum(serialize = "{")]
    LeftBrace,
    #[strum(serialize = "}")]
    RightBrace,
    #[strum(serialize = "[")]
    LeftBracket,
    #[strum(serialize = "]")]
    RightBracket,
    #[strum(serialize = ",")]
    Comma,
    #[strum(serialize = ".")]
    Dot,
    #[strum(serialize = "'")]
    Apostrophe,
    #[strum(serialize = r#"""#)]
    DoubleQuote,
    #[strum(serialize = r#"\"#)]
    BackSlash,
    #[strum(serialize = "-")]
    Minus,
    #[strum(serialize = "+")]
    Plus,
    #[strum(serialize = "*")]
    Star,
    #[strum(serialize = "/")]
    ForwardSlash,
    #[strum(serialize = ";")]
    Semicolon,
    #[strum(serialize = "=")]
    Assignment,

    // Operators
    // Operators: Equality
    #[strum(serialize = "==")]
    LogicalEqual,
    #[strum(serialize = "===")]
    StateEqual,
    #[strum(serialize = "==?")]
    WildcardEqual,
    #[strum(serialize = "!=")]
    LogicalUnequal,
    #[strum(serialize = "!==")]
    StateUnequal,
    #[strum(serialize = "!=?")]
    WildcardUnequal,
    #[strum(serialize = ">")]
    Greater,
    #[strum(serialize = ">=")]
    GreaterEqual,
    #[strum(serialize = "<")]
    Less,
    #[strum(serialize = "<=")]
    LessEqual,

    // Operators: Arithmetic Assignment
    #[strum(serialize = "+=")]
    PlusAssign,
    #[strum(serialize = "-=")]
    MinusAssign,
    #[strum(serialize = "*=")]
    MultiplyAssign,
    #[strum(serialize = "/=")]
    DivideAssign,
    #[strum(serialize = "&=")]
    BitwiseAndAssign,
    #[strum(serialize = "|=")]
    BitwiseOrAssign,
    #[strum(serialize = "^=")]
    BitwiseXorAssign,
    #[strum(serialize = "%=")]
    ModuloAssign,
    #[strum(serialize = "<<=")]
    LogicalLeftShiftAssign,
    #[strum(serialize = ">>=")]
    LogicalRightShiftAssign,
    #[strum(serialize = "<<<=")]
    ArithmeticLeftShiftAssign,
    #[strum(serialize = ">>>=")]
    ArithmeticRightShiftAssign,

    // Operators: Shorthand variable modification
    #[strum(serialize = "++")]
    PlusPlus,
    #[strum(serialize = "--")]
    MinusMinus,

    // Operators: Shifting
    #[strum(serialize = "<<")]
    LogicalLeftShift,
    #[strum(serialize = ">>")]
    LogicalRightShift,
    #[strum(serialize = "<<<")]
    ArithmeticLeftShift,
    #[strum(serialize = ">>>")]
    ArithmeticRightShift,

    // Operators: Logical
    #[strum(serialize = "~&")]
    Nand,
    #[strum(serialize = "~|")]
    Nor,
    #[strum(serialize = "~^", serialize = "^~")]
    Xnor,
    #[strum(serialize = "&&")]
    BooleanAnd,
    #[strum(serialize = "||")]
    BooleanOr,
    #[strum(serialize = "->")]
    Implication,
    #[strum(serialize = "<->")]
    Iff,

    // Etc.
    #[strum(serialize = "//")]
    Comment,

    // Keywords: SystemVerilog OOP Constructs
    Class,
    Endclass,
    Interface,
    Endinterface,
    Extends,
    Void,
    Static,
    Pure,
    Virtual,
    Type,

    // Keywords: Scope
    Begin,
    End,

    // Keywords: Data Types
    Typedef,
    Struct,
    Union,
    Packed,
    Const,
    Unsigned,
    Bit,
    Byte,
    Int,
    Longint,
    Shortint,
    Integer,
    Real,
    Longreal,
    Shortreal,
    Reg,
    Logic,
    Time,
    String,
    Assign,
    Var,
    Enum,

    // Keywords: Parameters
    Parameter,
    Localparam,
    Specparam,
    Defparam,

    // Keywords: Nets
    Nettype,
    Wire,
    Wand,
    Wor,
    Uwire,
    Tri,
    Triand,
    Trior,
    Tri0,
    Tri1,
    Trireg,
    Supply0,
    Supply1,
    Interconnect,

    // Keywords: Signal Strength
    Small,
    Medium,
    Large,

    // Keywords: Packages
    Package,
    Endpackage,
    Import,

    // Keywords: Functions
    Function,
    Automatic,
    Input,
    Output,
    Inout,
    Endfunction,
    Return,

    // Keywords: Modules
    Module,
    Endmodule,

    // Keywords: Tasks
    Task,
    Endtask,

    // Keywords: Configs
    Config,
    Endconfig,

    // Keywords: Clocking
    Posedge,
    Negedge,
    Edge,
    Always,
    AlwaysComb,
    AlwaysLatch,
    AlwaysFf,
    Initial,

    // Keywords: Properties
    Property,
    Endproperty,

    // Keywords: Program
    Program,
    Endprogram,

    // Keywords: Generate
    Generate,
    Endgenerate,

    // Keywords: Specify
    Specify,
    Endspecify,

    // Keywords: Loops
    Forever,
    Repeat,
    For,
    Foreach,
    While,
    Do,
    Break,
    Continue,

    // Keywords: Conditionals
    If,
    Else,
    Case,
    Endcase,
    Casez,
    Casex,
    Unique,
    Unique0,
    Priority,
    Matches,
    Tagged,

    // Keywords: Interprocess Communication
    Semaphore,
    Mailbox,
    Wait,
    WaitOrder,
    Event,
    Fork,
    JoinAny,
    JoinNone,
    Join,

    // Keywords: Assertions
    Assert,
    Assume,
    Restrict,
    Final,
    Cover,

    // Keywords: Etc.
    Library,
    Default,
    Timeunit,
    Inside,
    Dist,

    // Literals
    StringLiteral,
    NumberLiteral,
    IdentifierLiteral,

    #[strum(serialize = "EOF")]
    Eof,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_enum_reference() {
        let _ = TokenType::Comma;
        let _ = TokenType::LeftParen;
    }
}
