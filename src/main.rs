use std::{io, mem, process};

use scraper::Html;
use tokio::runtime::Runtime;
use tokio::sync::mpsc;

use async_demo::{parse_blogs, parse_weeks, print_week};

async fn run() -> Result<(), reqwest::Error> {
    let doc_body = reqwest::get("https://this-week-in-rust.org/blog/archives/index.html")
        .await?
        .text()
        .await?;

    let doc = Html::parse_document(&doc_body);
    let weeks = parse_weeks(&doc);

    // важно - използваме канали от `tokio::sync::mpsc`, а не от `std::sync::mpsc`.
    // Каналите от tokio блокират само текущия task, докато каналите от std
    // блокират цялата нишка
    let (sender, mut receiver) = mpsc::unbounded_channel();

    let limit = 3;
    for week in weeks.into_iter().take(limit) {
        let sender = sender.clone();

        tokio::spawn(async move {
            let week_doc_body = reqwest::get(&week.href).await?.text().await?;
            let week_doc = Html::parse_document(&week_doc_body);

            let _ = sender.send((week, parse_blogs(&week_doc)));

            Result::<_, reqwest::Error>::Ok(())
        });
    }

    // трябва да деструктираме изпращащата част на канала, иначе цикълът по-долу
    // никога няма да свърши. останалите копия на sender ще се деструктират като
    // приключат задачите създадени от `tokio::spawn`
    mem::drop(sender);

    while let Some((week, blogs)) = receiver.recv().await {
        print_week(io::stdout().lock(), &week, &blogs).unwrap();
    }

    Ok(())
}

fn main() {
    let runtime = Runtime::new().unwrap();

    match runtime.block_on(run()) {
        Ok(()) => (),
        Err(e) => {
            eprintln!("An error occurred: {}", e);
            process::exit(1);
        }
    }
}
