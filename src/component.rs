#[derive(PartialEq, PartialOrd, Debug)]
pub enum ComponentPrecedence {
    LowerBound,
    PreRelease,
    Zero,
    PostRelease,
    NonZero,
    LetterSuffix,
    UpperBound,
}

#[derive(Debug)]
pub struct Component<'a> {
    pub precedence: ComponentPrecedence,
    pub value: &'a str,
}
