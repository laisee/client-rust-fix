#[cfg(test)]
mod scenario_tests {
    use super::*;
    use mockall::predicate::*;
    use mockall::*;
    use quickfix::Message;
    use quickfix_msg44::field_types::{OrdType, Side};
    use std::sync::{Arc, Mutex};

    mock! {
        Stream {}
        impl Write for Stream {
            fn write(&mut self, buf: &[u8]) -> std::io::Result<usize>;
            fn flush(&mut self) -> std::io::Result<()>;
        }
        impl Read for Stream {
            fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize>;
        }
    }

    #[test]
    fn test_send_single_order_success() {
        let mut stream = MockStream::new();
        stream
            .expect_write()
            .times(1)
            .returning(|buf| Ok(buf.len()));
        stream.expect_read().times(1).returning(|buf| {
            let resp = b"8=FIX.4.4\x0135=8\x0139=0\x0137=EXCH\x0111=ID\x01";
            buf[..resp.len()].copy_from_slice(resp);
            Ok(resp.len())
        });
        stream.expect_flush().returning(|| Ok(()));
        let stream = Arc::new(Mutex::new(stream));

        let order = FixMessageFactory::new_single_leg_order(
            "key".to_string(),
            1.0,
            1.0,
            "BTC-USD".to_string(),
            Side::Buy,
            OrdType::Limit,
            1,
        )
        .unwrap();

        send_single_order("key", stream, order, 1, Some(false));
    }

    #[test]
    fn test_send_single_order_with_cancel() {
        let mut stream = MockStream::new();
        stream
            .expect_write()
            .times(2)
            .returning(|buf| Ok(buf.len()));
        stream.expect_read().times(2).returning(|buf| {
            let resp = b"8=FIX.4.4\x0135=8\x0139=0\x0137=EXCH\x0111=ID\x01";
            buf[..resp.len()].copy_from_slice(resp);
            Ok(resp.len())
        });
        stream.expect_flush().returning(|| Ok(()));
        let stream = Arc::new(Mutex::new(stream));

        let order = FixMessageFactory::new_single_leg_order(
            "key".to_string(),
            1.0,
            1.0,
            "BTC-USD".to_string(),
            Side::Buy,
            OrdType::Limit,
            1,
        )
        .unwrap();

        send_single_order("key", stream, order, 1, Some(true));
    }

    #[test]
    fn test_rfq_publish_fix() {
        let msgs = "8=FIX.4.4\x0135=8\x01".to_string();
        let res = super::super::single_leg_order::split_fix_messages(&msgs);
        assert_eq!(res.len(), 1);
    }
}
