// src/lib.rs
#[path = "messages/utils/mod.rs"]
mod util;

#[cfg(test)]
mod fix_msg_tests {

    use quickfix::Message;

    #[test]
    fn test_message_equality() {
        let fix_msg_txt = "8=FIX.4.2|9=12|35=D|49=CLIENT|56=SERVER|34=2|52=20230612-12:34:56|".replace('|', "\x01");
        let msg1: Message = Message::try_from_text(&fix_msg_txt).unwrap();
        let msg2: Message = Message::try_from_text(&fix_msg_txt).unwrap();
        assert_eq!(msg1.to_fix_string(), msg2.to_fix_string());
    }
    #[test]
    fn test_message_inequality() {
        let fix_msg_txt1 = "8=FIX.4.2|9=12|35=D|49=CLIENT|56=SERVER|34=2|52=20230612-12:34:56|".replace('|', "\x01");
        let fix_msg_txt2 = "8=FIX.4.2|9=12|35=A|49=CLIENT|56=SERVER|34=2|52=20230612-12:34:56|".replace('|', "\x01");
        let msg1: Message = Message::try_from_text(&fix_msg_txt1).unwrap();
        let msg2: Message = Message::try_from_text(&fix_msg_txt2).unwrap();
        assert_ne!(msg1.to_fix_string(), msg2.to_fix_string());
    }
}

#[cfg(test)]
mod fix_msg_enum_tests {

    use super::*;
    use quickfix_msg44::field_types::{OrdType, Side, TimeInForce};

    #[allow(unused_imports)]
    use util::{side_as_int, order_type_to_char};

    #[test]
    fn test_time_in_force_day() {
        let tif = TimeInForce::Day;
        assert_eq!(tif, TimeInForce::Day);
    }

    #[test]
    fn test_time_in_force_good_till_cancel() {
        let tif = TimeInForce::GoodTillCancel;
        assert_eq!(tif, TimeInForce::GoodTillCancel);
    }

    #[test]
    fn test_time_in_force_at_the_opening() {
        let tif = TimeInForce::AtTheOpening;
        assert_eq!(tif, TimeInForce::AtTheOpening);
    }

    #[test]
    fn test_time_in_force_immediate_or_cancel() {
        let tif = TimeInForce::ImmediateOrCancel;
        assert_eq!(tif, TimeInForce::ImmediateOrCancel);
    }

    #[test]
    fn test_time_in_force_fill_or_kill() {
        let tif = TimeInForce::FillOrKill;
        assert_eq!(tif, TimeInForce::FillOrKill);
    }

    #[test]
    fn test_time_in_force_good_till_crossing() {
        let tif = TimeInForce::GoodTillCrossing;
        assert_eq!(tif, TimeInForce::GoodTillCrossing);
    }

    #[test]
    fn test_time_in_force_good_till_date() {
        let tif = TimeInForce::GoodTillDate;
        assert_eq!(tif, TimeInForce::GoodTillDate);
    }

    #[test]
    fn test_time_in_force_debug_format() {
        let tif = TimeInForce::Day;
        assert_eq!(format!("{:?}", tif), "Day");
    }

    #[test]
    fn test_time_in_force_equality() {
        assert_eq!(TimeInForce::Day, TimeInForce::Day);
        assert_ne!(TimeInForce::Day, TimeInForce::GoodTillCancel);
    }

    #[test]
    fn test_time_in_force_clone() {
        let tif = TimeInForce::FillOrKill;
        let tif_clone = tif.clone();
        assert_eq!(tif, tif_clone);
    }

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

