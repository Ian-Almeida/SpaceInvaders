use std::ops::Mul;
use std::time::{Duration, Instant};
use std::thread;

pub struct Game {
    // Renderer attributes
    start_time: Instant,
    frame_count: u32,
    frame_time: Duration,


}

impl Default for Game {
    fn default() -> Self {
        Game {
            start_time: Instant::now(),
            frame_count: 0,
            frame_time: Duration::from_secs_f64(1.0 / 60.0) // 60 fps
        }
    }
}

impl Game {

    pub fn start_game(&mut self) {
        //Main game loop
        loop {
            // Render the frame
            render_frame();

            // Update the frame count
            self.frame_count += 1;

            // Wait for the next frame
            let elapsed_time = self.start_time.elapsed();
            let target_time = self.frame_time.mul(self.frame_count as u32);
            if elapsed_time < target_time {
                thread::sleep(target_time - elapsed_time);
            }

            // Calculate the frame rate
            if let Some(frame_rate) = calculate_frame_rate(self.frame_count, elapsed_time) {
                println!("Frame rate: {:.2} fps", frame_rate);
            }
        }

    }
}

fn render_frame() {
    // unimplemented!("Render the frame");
}

fn calculate_frame_rate(frame_count: u32, elapsed_time: Duration) -> Option<f64> {
    if frame_count > 1 {
        let prev_time = elapsed_time.checked_sub(Duration::from_secs(1))?;
        let frame_rate = frame_count as f64 / prev_time.as_secs_f64();
        Some(frame_rate)
    } else {
        None
    }
}