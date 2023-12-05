use error_chain::error_chain;
// use std::io::Read;
use tokio;

error_chain! {
    foreign_links {
        Reqwest(reqwest::Error);
        Io(std::io::Error);
    }
}

#[tokio::main]
async fn main() -> Result<()>{
    let resp = reqwest::get("http://www.httpbin.org/get").await?;
    println!("Status: {}", resp.status());
    println!("Headers:\n{:#?}", resp.headers());
    println!("Body:\n{}", resp.text().await?);
    Ok(())
}

// fn main2() -> Result<()> {
//     let mut resp = reqwest::blocking::get("http://www.httpbin.org/get")?;
//     let mut body = String::new();
//     println!("Status: {}", resp.status());
//     println!("Headers:\n{:#?}", resp.headers());
//     println!("Body:\n{}", body);
//        Ok(()) 
// }
