use crate::util::ListInfo;
use reqwest::blocking::{self, Client};
use reqwest::Url;
use taskscheduler::{NaiveTask, Task, UpdateTask, TaskQueue};

pub fn list(host: String, port: u16, info: ListInfo) -> Result<String, String> {
    let mut url = convert_url(host, port)?;
    url.set_path("api/tasks");

    let response = blocking::get(url)
        .map_err(|e| e.to_string())?
        .text()
        .map_err(|e| e.to_string())?;

    let queue: TaskQueue = serde_json::from_str(&response).map_err(|e| e.to_string())?;
    let mut tasks: Vec<Task> = if info.completed {
        queue.iter_completed().map(|t| t.to_owned()).collect()
    } else {
        queue.iter().map(|t| t.to_owned()).collect()
    };

    // do some filtering based on cli options

    if let Some(before) = info.before {
        tasks.retain(|t| t.deadline < before);
    }
    if let Some(after) = info.after {
        tasks.retain(|t| t.deadline > after);
    }
    if let Some(shorter) = info.shorter {
        tasks.retain(|t| t.duration < shorter);
    }
    if let Some(longer) = info.longer {
        tasks.retain(|t| t.duration > longer);
    }
    if let Some(lower) = info.lower {
        tasks.retain(|t| t.priority < lower);
    }
    if let Some(higher) = info.higher {
        tasks.retain(|t| t.priority > higher);
    }

    // collect retained tasks into a string representation

    if tasks.is_empty() {
        Ok("No tasks match the specified bounds".to_string())
    } else {
        Ok(tasks.iter().map(|t| t.to_string()).collect())
    }
}

pub fn add(host: String, port: u16, task: NaiveTask) -> Result<String, String> {
    let mut url = convert_url(host, port)?;
    url.set_path("api/tasks");

    let client = Client::new();

    client
        .post(url)
        .body(serde_json::to_string(&task).map_err(|e| e.to_string())?)
        .send()
        .map_err(|e| e.to_string())?
        .text()
        .map_err(|e| e.to_string())
}

pub fn update(host: String, port: u16, task: UpdateTask) -> Result<String, String> {
    let mut url = convert_url(host, port)?;
    url.set_path("api/tasks");

    let client = Client::new();

    client
        .put(url)
        .body(serde_json::to_string(&task).map_err(|e| e.to_string())?)
        .send()
        .map_err(|e| e.to_string())?
        .text()
        .map_err(|e| e.to_string())
}

pub fn delete(host: String, port: u16, id: usize) -> Result<String, String> {
    let mut url = convert_url(host, port)?;
    url.set_path("api/tasks");

    let client = Client::new();

    client
        .delete(url)
        .body(serde_json::to_string(&id).map_err(|e| e.to_string())?)
        .send()
        .map_err(|e| e.to_string())?
        .text()
        .map_err(|e| e.to_string())
}

pub fn enable(host: String, port: u16, enable: bool) -> Result<String, String> {
    let mut url = convert_url(host, port)?;
    url.set_path(if enable {"api/tasks/enable"} else {"api/tasks/disable"});

    let client = Client::new();

    client
        .post(url)
        .send()
        .map_err(|e| e.to_string())?
        .text()
        .map_err(|e| e.to_string())
}

pub fn active(host: String, port: u16) -> Result<String, String> {
    let mut url = convert_url(host, port)?;
    url.set_path("api/tasks/active");

    let response = blocking::get(url)
        .map_err(|e| e.to_string())?
        .text()
        .map_err(|e| e.to_string())?;
    let task: Task = serde_json::from_str(&response)
        .map_err(|e| e.to_string())?;

    Ok(task.to_string())
}

fn convert_url(host: String, port: u16) -> Result<Url, String> {
    // constructing urls from scratch is not very simple, so setting the url to
    // 'http://example.com' allows us to work off of a base and swap in the
    // user configured values.
    let mut url = Url::parse("http://example.com").map_err(|e| e.to_string())?;
    url.set_host(Some(&host))
        .map_err(|_| "Unable to set URL host")?;
    url.set_port(Some(port))
        .map_err(|_| "Unable to set URL port")?;
    url.set_scheme("http")
        .map_err(|_| "Unable to set URL scheme")?;

    Ok(url)
}
