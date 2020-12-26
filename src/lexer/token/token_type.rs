#[derive(Debug, Eq, PartialEq, Clone, EnumString)]
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
    #[strum(serialize = "class")]
    Class,
    #[strum(serialize = "endclass")]
    EndClass,
    #[strum(serialize = "interface")]
    Interface,
    #[strum(serialize = "endinterface")]
    EndInterface,
    #[strum(serialize = "extends")]
    Extends,
    #[strum(serialize = "void")]
    Void,
    #[strum(serialize = "static")]
    Static,
    #[strum(serialize = "pure")]
    Pure,
    #[strum(serialize = "virtual")]
    Virtual,
    #[strum(serialize = "type")]
    Type,

    // Keywords: Scope
    #[strum(serialize = "begin")]
    Begin,
    #[strum(serialize = "end")]
    End,

    // Keywords: Data Types
    #[strum(serialize = "typedef")]
    TypeDef,
    #[strum(serialize = "struct")]
    Struct,
    #[strum(serialize = "union")]
    Union,
    #[strum(serialize = "packed")]
    Packed,
    #[strum(serialize = "const")]
    Const,
    #[strum(serialize = "unsigned")]
    Unsigned,
    #[strum(serialize = "bit")]
    Bit,
    #[strum(serialize = "byte")]
    Byte,
    #[strum(serialize = "int")]
    Int,
    #[strum(serialize = "longint")]
    LongInt,
    #[strum(serialize = "shortint")]
    ShortInt,
    #[strum(serialize = "integer")]
    Integer,
    #[strum(serialize = "real")]
    Real,
    #[strum(serialize = "longreal")]
    LongReal,
    #[strum(serialize = "shortreal")]
    ShortReal,
    #[strum(serialize = "reg")]
    Reg,
    #[strum(serialize = "logic")]
    Logic,
    #[strum(serialize = "time")]
    Time,
    #[strum(serialize = "string")]
    String,
    #[strum(serialize = "assign")]
    Assign,
    #[strum(serialize = "var")]
    Var,
    #[strum(serialize = "enum")]
    EnumeratedType,

    // Keywords: Parameters
    #[strum(serialize = "parameter")]
    Parameter,
    #[strum(serialize = "localparam")]
    LocalParam,
    #[strum(serialize = "specparam")]
    SpecParam,
    #[strum(serialize = "defparam")]
    DefParam,

    // Keywords: Nets
    #[strum(serialize = "nettype")]
    NetType,
    #[strum(serialize = "wire")]
    Wire,
    #[strum(serialize = "wand")]
    Wand,
    #[strum(serialize = "wor")]
    Wor,
    #[strum(serialize = "uwire")]
    UWire,
    #[strum(serialize = "tri")]
    Tri,
    #[strum(serialize = "triand")]
    TriAnd,
    #[strum(serialize = "trior")]
    TriOr,
    #[strum(serialize = "tri0")]
    Tri0,
    #[strum(serialize = "tri1")]
    Tri1,
    #[strum(serialize = "trireg")]
    TriReg,
    #[strum(serialize = "supply0")]
    Supply0,
    #[strum(serialize = "supply1")]
    Supply1,
    #[strum(serialize = "interconnect")]
    InterConnect,

    // Keywords: Signal Strength
    #[strum(serialize = "small")]
    Small,
    #[strum(serialize = "medium")]
    Medium,
    #[strum(serialize = "large")]
    Large,

    // Keywords: Packages
    #[strum(serialize = "package")]
    Package,
    #[strum(serialize = "endpackage")]
    EndPackage,
    #[strum(serialize = "import")]
    Import,

    // Keywords: Functions
    #[strum(serialize = "function")]
    Function,
    #[strum(serialize = "automatic")]
    Automatic,
    #[strum(serialize = "input")]
    Input,
    #[strum(serialize = "output")]
    Output,
    #[strum(serialize = "inout")]
    InOut,
    #[strum(serialize = "endfunction")]
    EndFunction,
    #[strum(serialize = "return")]
    Return,

    // Keywords: Modules
    #[strum(serialize = "module")]
    Module,
    #[strum(serialize = "endmodule")]
    EndModule,

    // Keywords: Tasks
    #[strum(serialize = "task")]
    Task,
    #[strum(serialize = "endtask")]
    EndTask,

    // Keywords: Configs
    #[strum(serialize = "config")]
    Config,
    #[strum(serialize = "endconfig")]
    EndConfig,

    // Keywords: Clocking
    #[strum(serialize = "posedge")]
    PosEdge,
    #[strum(serialize = "negedge")]
    NegEdge,
    #[strum(serialize = "edge")]
    Edge,
    #[strum(serialize = "always")]
    Always,
    #[strum(serialize = "always_comb")]
    AlwaysComb,
    #[strum(serialize = "always_latch")]
    AlwaysLatch,
    #[strum(serialize = "always_ff")]
    AlwaysFF,
    #[strum(serialize = "initial")]
    Initial,

    // Keywords: Properties
    #[strum(serialize = "property")]
    Property,
    #[strum(serialize = "endproperty")]
    EndProperty,

    // Keywords: Program
    #[strum(serialize = "program")]
    Program,
    #[strum(serialize = "endprogram")]
    EndProgram,

    // Keywords: Generate
    #[strum(serialize = "generate")]
    Generate,
    #[strum(serialize = "endgenerate")]
    EndGenerate,

    // Keywords: Specify
    #[strum(serialize = "specify")]
    Specify,
    #[strum(serialize = "endspecify")]
    EndSpecify,

    // Keywords: Loops
    #[strum(serialize = "forever")]
    Forever,
    #[strum(serialize = "repeat")]
    Repeat,
    #[strum(serialize = "for")]
    For,
    #[strum(serialize = "foreach")]
    ForEach,
    #[strum(serialize = "while")]
    While,
    #[strum(serialize = "do")]
    Do,
    #[strum(serialize = "break")]
    Break,
    #[strum(serialize = "continue")]
    Continue,

    // Keywords: Conditionals
    #[strum(serialize = "if")]
    If,
    #[strum(serialize = "else")]
    Else,
    #[strum(serialize = "case")]
    Case,
    #[strum(serialize = "endcase")]
    EndCase,
    #[strum(serialize = "casez")]
    CaseZ,
    #[strum(serialize = "casex")]
    CaseX,
    #[strum(serialize = "unique")]
    Unique,
    #[strum(serialize = "unique0")]
    Unique0,
    #[strum(serialize = "priority")]
    Priority,
    #[strum(serialize = "matches")]
    Matches,
    #[strum(serialize = "tagged")]
    Tagged,

    // Keywords: Interprocess Communication
    #[strum(serialize = "semaphore")]
    Semaphore,
    #[strum(serialize = "mailbox")]
    Mailbox,
    #[strum(serialize = "wait")]
    Wait,
    #[strum(serialize = "wait_order")]
    WaitOrder,
    #[strum(serialize = "event")]
    Event,
    #[strum(serialize = "fork")]
    Fork,
    #[strum(serialize = "join_any")]
    JoinAny,
    #[strum(serialize = "join_none")]
    JoinNone,
    #[strum(serialize = "join")]
    Join,

    // Keywords: Assertions
    #[strum(serialize = "assert")]
    Assert,
    #[strum(serialize = "assume")]
    Assume,
    #[strum(serialize = "restrict")]
    Restrict,
    #[strum(serialize = "final")]
    Final,
    #[strum(serialize = "cover")]
    Cover,

    // Keywords: Etc.
    #[strum(serialize = "library")]
    Library,
    #[strum(serialize = "default")]
    Default,
    #[strum(serialize = "timeunit")]
    TimeUnit,
    #[strum(serialize = "inside")]
    Inside,
    #[strum(serialize = "dist")]
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
