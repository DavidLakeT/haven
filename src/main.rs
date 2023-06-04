#[macro_use] extern crate rocket;

#[get("/echo")]
fn echo_stream(ws: ws::WebSocket) -> ws::Stream!['static] {
    let ws = ws.config(ws::Config {
        max_send_queue: Some(5),
        ..Default::default()
    });

    ws::Stream! { ws =>
        for await message in ws {
            yield message?;
        }
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![echo_stream(ws)])
}