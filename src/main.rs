use std::error::Error;
mod colors;
mod config;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let res = reqwest::get(config::url)
        .await?
        .text()
        .await?;
    println!("{green}[RESPONSE]:{reset} {:#?}", 
             res,
             green = colors::green,
             reset = colors::reset);
    Ok(())
}
