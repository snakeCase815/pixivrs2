#![feature(async_closure)]
#![feature(box_into_pin)]
#![allow(dead_code)]
mod pixiv;
use pixiv::spider;

fn config_logger() {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.target(),
                record.level(),
                message
            ))
        })
        .level_for("hyper", log::LevelFilter::Off)
        .level_for("reqwest", log::LevelFilter::Off)
        .level_for("isahc", log::LevelFilter::Off)
        .level_for("tracing", log::LevelFilter::Off)
        .level(log::LevelFilter::Debug)
        .chain(std::io::stdout())
        .chain(fern::log_file("runtime/output.log").unwrap())
        .apply()
        .unwrap();
}

fn spider_run() {
    config_logger();
    let future = async move {
        let config = pixiv::config::GLOBAL_CONFIG.clone();
        let collection = mongodb::Client::with_uri_str(&config.mongo_url)
            .await
            .unwrap()
            .database("Pixiv")
            .collection("Illusts");
        
        
        let h1 = async_std::task::spawn(spider::artworks_spider::run(
            config.clone(),
            collection.clone(),
        ));
        let h2 = async_std::task::spawn(spider::tags_spider::run(config.clone(), collection.clone()));
        let h3 = async_std::task::spawn(spider::authors_spider::run(
            config.clone(),
            collection.clone(),
        ));
        h1.await;
        h2.await;
        h3.await;
        
    };

    // tokio::runtime::Runtime::new().unwrap().block_on(future);
    
    async_std::task::block_on(future);
}

fn main() {
    let args = std::env::args().skip(1).collect::<Vec<String>>();
    if args.len() <= 0 {
        println!("usage : spider");
        return;
    }
    let subcommand = args.get(0).unwrap();
    if subcommand == "spider" {
        spider_run();
    }
}
