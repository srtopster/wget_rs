use reqwest;
use std::fs::File;
use std::io::Write;
use futures_util::StreamExt;
use std::io;
use std::time::Instant;
use std::env;
use std::process;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        println!("Uso: wget URL");
        process::exit(1)
    }
    if args[1].starts_with("https://") | args[1].starts_with("http://"){
        let url = &args[1];
        let file_name = &args[1].split("/").collect::<Vec<&str>>();
        let resp = reqwest::get(url).await?;
        let size = resp.content_length().unwrap() as f64;
        let mut stream = resp.bytes_stream();

        println!("[{:.2}Mb]{}",(size/1048576.0),file_name.last().unwrap());
        let mut file = File::create(file_name.last().unwrap()).unwrap();
        let mut done: f64 = 0.0;
        let old = Instant::now();
        while let Some(item) = stream.next().await {
            let chunk = item.or(Err(format!("Erro ao baixar")))?;
            file.write_all(&chunk).unwrap();
            done += chunk.len() as f64;
            let perc = ((done/size)*100.0) as i32;
            print!("\r[{}][{:.2}Mbps]{}%","=".repeat(perc as usize/2)+&" ".repeat(50-perc as usize/2),(done/(old.elapsed().as_secs() as f64))/1048576.0,perc);
            io::stdout().flush().unwrap()
        }
    }
    Ok(())
}