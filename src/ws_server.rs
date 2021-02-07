use mpsc::channel;
use std::net::TcpListener;
use std::thread::spawn;
use std::{
    io,
    net::TcpStream,
    sync::mpsc::{self, Sender},
};
use tungstenite::{accept, Message};

pub fn start() {
    let server = TcpListener::bind("0.0.0.0:9001").unwrap();
    server.set_nonblocking(true).unwrap();

    let (p_sender, p_receiver) = channel();
    let mut watchers: Vec<Sender<String>> = vec![];

    loop {
        match server.incoming().next().unwrap() {
            Ok(stream) => {
                // do something with the TcpStream
                watchers.push(new_connection(stream, p_sender.clone()));
            }
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {}
            Err(e) => panic!("encountered IO error: {}", e),
        }

        if let Ok(msg) = p_receiver.try_recv() {
            println!("[WEBSOCKET]: Message in center {:?}", msg);
            let mut invalid: Vec<usize> = watchers
                .iter()
                .enumerate()
                .filter_map(|(index, sender)| {
                    if sender.send(msg.clone()).is_ok() {
                        None
                    } else {
                        Some(index)
                    }
                })
                .collect();

            invalid.drain(..).rev().for_each(|x| {
                watchers.remove(x);
            });
        }
    }
}

fn new_connection(stream: TcpStream, player_sender: Sender<String>) -> Sender<String> {
    let (watcher_sender, watcher_receiver) = channel();

    spawn(move || {
        println!("[WEBSOCKET]: New Connection from {:?}", stream.peer_addr());
        stream.set_nonblocking(true).unwrap();
        let mut websocket = accept(stream).unwrap();
        loop {
            if let Ok(msg) = watcher_receiver.try_recv() {
                websocket.write_message(Message::Text(msg)).unwrap();
            }
            match websocket.read_message() {
                Ok(msg) => {
                    if let Message::Text(msg) = &msg {
                        player_sender.send(msg.clone()).unwrap();
                    }

                    // We do not want to send back ping/pong messages.
                    if msg.is_binary() || msg.is_text() {
                        websocket.write_message(msg).unwrap();
                    }
                }
                Err(tungstenite::Error::ConnectionClosed) => break,
                Err(tungstenite::Error::Io(e)) if e.kind() == io::ErrorKind::WouldBlock => {}
                Err(e) => panic!("[WEBSOCKET]: ERROR {:?}", e),
            }
        }
    });

    watcher_sender
}
