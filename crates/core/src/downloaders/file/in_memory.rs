use async_trait::async_trait;
use eyre::eyre;
use std::fs::{create_dir_all, File};

pub struct InMemoryFileDownloader;

#[async_trait]
impl super::FileDownloader for InMemoryFileDownloader {
    async fn download(&self, url: &'_ str, output_path: std::path::PathBuf) -> crate::Result<u64> {
        let mut output_file = if output_path.exists() {
            log::debug!("opening file: {:?}", output_path);
            File::open(output_path)
        } else {
            log::debug!("creating file: {:?}", output_path);
            create_dir_all(output_path.parent().unwrap())?;
            File::create(output_path)
        }?;
        let client = reqwest::Client::default();
        let req = client.get(url).build()?;
        match client.execute(req).await {
            Ok(mut resp) => {
                log::debug!("Download response for {} {:?}", url, resp);
                // std::io::copy(&mut resp., &mut output_file).unwrap();
                let content = resp.text().await?;
                match std::io::copy(&mut content.as_bytes(), &mut output_file) {
                    Ok(written) => Ok(written),
                    Err(copy_err) => match copy_err.kind() {
                        std::io::ErrorKind::AlreadyExists | std::io::ErrorKind::WriteZero => Ok(0),
                        _ => Err(eyre!("{:?}", copy_err)),
                    },
                }
                // match futures::io::copy(&mut resp_bytes, &mut output_file).await {
                //     Ok(bytes_written) => Ok(bytes_written),
                //     Err(copy_err) => {
                //         let copy_err: std::io::ErrorKind = copy_err.kind();
                //         match copy_err {
                //             std::io::ErrorKind::WriteZero | std::io::ErrorKind::AlreadyExists => {
                //                 Ok(0)
                //             }
                //             _ => Err(eyre!(
                //                 "Failed to copy bytes to file {:?}. {:?}",
                //                 output_file,
                //                 copy_err
                //             )),
                //         }
                //     }
                // }
            }
            Err(req_err) => Err(eyre!("in-mem: Request to {} failed. {}", &url, req_err)),
        }
    }
}
