use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum Method {
    Get,
    Post,
    Uninitialized,
}

impl From<&str> for Method {
    fn from(s: &str) -> Self {
        match s {
            "GET" => Method::Get,
            "POST" => Method::Post,
            &_ => Method::Uninitialized,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Version {
    V1_1,
    V2_0,
    Uninitialized,
}

impl From<&str> for Version {
    fn from(s: &str) -> Self {
        match s {
            "HTTP/1.1" => Version::V1_1,
            "HTTP/2.0" => Version::V2_0,
            &_ => Version::Uninitialized,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Resource {
    Path(String)
}

#[derive(Debug)]
pub struct HttpRequest {
    pub method: Method,
    pub version: Version,
    pub resource: Resource,
    pub headers: HashMap<String, String>,
    pub msg_body: String,
}

impl HttpRequest {
    pub fn process_req_line(s: &str) -> (Method, Version, Resource) {
        let mut words = s.split_whitespace();
        let method = words.next().unwrap();
        let resource = words.next().unwrap();
        let version = words.next().unwrap();

        (
            method.into(),
            version.into(),
            Resource::Path(resource.to_string())
        )
    }

    pub fn process_header_line(s: &str) -> (String, String) {
        // let mut header_items = s.split(":");
        // let mut key = String::new();
        // let mut value = String::new();
        // if let Some(k) = header_items.next() {
        //     key = k.to_string();
        // }
        // if let Some(v) = header_items.next() {
        //     value = v.to_string();
        // }

        if let Some((key, value)) = s.split_once(":") {
            (key.trim().to_string(), value.trim().to_string())
        } else {
            ("".to_string(), "".into())
        }
    }
}

impl From<String> for HttpRequest {
    fn from(req: String) -> Self {
        let mut parsed_method = Method::Uninitialized;
        let mut parsed_version = Version::Uninitialized;
        let mut parsed_resource = Resource::Path("".to_string());
        let mut parsed_headers = HashMap::new();
        let mut parsed_msg_body = "";

        for line in req.lines() {
            if line.contains("HTTP") {
                let (method, version, resource) = Self::process_req_line(line);
                parsed_method = method;
                parsed_version = version;
                parsed_resource = resource;
            } else if line.contains(":") {
                let (key, value) = Self::process_header_line(line);
                parsed_headers.insert(key, value);
            } else if line.is_empty() {} else {
                parsed_msg_body = line;
            }
        }

        HttpRequest {
            method: parsed_method,
            version: parsed_version,
            resource: parsed_resource,
            headers: parsed_headers,
            msg_body: parsed_msg_body.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_method_into() {
        let m: Method = "GET".into();
        assert_eq!(m, Method::Get);
        let m: Method = "POST".into();
        assert_eq!(m, Method::Post);
        let m: Method = "xxx".into();
        assert_eq!(m, Method::Uninitialized);
    }

    #[test]
    fn test_version_into() {
        let m: Version = "HTTP/1.1".into();
        assert_eq!(m, Version::V1_1);
        let m: Version = "HTTP/2.0".into();
        assert_eq!(m, Version::V2_0);
        let m: Version = "xxx".into();
        assert_eq!(m, Version::Uninitialized);
    }

    #[test]
    fn test_read_http_request() {
        let mut s = String::from("GET /greeting HTTP/1.1\r\n");
        s.push_str("Host: localhost:3000\r\n");
        s.push_str("User-Agent: curl/7.71.1\r\n");
        s.push_str("Accept: */*\r\n\r\n");
        let req: HttpRequest = s.into();

        let mut headers_expected: HashMap<String, String> = HashMap::new();
        headers_expected.insert("Host".into(), "localhost:3000".into());
        headers_expected.insert("User-Agent".to_string(), "curl/7.71.1".to_string());
        headers_expected.insert("Accept".to_string(), "*/*".into());

        assert_eq!(Method::Get, req.method);
        assert_eq!(Version::V1_1, req.version);
        assert_eq!(Resource::Path("/greeting".into()), req.resource);
        assert_eq!(headers_expected, req.headers);
    }
}