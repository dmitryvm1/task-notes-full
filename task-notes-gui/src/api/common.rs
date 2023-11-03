use eframe::web_sys::{Request, RequestInit, RequestMode, Response};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;

pub async fn get_json(url: String) -> Result<JsValue, JsValue> {
    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors);
    let request = Request::new_with_str_and_init(&url, &opts)?;
    let window = eframe::web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
    // `resp_value` is a `Response` object.
    assert!(resp_value.is_instance_of::<Response>());
    let resp: Response = resp_value.dyn_into().unwrap();
    // Convert this other `Promise` into a rust `Future`.
    let json = JsFuture::from(resp.json()?).await?;
    Ok(json)
}

pub async fn delete_json(url: String) -> Result<JsValue, JsValue> {
    let mut opts = RequestInit::new();
    opts.method("DELETE");
    opts.mode(RequestMode::Cors);
    let request = Request::new_with_str_and_init(&url, &opts)?;
    let window = eframe::web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
    // `resp_value` is a `Response` object.
    assert!(resp_value.is_instance_of::<Response>());
    let resp: Response = resp_value.dyn_into().unwrap();
    // Convert this other `Promise` into a rust `Future`.
    let json = JsFuture::from(resp.json()?).await?;
    Ok(json)
}

pub async fn post_json(url: String, data: &JsValue) -> Result<JsValue, JsValue> {
    let mut opts = RequestInit::new();
    opts.method("POST");
    opts.mode(RequestMode::Cors);
    opts.body(Some(data));
    opts.credentials(eframe::web_sys::RequestCredentials::Include);
    let request = Request::new_with_str_and_init(&url, &opts)?;
    request.headers().set("content-type", "application/json");
    let window = eframe::web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
    // `resp_value` is a `Response` object.
    assert!(resp_value.is_instance_of::<Response>());
    let resp: Response = resp_value.dyn_into().unwrap();
    // Convert this other `Promise` into a rust `Future`.
    let json = JsFuture::from(resp.json()?).await?;
    Ok(json)
}

pub async fn patch_json(url: String, data: &JsValue) -> Result<JsValue, JsValue> {
    let mut opts = RequestInit::new();
    opts.method("PATCH");
    opts.mode(RequestMode::Cors);
    opts.body(Some(data));
    opts.credentials(eframe::web_sys::RequestCredentials::Include);
    let request = Request::new_with_str_and_init(&url, &opts)?;
    request.headers().set("content-type", "application/json");
    let window = eframe::web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
    // `resp_value` is a `Response` object.
    assert!(resp_value.is_instance_of::<Response>());
    let resp: Response = resp_value.dyn_into().unwrap();
    // Convert this other `Promise` into a rust `Future`.
    let json = JsFuture::from(resp.json()?).await?;
    Ok(json)
}