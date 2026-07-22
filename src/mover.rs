use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use crate::mouse;

struct Lcg(u64);

impl Lcg {
    fn seeded() -> Self {
        use std::time::{SystemTime, UNIX_EPOCH};
        let seed = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos() as u64;
        Self(seed ^ 0x517cc1b727220a95)
    }

    fn next_f64(&mut self) -> f64 {
        self.0 = self
            .0
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        (self.0 >> 11) as f64 / (1u64 << 53) as f64
    }
}

pub fn spawn_mover(flag: Arc<Mutex<bool>>) {
    thread::spawn(move || {
        let mut rng = Lcg::seeded();
        let (sw, sh) = mouse::screen_size();

        loop {
            thread::sleep(Duration::from_secs(5));

            if !*flag.lock().unwrap() {
                break;
            }

            let x = rng.next_f64() * sw;
            let y = rng.next_f64() * sh;

            let (sx, sy) = mouse::cursor_position().unwrap_or((x, y));

            // Move along a smooth path so cursor motion looks natural.
            let duration = Duration::from_millis(800);
            let frame = Duration::from_millis(16);
            let steps = (duration.as_millis() / frame.as_millis()).max(1) as u32;

            for step in 1..=steps {
                if !*flag.lock().unwrap() {
                    break;
                }

                let t = step as f64 / steps as f64;
                let eased = t * t * (3.0 - 2.0 * t);
                let ix = sx + (x - sx) * eased;
                let iy = sy + (y - sy) * eased;
                mouse::move_cursor(ix, iy);
                thread::sleep(frame);
            }
        }
    });
}
