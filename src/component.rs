#[derive(PartialEq, PartialOrd)]
pub enum ComponentPrecedence {
    LowerBound,
    PreRelease,
    Zero,
    PostRelease,
    NonZero,
    LetterSuffix,
    UpperBound,
}

pub struct Component<'a> {
    pub precedence: ComponentPrecedence,
    pub value: &'a str,
}
