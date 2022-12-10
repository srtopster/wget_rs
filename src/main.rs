use reqwest;
use std::fs::File;
use std::io::{Write,stdout};
use futures_util::StreamExt;
use std::{env,process,path};
use std::time::Instant;

const MB_IN_BYTES: f64 = 1048576.0;

#[tokio::main]
async fn main() {
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
        println!("Arquivo existente !\nCaso queira baixar use: wget <URL> NEW_FILENAME");
        process::exit(0)
    }

    let resp = reqwest::get(url).await.unwrap();
    let size = resp.content_length().unwrap() as f64;
    let mut stream = resp.bytes_stream();

    println!("[{:.2}Mb]{}",(size/MB_IN_BYTES),filename);
    let mut file = File::create(filename).expect("Erro ao criar arquivo");
    let mut done: f64 = 0.0;
    let old = Instant::now();

    while let Some(item) = stream.next().await {
        let chunk = item.expect("Erro ao baixar");
        file.write_all(&chunk).expect("Erro ao escrever arquivo");
        done += chunk.len() as f64;

        let perc = ((done/size)*100.0) as usize;
        let progress_bar = format!("{}>{}","=".repeat(perc/2)," ".repeat(50-perc/2));
        let bps = done/old.elapsed().as_secs_f64();
        let eta = ((size-done)/bps)/60.0;

        print!("\r[{}][{:.2}Mbps][{:.2}Min]{}% ",progress_bar,bps/MB_IN_BYTES,eta,perc);
        stdout().flush().unwrap();
    }
}