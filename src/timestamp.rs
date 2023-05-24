use rust_decimal::Decimal;
#[derive(Clone)]
pub struct Timestamp {
    pub hour: u8,
    pub amount: Decimal,
}
