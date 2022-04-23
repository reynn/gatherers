use async_trait::async_trait;

pub struct InMemoryFileDownloader;

#[async_trait]
impl super::FileDownloader for InMemoryFileDownloader {
    async fn download(&self, url: &'_ str, output_path: std::path::PathBuf) -> crate::Result<u64> {
        let mut output_file = if output_path.exists() {
            log::debug!("opening file: {:?}", output_path);
            async_fs::File::open(output_path).await
        } else {
            log::debug!("creating file: {:?}", output_path);
            async_fs::create_dir_all(output_path.parent().unwrap()).await?;
            async_fs::File::create(output_path).await
        }?;
        let client = surf::Client::default();
        let req = client.get(&url).build();
        match client.send(req).await {
            Ok(mut resp) => {
                log::debug!("Download response for {} {:?}", url, resp);
                match futures::io::copy(&mut resp, &mut output_file).await {
                    Ok(bytes_written) => Ok(bytes_written),
                    Err(copy_err) => {
                        let copy_err: std::io::ErrorKind = copy_err.kind();
                        match copy_err {
                            std::io::ErrorKind::WriteZero | std::io::ErrorKind::AlreadyExists => {
                                Ok(0)
                            }
                            _ => Err(format!(
                                "Failed to copy bytes to file {:?}. {:?}",
                                output_file, copy_err
                            )
                            .into()),
                        }
                    }
                }
            }
            Err(req_err) => Err(format!("in-mem: Request to {} failed. {}", &url, req_err).into()),
        }
    }
}
