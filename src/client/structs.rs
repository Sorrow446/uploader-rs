use std::io;
use std::io::Read;
use indicatif::{ProgressBar, ProgressStyle};
use indicatif::style::TemplateError;

pub(crate) struct ProgressReader<R> {
    inner: R,
    pb: ProgressBar,
}

impl<R: Read> ProgressReader<R> {
    pub(crate) fn new(inner: R, total_size: u64) -> Result<Self, TemplateError> {
        let pb = ProgressBar::new(total_size);
        pb.set_style(ProgressStyle::with_template("[{elapsed_precise}] [{bar:40.cyan/blue}] {percent}% at {binary_bytes_per_sec}, {bytes}/{total_bytes} (ETA: {eta})")?
            .progress_chars("#>-"));

        Ok(ProgressReader {
            inner,
            pb,
        })
    }
}

impl<R: Read> Read for ProgressReader<R> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let n = self.inner.read(buf)?;
        let pos = self.pb.position() + n as u64;
        self.pb.set_position(pos);
        Ok(n)
    }
}

impl<R> Drop for ProgressReader<R> {
    fn drop(&mut self) {
        self.pb.finish();
    }
}