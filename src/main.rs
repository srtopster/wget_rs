use reqwest;
use std::fs::File;
use std::io::{Write,stdout};
use futures_util::StreamExt;
use std::{env,process,path};
use std::time::Instant;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        println!("Uso: wget URL (filename)");
        process::exit(1)
    }
    if !args[1].starts_with("https://") & !args[1].starts_with("http://"){
        println!("Url invalida.");
        process::exit(1)
    }
    let url = &args[1];
    let filename: String;
    if args.len() >= 3{
        filename = format!("{}.{}",&args[2],args[1].split(".").last().unwrap());
    }
    else {
        filename = args[1].split("/").last().unwrap().to_owned();
    }
    if path::Path::new(&filename).exists() {
        println!("Arquivo existente");
        println!("Caso queira baixar use: wget <URL> NEW_FILENAME");
        process::exit(0)
    }
    let resp = reqwest::get(url).await?;
    let size = resp.content_length().unwrap() as f64;
    let mut stream = resp.bytes_stream();

    println!("[{:.2}Mb]{}",(size/1048576.0),filename);
    let mut file = File::create(filename).unwrap();
    let mut done: f64 = 0.0;
    let old = Instant::now();
    while let Some(item) = stream.next().await {
        let chunk = item.or(Err(format!("Erro ao baixar")))?;
        file.write_all(&chunk).unwrap();
        done += chunk.len() as f64;
        let perc = ((done/size)*100.0) as i32;
        print!("\r[{}][{:.2}Mbps]{}%","=".repeat(perc as usize/2)+&" ".repeat(50-perc as usize/2),(done/(old.elapsed().as_secs() as f64))/1048576.0,perc);
        stdout().flush().unwrap();
    }
    Ok(())
}