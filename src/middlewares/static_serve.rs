use std::fs;
use std::path;

use crate::router::RouteHandler;

pub fn serve_static(path: &'static str) -> RouteHandler {
    let path = path::Path::new(path);
    Box::new(move |req, res| {
        let target = path;
        let filepath = target.join(&req.path.strip_prefix("/").unwrap_or_else(|| "/"));

        if filepath.starts_with(&target) {
            if filepath.exists() {
                let string_path = if let Some(path) = filepath.to_str() {
                    path
                } else {
                    res.set_status(500).unwrap();
                    res.send("internal server error").unwrap();
                    return Ok(true);
                };

                res.send_file(string_path).unwrap();
            }
        }
        Ok(true)
    })
}
