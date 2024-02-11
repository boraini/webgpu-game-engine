use std::sync::Mutex;

use lazy_static::lazy_static;

lazy_static! {
    static ref LAST_TIME: Mutex<f64> = Mutex::from(-1.0);
}

pub fn get_frame_delta(time: f64) -> f64 {
    let mut last_time = LAST_TIME.lock().unwrap();
    let delta = if *last_time < 0.0 {
        0.0
    } else {
        time - *last_time
    };
    *last_time = time;
    delta
}
