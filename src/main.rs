use color::term::BrightRed;

mod config;

fn main() {
    if let Err(e) = dispatch() {
        println!("{}: {}", BrightRed("Error:"), e.to_string());
    }
}

fn dispatch() -> Result<(), &'static str> {
    let _config = config::config().ok_or("Configuration file not found.")?;

    Ok(())
}
