use reqwest::blocking;
use reqwest::Url;
use taskscheduler::TaskQueue;

pub fn list(host: String, port: u16) -> Result<String, String> {
    let mut url = convert_url(host, port)?;
    url.set_path("api/tasks");

    let response = blocking::get(url)
        .map_err(|e| e.to_string())?
        .text()
        .map_err(|e| e.to_string())?;

    let queue: TaskQueue = serde_json::from_str(&response).map_err(|e| e.to_string())?;

    let priority = queue.show_priority();
    let tasks: String = queue.iter().map(|t| t.to_string()).collect();
    let completed: String = queue.iter_completed().map(|t| format!("{},", t.title)).collect();

    Ok(format!("Priority: {priority}\n\n{tasks}\nCompleted: {{ {completed} }}"))
}

fn convert_url(host: String, port: u16) -> Result<Url, String> {
    let mut url = Url::parse("http://example.com").map_err(|e| e.to_string())?;
    url.set_host(Some(&host)).map_err(|_| "Unable to set URL host")?;
    url.set_port(Some(port)).map_err(|_| "Unable to set URL port")?;
    url.set_scheme("http").map_err(|_| "Unable to set URL scheme")?;

    Ok(url)
}
