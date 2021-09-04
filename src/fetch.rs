use crate::error::BertrandError;
use std::collections::HashMap;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};

/// Fetch a file given a base URL and file.
pub async fn fetch_url(base_url: String, file: String) -> Result<String, BertrandError> {
    let url = format!("{}/{}", base_url, file);
    log::info!("Fetching {}.", url);

    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors);

    let request = Request::new_with_str_and_init(&url, &opts)?;

    let window = yew::utils::window();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
    let resp: Response = resp_value.dyn_into().unwrap();

    let text = JsFuture::from(resp.text()?).await?;
    Ok(text.as_string().unwrap())
}

/// Get a list of files with the same extension from a base URL.
pub async fn get_data(
    base_url: String,
    files: Vec<String>,
    ext: Option<&str>,
) -> Result<HashMap<String, String>, BertrandError> {
    let mut res = HashMap::new();

    for f in files {
        let mut file = f.clone();
        if let Some(ext) = &ext {
            file = format!("{}.{}", file, ext)
        };
        let content = fetch_url(base_url.clone(), file).await?;
        res.insert(f, content);
    }

    Ok(res)
}
