use crate::{
    static_items::{
        position::{clear_sombol_position, update_symbol_position_price},
        price::update_symbol_price,
        symbol,
    },
    utils::{self, format_url, trim_trailing_zeros},
};
use futures_util::{future::join_all, SinkExt, StreamExt};

use tokio::time::{self, timeout, Duration};
use tokio_tungstenite::{connect_async, tungstenite::Message};
use url::Url;

pub async fn start_websocket() {
    let symbols = symbol::get_symbols();
    let mut tasks: Vec<tokio::task::JoinHandle<()>> = Vec::new();

    for symbol in symbols {
        let symbol_clone = symbol.clone();

        let task = tokio::spawn(async move {
            connect_to_websocket(symbol_clone).await;
        });
        tasks.push(task);
    }

    // 使用 join_all 处理所有 WebSocket 任务
    join_all(tasks).await;
}

async fn connect_to_websocket(symbol: String) {
    let url = format_url(&symbol);
    // let mut flag = false;
    // let key = symbol.to_string();
    loop {
        match connect_async(Url::parse(&url).unwrap()).await {
            Ok((mut socket, _response)) => {
                loop {
                    let msg = timeout(Duration::from_secs(30), socket.next()).await;
                    match msg {
                        Ok(Some(inner_msg)) => match inner_msg {
                            Ok(Message::Text(text)) => match utils::parse_trade_json(&text) {
                                Ok(data) => {
                                    // if !flag {
                                    //     flag = true;
                                    //     println!("price=={:?}: {:?}", symbol, text);
                                    // }
                                    let book_price = (
                                        trim_trailing_zeros(&data.a),
                                        trim_trailing_zeros(&data.b),
                                    );
                                    update_symbol_price(&symbol, book_price.clone()).await;
                                    clear_sombol_position(&symbol).await;
                                    update_symbol_position_price(&symbol, book_price).await;
                                }
                                Err(_e) => {
                                    // println!("price: {}", text);
                                    // eprintln!("failed to parse JSON: {:?}", e);
                                }
                            },
                            Ok(Message::Ping(ping)) => {
                                // println!("Received Ping from {}: {:?}", url, ping);
                                socket
                                    .send(Message::Pong(ping))
                                    .await
                                    .expect("Failed to send Pong");
                            }
                            Ok(Message::Close(_frame)) => {
                                // println!("Connection closed from {}: {:?}", url, frame);
                                break;
                            }
                            _ => (),
                        },
                        Ok(None) => {
                            // println!("WebSocket closed, reconnecting...");
                            break;
                        }
                        Err(_) => {
                            // eprintln!(
                            //     "Timeout while waiting for WebSocket message, reconnecting..."
                            // );
                            break;
                        }
                    }
                }
            }
            Err(e) => {
                eprintln!(
                    "Connection failed to {}: {:?}. Retrying in 5 seconds...",
                    url, e
                );
            }
        }

        time::sleep(Duration::from_secs(5)).await;
        println!("Reconnecting to {}...", url);
    }
}
