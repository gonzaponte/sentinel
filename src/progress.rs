use indicatif::{ProgressBar, ProgressStyle};

pub struct MaybeProgressBar(Option<ProgressBar>);

impl MaybeProgressBar {
    pub fn new(nbatch: usize, batch_mode: bool) -> Self {
        if batch_mode { return Self(None); }

        let pb = ProgressBar::new(nbatch as u64);
        pb.set_style(
            ProgressStyle::with_template(
                "{spinner:.magenta} [{elapsed_precise}] \
                 [{bar:40.bold.#c000ff/#5a00aa}] \
                 {pos:>7}/{len:7} \
                 {percent:>3}% \
                 ETA {eta_precise}"
            ).unwrap()
             .progress_chars("█▉▊▋▌▍▎▏ ") // other: "█▓░" "█▇▆▅▄▃▂▁ "
        );
        pb.reset(); // force drawing
        Self(Some(pb))
    }

    pub fn inc(&self, n: u64) {
        if let Some(pb) = &self.0 {
            pb.inc(n);
        }
    }

    pub fn finish(&self) {
        if let Some(pb) = &self.0 {
            pb.finish();
        }
    }

    pub fn reset(&self) {
        if let Some(pb) = &self.0 {
            pb.reset();
        }
    }
}
