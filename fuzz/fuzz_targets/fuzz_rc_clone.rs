use honggfuzz::fuzz;
use rclite::Rc;

fn main() {
    loop {
        fuzz!(|data: &[u8]| {
            if data.len() < 2 {
                return;
            }

            let rc = Rc::new(data.to_vec());
            let _ = Rc::clone(&rc);
        });
    }
}