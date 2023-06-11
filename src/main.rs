use dotenv::dotenv;
use haven::{discord::bot::build_discord, utils::server::build_rocket};

fn main() {
    env_logger::init();
    dotenv().ok();

    let application = std::env::args().nth(1);
    match application {
        Some(app) => match app.as_str() {
            "BOT" => {
                rocket::tokio::runtime::Runtime::new()
                    .unwrap()
                    .block_on(build_discord());
            }
            "API" => {
                rocket::tokio::runtime::Runtime::new()
                    .unwrap()
                    .block_on(build_rocket().launch())
                    .unwrap();
            }
            _ => {
                println!("Parámetro desconocido: {}", app);
                return;
            }
        },
        None => {
            println!("Usage: `cargo run <BOT/API>`");
        }
    }
}
