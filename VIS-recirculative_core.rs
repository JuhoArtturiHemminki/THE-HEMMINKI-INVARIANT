use std::f64::consts::PI;
use std::time::{Duration, Instant};
use std::thread::sleep;

const PHI: f64 = 1.618033988749895;
const HEMMINKI_CONSTANT: f64 = 5.0832;
const RECOVERY_RATE: f64 = 0.005;

pub struct HemminkiCore {
    start_time: Instant,
    e_drift: f64,
}

impl HemminkiCore {
    pub fn init() -> Self {
        Self {
            start_time: Instant::now(),
            e_drift: 0.0,
        }
    }

    #[inline(always)]
    fn phi_interval(&self, n: u64) -> Duration {
        let phase = (n as f64 * PHI) % (2.0 * PI);
        Duration::from_nanos((phase * 1000.0) as u64)
    }

    fn self_repair(&mut self, duration: Duration) {
        let recovery = duration.as_nanos() as f64 * RECOVERY_RATE / 1000.0;
        self.e_drift = (self.e_drift - recovery).max(0.0);
    }

    pub fn execute_transduction(&mut self, bit_stream: &[u8]) -> Result<f64, String> {
        for (n, &bit) in bit_stream.iter().enumerate() {
            let window = self.phi_interval(n as u64);
            
            self.self_repair(window);
            sleep(window / 500); 

            let step_drift = if bit == 1 { 0.042 } else { 0.011 };
            self.e_drift += step_drift;

            if self.e_drift > HEMMINKI_CONSTANT {
                return Err(format!("Geometric Violation at bit {}: E_drift {:.4}", n, self.e_drift));
            }
        }
        Ok(self.e_drift)
    }
}

fn main() {
    let mut core = HemminkiCore::init();
    let data = vec![1, 0, 1, 1, 0, 0, 1, 0, 1, 0, 1, 1, 1, 0, 0, 1];

    match core.execute_transduction(&data) {
        Ok(final_drift) => {
            println!("W_rec Active. Stable Determinism. Final E_drift: {:.4}", final_drift);
        }
        Err(e) => {
            eprintln!("System Failure: {}", e);
        }
    }
}
