pub struct IntegerLiteral {
    pub value: i64,
}

pub struct InfixExpression {
    pub left: Box<Expression>,
    pub operator: String,
    pub right: Box<Expression>,
}
