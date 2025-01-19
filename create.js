const fs = require("fs");
let types = require("./data/mime_types.json");

let rust_output = `
use std::collections::HashMap;
pub struct MimeTypes;
impl MimeTypes {
    pub fn get_map () -> HashMap<&'static str, Vec<&'static str>> {
        HashMap::from([\n`;

for (const key in types) {
    for (const e of types[key]) {
        let input = `\t\t\t("${e}", "${key}"),\n`;
        rust_output = rust_output.concat(input);
    }
}

rust_output = rust_output.concat(`
        ])
    }
}
`);

fs.writeFileSync("./src/mime_types.rs", rust_output);