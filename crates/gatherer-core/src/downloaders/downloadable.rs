use crate::{downloaders::DownloaderErrors, gatherers::Media, AsyncResult};
use async_fs::File;
use futures_lite::{
    io::{copy, AsyncWriteExt, BufReader},
    stream, StreamExt,
};
use std::{
    fmt::Display,
    path::{Path, PathBuf},
    str::FromStr,
};
use surf::{
    http::{
        headers::{HeaderValue, CONTENT_LENGTH, CONTENT_RANGE},
        Method,
    },
    Request, StatusCode, Url,
};
use tracing::{debug, error, info, trace};

#[derive(Debug)]
pub struct Downloadable {
    pub public_url: String,
    pub file_name: String,
    pub base_path: PathBuf,
}

impl Display for Downloadable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let path = self.base_path.join(&self.file_name[..]);
        write!(f, "{:?}", path)
    }
}

const DEFAULT_BUFFER_SIZE: u32 = 1024; // ~1 mb
const DEFAULT_MIN_SIZE_TO_CHUNK: u64 = (100 * 1024) * 1024; // roughly 100 mb
const TEST_AGENT: &str = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/93.0.4577.82 Safari/537.36";

impl Downloadable {
    pub async fn save_item(
        &self,
        chunk_size: Option<u32>,
        min_size_to_chunk: Option<u64>,
    ) -> AsyncResult<u64> {
        let min_size_to_chunk = min_size_to_chunk.unwrap_or(DEFAULT_MIN_SIZE_TO_CHUNK);
        let chunk_sizes = chunk_size.unwrap_or(DEFAULT_BUFFER_SIZE);
        let file_path = &self.get_file_path();

        let mut output_file = if file_path.exists() {
            debug!("opening file: {:?}", file_path);
            async_fs::File::open(file_path).await
        } else {
            debug!("creating file: {:?}", file_path);
            async_fs::create_dir_all(file_path.parent().unwrap()).await?;
            async_fs::File::create(file_path).await
        }?;
        output_file.sync_all().await?;

        let resp = surf::head(&self.public_url).send().await?;
        let content_length: u64 = if let Some(cl_header) = resp.header(CONTENT_LENGTH) {
            cl_header.as_str().parse().unwrap()
        } else {
            0
        };

        // let mut output_file = async_fs::File::create(file_path).await?;

        debug!(
            "{:?} current size {}, expected length {}",
            file_path,
            output_file.metadata().await?.len(),
            content_length
        );
        if output_file.metadata().await?.len().eq(&content_length) {
            return Ok(u64::MIN);
        };

        // if content_length > DEFAULT_MIN_SIZE_TO_CHUNK {
        //     info!(
        //         "Downloading {} in {} byte chunks sized {} < {}",
        //         self.file_name, DEFAULT_BUFFER_SIZE, content_length, DEFAULT_MIN_SIZE_TO_CHUNK
        //     );
        //     let mut total_written_bytes: u64 = 0;
        //     // Iterate through the content length, get chunks of data to write instead of a full buffer in memory
        //     for range in PartialRangeIter::new(0, content_length - 1, DEFAULT_BUFFER_SIZE)? {
        //         debug!("Getting chunk range {:?} of {}", range, content_length);
        //         match surf::get(&self.public_url)
        //             .header(CONTENT_RANGE, range.as_str())
        //             .await
        //         {
        //             Ok(mut data_chunk) => {
        //                 match futures::io::copy(&mut data_chunk, &mut output_file).await {
        //                     Ok(bytes_written) => total_written_bytes += bytes_written,
        //                     Err(err) => {
        //                         error!("Failed to write {:?} bytes to {:?}", range, file_path);
        //                         return Err(Box::new(err));
        //                     }
        //                 }
        //             }
        //             Err(chunk_err) => error!(
        //                 "Failed to get chunk of {} sized {}. {:?}",
        //                 &self.file_name, DEFAULT_BUFFER_SIZE, chunk_err
        //             ),
        //         };
        //     }
        //     Ok(total_written_bytes)
        // } else {
        debug!(
            "Downloading {} all at once sized {} < {}",
            self.file_name, content_length, DEFAULT_MIN_SIZE_TO_CHUNK
        );

        match surf::get(&self.public_url).await {
            Ok(mut resp) => {
                debug!("Download response for {} {:?}", &self.public_url, resp);
                match futures::io::copy(&mut resp, &mut output_file).await {
                    Ok(bytes_written) => Ok(bytes_written),
                    Err(copy_err) => Err(format!(
                        "Failed to copy bytes to file {:?}. {:?}",
                        file_path, copy_err
                    )
                    .into()),
                }
            }
            Err(req_err) => {
                Err(format!("Request to {} failed. {}", &self.public_url, req_err).into())
            }
        }
        // }
    }

    pub fn get_file_path(&self) -> PathBuf {
        self.base_path.join(&self.file_name)
    }

    pub fn from_media_with_path(media: &'_ Media, path: PathBuf) -> Self {
        debug!(
            "Creating downloadable for {} in {:?}",
            media.file_name, path
        );
        Self {
            file_name: media.file_name.to_string(),
            base_path: path,
            public_url: media.url.to_string(),
        }
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
    pub fn new(start: u64, end: u64, buffer_size: u32) -> AsyncResult<Self> {
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
                    debug!("{{PartialRangeIter}}: sent a header `{:?}`", hv);
                    Some(hv)
                }
                Err(err) => {
                    error!(
                        "{{PartialRangeIter}}: failed to create header from string bytes({}) [{:?}]. Error: {:?}",
                        hs.len(),
                        hs,
                        err
                    );
                    None
                }
            }
            // Some(
            //     HeaderValue::from_bytes(hs_bytes)
            //         .unwrap_or_else(|_| panic!("Unable to create header for {}", hs)),
            // )
        }
    }
}
