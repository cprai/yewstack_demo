//! ```cargo
//! [dependencies]
//! maud = "*"
//! base64 = "*"
//! ```

extern crate maud;
extern crate base64;
use maud::html;
use base64::encode;
use std::fs::{read, write};

// Beware of data url size limits on certain browsers!!!
fn into_url(data: String, mime_type: &str) -> String {
    format!("data:{};base64,{}", mime_type, data)
}

fn file_as_url(filename: &str, mime_type: &str) -> String {
    read(filename)
        .map(|bytes| encode(bytes))
        .map(|b64_string| into_url(b64_string, mime_type))
        .unwrap_or_else(|_| panic!("Failed to read and encode: {}", filename))
}

fn str_as_url(data: String, mime_type: &str) -> String {
    Some(data)
        .map(|bytes| encode(bytes))
        .map(|b64_string| into_url(b64_string, mime_type))
        .unwrap_or_else(|| panic!("Failed to encode script"))
}

fn main() {
    let js = file_as_url("./pkg/yewstack_demo.js", "application/javascript");
    let wasm = file_as_url("./pkg/yewstack_demo_bg.wasm", "application/wasm");
    let loader_script = format!("import init from '{}';(async()=>init('{}'))()", js, wasm);
    let loader = str_as_url(loader_script, "application/javascript");

    let markup = html!{
        html {
            head {
                meta content="text/html;charset=utf-8" http-equiv="Content-Type" { }
                link rel="icon" href="data:," { }
                script type="module" src=(loader) { }
            }
        }
    };

    write("./index.html", markup.into_string());
}
