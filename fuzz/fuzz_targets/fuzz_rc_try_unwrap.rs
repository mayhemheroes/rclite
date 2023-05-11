use honggfuzz::fuzz;
use rclite::Rc;

fn main() {
    loop {
        fuzz!(|data: &[u8]| {
            if data.len() < 2 {
                return;
            }

            let rc = Rc::new(data.to_vec());

            if let Ok(unwrapped_rc) = Rc::try_unwrap(rc) {
                assert_eq!(unwrapped_rc, data);
            }
        });
    }
}