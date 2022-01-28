use regex::Regex;

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
        match s.body_string().await {
            Ok(body) => match serde_json::from_str(&body) {
                Ok(res) => Ok(res),
                Err(serde_err) => {
                    Err(format!("Failed to read body {} as json. {:?}", body, serde_err).into())
                }
            },
            Err(err) => Err(format!("Failed to get the response body: {:?}", err).into()),
        }
    }
    /// Give an opportunity to strip odd characters out of the JSON before sending through Serde
    pub async fn as_json_with_strip<T>(self, re: Option<&'_ Regex>) -> crate::Result<T>
    where
        T: for<'de> serde::Deserialize<'de>,
    {
        let mut s = self.0;
        match s.body_string().await {
            Ok(body) => {
                let mut body_json = body;
                if let Some(re) = re {
                    body_json = re.replace_all(&body_json, "").to_string();
                }
                match serde_json::from_str(&body_json) {
                    Ok(res) => Ok(res),
                    Err(serde_err) => Err(format!(
                        "Failed to read body {} as json. {:?}",
                        body_json, serde_err
                    )
                    .into()),
                }
            }
            Err(err) => Err(format!("Failed to get the response body: {:?}", err).into()),
        }
    }
}
