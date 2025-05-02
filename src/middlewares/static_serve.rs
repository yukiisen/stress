use std::path::{self, PathBuf};

use crate::router::RouteHandler;

pub fn serve_static(path: &'static str) -> RouteHandler {
    let path = path::Path::new(path);
    Box::new(move |req, res| {
        let target = path;
        let mut filepath = target.join(&req.path.strip_prefix("/").unwrap_or_else(|| "/"));

        if filepath == PathBuf::from("./") {
            filepath = PathBuf::from("./index.html");
        }

        dbg!(&filepath);

        if filepath.starts_with(&target) && filepath.exists() {
            let string_path = if let Some(path) = filepath.to_str() {
                path
            } else {
                res.set_status(500)?;
                res.send("internal server error")?;
                return Ok(true);
            };

            res.set_status(200)?;
            res.send_file(string_path)?;
        } else {
            res.set_status(404)?;
            res.send("File not found")?;
        }
        Ok(true)
    })
}
