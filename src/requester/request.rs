use super::models::Response;

pub fn get(url: &str, headers: Vec<(&str, &str)>) -> Result<Response, String> {
    let client = reqwest::blocking::Client::builder()
        .danger_accept_invalid_certs(true)
        .build()
        .unwrap();

    let mut request = client.get(url);

    if headers.len() != 0 {
        for (key, value) in headers {
            request = request.header(key, value);
        }
    }

    let response = request.send();

    return handle_response(response);
}

pub fn post(url: &str, headers: Vec<(&str, &str)>, body: String) -> Result<Response, String> {
    let client = reqwest::blocking::Client::builder()
        .danger_accept_invalid_certs(true)
        .build()
        .unwrap();

    let mut request = client.post(url).body(body);

    if headers.len() != 0 {
        for (key, value) in headers {
            request = request.header(key, value);
        }
    }

    let response = request.send();

    return handle_response(response);
}

pub fn post_form(
    url: &str,
    headers: Vec<(&str, &str)>,
    form: Vec<(&str, &str)>,
) -> Result<Response, String> {
    let client = reqwest::blocking::Client::builder()
        .danger_accept_invalid_certs(true)
        .build()
        .unwrap();

    let mut request = client.post(url).form(&form);

    if headers.len() != 0 {
        for (key, value) in headers {
            request = request.header(key, value);
        }
    }

    let response = request.send();

    return handle_response(response);
}

pub fn put(url: &str, headers: Vec<(&str, &str)>, body: String) -> Result<Response, String> {
    let client = reqwest::blocking::Client::builder()
        .danger_accept_invalid_certs(true)
        .build()
        .unwrap();

    let mut request = client.put(url).body(body);

    if headers.len() != 0 {
        for (key, value) in headers {
            request = request.header(key, value);
        }
    }

    let response = request.send();

    return handle_response(response);
}

pub fn delete(url: &str, headers: Vec<(&str, &str)>) -> Result<Response, String> {
    let client = reqwest::blocking::Client::builder()
        .danger_accept_invalid_certs(true)
        .build()
        .unwrap();

    let mut request = client.delete(url);

    if headers.len() != 0 {
        for (key, value) in headers {
            request = request.header(key, value);
        }
    }

    let response = request.send();

    return handle_response(response);
}

fn handle_response(
    response: Result<reqwest::blocking::Response, reqwest::Error>,
) -> Result<Response, String> {
    return match response {
        Ok(value) => {
            let response_status = value.status().as_u16();
            let response_body = value.text().unwrap();

            let body_json: Result<serde_json::Value, serde_json::Error> =
                serde_json::from_str(response_body.as_str());

            match body_json {
                Ok(value) => Ok(Response {
                    status: response_status.to_string(),
                    body: value,
                }),
                Err(_) => {
                    let text: String = response_body.chars().filter(|c| !c.is_control()).collect();
                    let json_text = format!(r#"{{"text": "{}"}}"#, text);
                    Ok(Response {
                        status: response_status.to_string(),
                        body: serde_json::from_str(json_text.as_str()).unwrap(),
                    })
                }
            }
        }
        Err(err) => Err(err.to_string()),
    };
}
