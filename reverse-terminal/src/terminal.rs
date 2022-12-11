use std::io::{Read, Write};
use futures::{FutureExt, SinkExt, stream::StreamExt};
use axum::extract::ws::{Message, WebSocket};
use portable_pty::{CommandBuilder, NativePtySystem, PtySize, PtySystem};
use serde_json::Value;
use anyhow::Result;

// pub async fn list_terminals() {}

pub async fn handle_start_pty_terminal_socket(socket: WebSocket) {
    println!("websocket connected");
    let pty_system = NativePtySystem::default();
    let mut pair = pty_system.openpty(PtySize {
        rows: 24,
        cols: 80,
        pixel_width: 0,
        pixel_height: 0,
    }).unwrap();

    let default_shell = std::env::var("SHELL").unwrap_or_else(|_| String::from("sh"));

    let cmd = CommandBuilder::new(default_shell);
    let mut child = pair.slave.spawn_command(cmd).unwrap();
    let mut reader = pair.master.try_clone_reader().unwrap();
    let (mut sender, mut receiver) = socket.split();

    println!("child wait");

    let mut writer = pair.master.try_clone_writer().unwrap();

    let pty_task = tokio::task::spawn_blocking(move || {
        loop {
            let mut buffer = vec![];
            buffer.resize(512, 0u8);
            let n = reader.read(&mut buffer).unwrap();
            if n == 0 {
                break;
            }

            tokio::runtime::Builder::new_current_thread().build().unwrap().block_on(
                sender.send(Message::Binary(Vec::from(&buffer[..n])))
            ).unwrap();
        }

        println!("read data done")
    }).then(|_| async move {
        println!("pty closed");
    });

    while let Some(Ok(message)) = receiver.next().await {
        println!("receive message");
        match message {
            Message::Text(stdin) => {
                println!("stdin: {}", stdin);
                write!(pair.master, "{}", stdin).unwrap();
            }
            Message::Binary(data) => match data.get(0) {
                Some(0) => {
                    if data.len().gt(&0) {
                        writer.write_all(&data[1..]).unwrap();
                    }
                }
                Some(1) => {
                    let object: Value = serde_json::from_slice(&data[1..]).unwrap();
                    pair.master.resize(PtySize {
                        rows: object["rows"].as_u64().unwrap_or(0) as u16,
                        cols: object["cols"].as_u64().unwrap_or(0) as u16,
                        pixel_width: 0,
                        pixel_height: 0,
                    }).unwrap();
                }
                Some(2) => {
                    // sender.send(Message::Binary(vec![1u8])).await.unwrap();
                }
                _ => {}
            }
            Message::Ping(_) => {
                println!("receive ping message");
            }
            Message::Pong(_) => {
                println!("receive pong message");
            }
            Message::Close(_) => {
                println!("receive close message");
            }
        }
        println!("process message done");
    }

    child.kill().unwrap();
    println!("websocket done");
    child.wait().unwrap();
    pty_task.await;
    println!("kill pty command done");
}
