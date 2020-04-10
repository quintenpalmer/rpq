pub fn not_found() -> String {
    "<html><body><h1>not found</h1></body></html>".into()
}

pub fn internal_server_error() -> String {
    "<html><body><h1>internal server error</h1></body></html>".into()
}

pub fn bad_request<T: Into<String>>(message: T) -> String {
    format!(
        "<html><body><h1>bad request</h1><p>{}</p></body></html>",
        message.into()
    )
}
