use honggfuzz::fuzz;
use rclite::Arc;

fn main() {
    loop {
        fuzz!(|data: &[u8]| {
            if data.len() < 2 {
                return;
            }

            let arc = Arc::new(data.to_vec());

            if let Ok(unwrapped_arc) = Arc::try_unwrap(arc) {
                assert_eq!(unwrapped_arc, data);
            }
        });
    }
}