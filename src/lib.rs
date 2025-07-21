use reqwest::blocking::Client;

/// Gets current weather
#[unsafe(no_mangle)]
#[allow(unsafe_op_in_unsafe_fn)]
pub unsafe fn weather(
    city: *mut String,
    api_key: *mut String,
    lang: *mut String,
) -> *mut String {
    let url = format!(
        "https://api.openweathermap.org/data/2.5/weather?q={}&appid={}&units=metric&lang={}",
        &*city, &*api_key, &*lang
    );
    match Client::new().get(url).send() {
        Ok(response) => {
            Box::into_raw(Box::new(response.text().unwrap()))
        }
        Err(err) => {
            let mut response = String::from("{");
            response.push_str(&format!("\"error\": {}", err));
            response.push_str("}");
            Box::into_raw(Box::new(response))
        }
    }
}