
// src/lib.rs
#[path = "messages/utils/mod.rs"]
mod util;

#[allow(unused_imports)]
use util::{side_as_int, order_type_to_char};

#[allow(unused_imports)]
use quickfix_msg44::field_types::{OrdType, Side};

#[cfg(test)]
mod tests {
    use quickfix_msg44::field_types::Side;

    use super::*;

    #[test]
    fn test_order_type_market() {
        let order_type: OrdType = OrdType::Market;
        let order_type_char = order_type_to_char(order_type);
        assert_eq!(order_type_char, '1');
    }

    #[test]
    fn test_order_type_limit() {
        let order_type: OrdType = OrdType::Limit;
        let order_type_char = order_type_to_char(order_type);
        assert_eq!(order_type_char, '2');
    }

    #[test]
    fn test_side_buy() {
        let side: Side = Side::Buy;
        let side_int = side_as_int(side);
        assert_eq!(side_int, 1);
    }

    #[test]
    fn test_side_sell() {
        let side: Side = Side::Sell;
        let side_int = side_as_int(side);
        assert_eq!(side_int, 2);
    }
}

