macro_rules! http_request {
    ($methd:ident) => {
        /// Makes a [`stringify!($metd)`] request using the underyling HTTP client
        pub async fn $methd<U>(
            &self,
            endpoint: U,
            headers: Option<Headers>,
            // body: Option<B>,
        ) -> crate::Result<crate::http::Response>
        where
            U: AsRef<str>,
        {
            let request: surf::Request = self.client.$methd(endpoint).build();
            Ok(self.execute(headers, Request::from(request)).await?)
        }
    };
}

macro_rules! http_request_with_body {
    ($methd:ident) => {
        /// Makes a HTTP request for the same HTTP method as the function name
        ///
        /// Uses the underyling HTTP client when making requests
        pub async fn $methd<U, B>(
            &self,
            endpoint: U,
            headers: Option<Headers>,
            body: Option<B>,
        ) -> crate::Result<crate::http::Response>
        where
            U: AsRef<str>,
            B: Into<surf::Body>,
        {
            let req = if let Some(body) = body {
                let body: surf::Body = body.into();
                self.client.$methd(endpoint).body(body).build()
            } else {
                self.client.$methd(endpoint).build()
            };
            Ok(self.execute(headers, Request::from(req)).await?)
        }
    };
}
