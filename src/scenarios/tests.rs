#[cfg(test)]
mod scenario_tests {
    use mockall::predicate::*;
    use mockall::*;
    use quickfix_msg44::field_types::{OrdType, Side};
    use std::{io::{Read, Write}, sync::{Arc, Mutex}};

    use crate::{messages::factory::FixMessageFactory, scenarios::single_leg_order::send_single_order, config::Settings};

    mock! {
        #[derive(Debug)]
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
            .times(1..) // Expect at least 1 write call
            .returning(|buf| Ok(buf.len()));

        stream
            .expect_read()
            .times(1..) // Expect at least 1 read call
            .returning(|buf| {
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
        ).unwrap();

        // Create a test settings object with cancel_order set to false
        let settings = Settings {
            symbol: "BTC-USD".to_string(),
            price: 1.0,
            quantity: 1.0,
            side: "Buy".to_string(),
            order_type: "Limit".to_string(),
            scenario: "ORDER".to_string(),
            server: "test.server.com".to_string(),
            heartbeat_interval: 30,
            heartbeat_count: 10,
            listen_epoch: 42,
            publish_epoch: 24,
            cancel_order: false,
            log_file: "test.log".to_string(),
            target_comp_id: "TEST-ID".to_string(),
        };

        send_single_order(stream, order, Some(&settings));
    }

    #[test]
    fn test_send_single_order_with_cancel() {
        let mut stream = MockStream::new();

        stream
            .expect_write()
            .times(1..)
            .returning(|buf| Ok(buf.len()));

        stream
            .expect_read()
            .times(1..)
            .returning(|buf| {
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
        ).unwrap();

        // Create a test settings object with cancel_order set to true
        let settings = Settings {
            symbol: "BTC-USD".to_string(),
            price: 1.0,
            quantity: 1.0,
            side: "Buy".to_string(),
            order_type: "Limit".to_string(),
            scenario: "ORDER".to_string(),
            server: "test.server.com".to_string(),
            heartbeat_interval: 30,
            heartbeat_count: 10,
            listen_epoch: 42,
            publish_epoch: 24,
            cancel_order: true,
            log_file: "test.log".to_string(),
            target_comp_id: "TEST-ID".to_string(),
        };

        send_single_order(stream, order, Some(&settings));
    }

    #[test]
    fn test_rfq_publish_fix() {
        let msgs = "8=FIX.4.4\x0135=8\x01".to_string();
        let res = super::super::single_leg_order::split_fix_messages(&msgs);
        assert_eq!(res.len(), 1);
    }
}
