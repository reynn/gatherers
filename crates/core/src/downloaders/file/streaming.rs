use {crate::Result, async_trait::async_trait, surf::http::headers::HeaderValue};

pub struct StreamingFileDownloader;

#[async_trait]
impl super::FileDownloader for StreamingFileDownloader {
    async fn download(&self, _url: &'_ str, _output_path: std::path::PathBuf) -> Result<u64> {
        todo!()
    }
}

// Initial version from Rust Cookbook
//
// https://rust-lang-nursery.github.io/rust-cookbook/web/clients/download.html#make-a-partial-download-with-http-range-headers
struct PartialRangeIter {
    start: u64,
    end: u64,
    buffer_size: u32,
}

impl PartialRangeIter {
    #[allow(unused)]
    pub fn new(start: u64, end: u64, buffer_size: u32) -> Result<Self> {
        if buffer_size == 0 {
            return Err("invalid buffer_size, give a value greater than zero.".into());
        }
        Ok(PartialRangeIter {
            start,
            end,
            buffer_size,
        })
    }
}

impl Iterator for PartialRangeIter {
    type Item = HeaderValue;
    fn next(&mut self) -> Option<Self::Item> {
        if self.start > self.end {
            None
        } else {
            let prev_start = self.start;
            self.start += std::cmp::min(self.buffer_size as u64, self.end - self.start + 1);
            let hs = format!("bytes={}-{}", prev_start, self.start - 1);
            let hs_bytes = hs.as_bytes().to_vec();
            match HeaderValue::from_bytes(hs_bytes) {
                Ok(hv) => {
                    log::debug!("{{PartialRangeIter}}: sent a header `{:?}`", hv);
                    Some(hv)
                }
                Err(err) => {
                    log::error!(
                        "{{PartialRangeIter}}: failed to create header from string bytes({}) [{:?}]. Error: {:?}",
                        hs.len(),
                        hs,
                        err
                    );
                    None
                }
            }
        }
    }
}
