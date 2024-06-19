use std::collections::HashMap;
use std::io::Write;

#[derive(Debug, PartialEq, Clone)]
pub struct HttpResponse<'a> {
    version: &'a str,
    status_code: &'a str,
    status_text: &'a str,
    headers: Option<HashMap<&'a str, &'a str>>,
    body: Option<String>,
}

impl<'a> Default for HttpResponse<'a> {
    fn default() -> Self {
        Self {
            version: "HTTP/1.1".into(),
            status_code: "200".into(),
            status_text: "OK".into(),
            headers: None,
            body: None,
        }
    }
}

impl<'a> HttpResponse<'a> {
    pub fn new(
        status_code: &'a str,
        headers: Option<HashMap<&'a str, &'a str>>,
        body: Option<String>,
    ) -> Self {
        let mut response: HttpResponse<'a> = HttpResponse::default();
        if status_code != "200" {
            response.status_code = status_code.into();
        }
        response.headers = match &headers {
            None => {
                let mut h = HashMap::new();
                h.insert("Content-Type", "text/html");
                Some(h)
            }
            Some(_) => headers,
        };
        response.status_text = match response.status_code {
            "200" => "OK".into(),
            "400" => "Bad Request".into(),
            "404" => "Not Found".into(),
            "500" => "Internal Error".into(),
            &_ => "Not Found".into(),
        };
        response.body = body;

        response
    }

    pub fn send_response(&self, write_stream: &mut impl Write) -> Result<(), ()> {
        let res = self.clone();
        let response_string: String = String::from(res);
        let _ = write!(write_stream, "{}", response_string);

        Ok(())
    }
    pub fn version(&self) -> &'a str {
        self.version
    }
    pub fn status_code(&self) -> &'a str {
        self.status_code
    }
    pub fn status_text(&self) -> &'a str {
        self.status_text
    }
    pub fn headers(&self) -> String {
        let map = self.headers.clone().unwrap();
        let mut header_string = "".into();
        for (k, v) in map {
            header_string = format!("{}{}:{}\r\n", header_string, k, v);
        }
        header_string
    }

    pub fn body(&self) -> &str {
        match &self.body {
            Some(b) => b.as_str(),
            None => "",
        }
    }
}

impl<'a> From<HttpResponse<'a>> for String {
    fn from(res: HttpResponse<'a>) -> Self {
        let response = &res;
        let body = res.body();
        format!(
            "{} {} {}\r\n{}Content-Length: {}\r\n\r\n{}",
            response.version(),
            response.status_code(),
            response.status_text(),
            response.headers(),
            body.len(),
            body,
        )
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tes_t_response_struct_creation_200() {
        let response_actual = HttpResponse::new(
            "200",
            None,
            Some("xxxx".into()));

        let response_expected = HttpResponse {
            version: "HTTP/1.1",
            status_code: "200",
            status_text: "OK",
            headers: {
                let mut h = HashMap::new();
                h.insert("Content-Type", "text/html");
                Some(h)
            },
            body: Some("xxxx".into()),
        };

        assert_eq!(response_actual, response_expected);
    }

    #[test]
    fn tes_t_response_struct_creation_404() {
        let response_actual = HttpResponse::new(
            "404",
            None,
            Some("xxxx".into()));

        let response_expected = HttpResponse {
            version: "HTTP/1.1",
            status_code: "404",
            status_text: "Not Found",
            headers: {
                let mut h = HashMap::new();
                h.insert("Content-Type", "text/html");
                Some(h)
            },
            body: Some("xxxx".into()),
        };

        assert_eq!(response_actual, response_expected);
    }

    #[test]
    fn tes_t_response_struct_creation() {
        let response_expected = HttpResponse {
            version: "HTTP/1.1",
            status_code: "404",
            status_text: "Not Found",
            headers: {
                let mut h = HashMap::new();
                h.insert("Content-Type", "text/html");
                Some(h)
            },
            body: Some("xxxx".into()),
        };

        let http_string: String = response_expected.into();

        let mut actual_string = "HTTP/1.1 404 Not Found\r\n".to_string();
        actual_string.push_str("Content-Type:text/html\r\n".into());
        actual_string.push_str("Content-Length: 4\r\n\r\n".into());
        actual_string.push_str("xxxx".into());
        assert_eq!(actual_string, http_string);
    }
}