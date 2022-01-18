#[derive(Debug)]
pub struct Response(surf::Response);

impl Response {
    pub fn from_surf(surf: surf::Response) -> Self {
        Self(surf)
    }

    pub fn status(&self) -> surf::StatusCode {
        self.0.status()
    }

    pub fn get_header(&self, name: &'_ str) -> Option<&'_ str> {
        let header = self.0.header(name);
        if let Some(header) = header {
            Some(header.as_str())
        } else {
            None
        }
    }

    pub async fn as_bytes(self) -> crate::Result<Vec<u8>> {
        let mut s = self.0;
        Ok(s.body_bytes().await?)
    }

    pub async fn as_string(self) -> crate::Result<String> {
        let mut s = self.0;
        Ok(s.body_string().await?)
    }

    pub async fn as_json<T>(self) -> crate::Result<T>
    where
        T: for<'de> serde::Deserialize<'de>,
    {
        let mut s = self.0;
        match s.body_bytes().await {
            Ok(bytes) => {
                // let bytes = bytes.to_ascii_lowercase();
                match String::from_utf8(bytes) {
                    Ok(body) => match serde_json::from_str(&body) {
                        Ok(res) => Ok(res),
                        Err(serde_err) => Err(format!(
                            "Failed to read body {} as json. {:?}",
                            body, serde_err
                        )
                        .into()),
                    },
                    Err(from_utf8_err) => Err(format!(
                        "Failed to read response body as a UTF-8 string. {:?}",
                        from_utf8_err
                    )
                    .into()),
                }
            }
            Err(err) => Err(format!("Failed to get the respone body as bytes: {:?}", err).into()),
        }
    }
}
