use std::{
    collections::HashMap,
    net::SocketAddr,
    str,
    sync::{Arc, Mutex},
};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};

#[derive(Debug)]
struct User {
    username: Option<String>,
    realname: Option<String>,
    nick: Option<String>,
}

impl Default for User {
    fn default() -> Self {
        User {
            username: None,
            realname: None,
            nick: None,
        }
    }
}

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("0.0.0.0:6667").await.unwrap();
    let users: Arc<Mutex<HashMap<SocketAddr, User>>> = Arc::new(Mutex::new(HashMap::new()));

    loop {
        let (stream, _) = listener.accept().await.unwrap();
        let ip: SocketAddr = stream.peer_addr().unwrap();
        println!("New connection established with {}", ip);

        let users = users.clone();
        tokio::spawn(async move {
            handle_client(stream, users, ip).await;
        });
    }
}

async fn handle_client(
    mut stream: TcpStream,
    users: Arc<Mutex<HashMap<SocketAddr, User>>>,
    ip: SocketAddr,
) {
    let mut buffer = [0; 10 * 1024];
    loop {
        match stream.read(&mut buffer).await {
            Ok(n) => {
                if n == 0 {
                    break;
                }
                let s = match str::from_utf8(&buffer[..n]) {
                    Ok(v) => v,
                    Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
                };
                let mut c = s.split_whitespace();
                let mut users_lock = users.lock().unwrap();
                let user = users_lock.entry(ip).or_default();
                match c.next().unwrap_or("") {
                    "NICK" => {
                        let nick = Some(c.next().unwrap().to_string());
                        user.nick = nick;
                    }
                    "USER" => {
                        let username = c.next().unwrap_or("");
                        c.next();
                        c.next();
                        let d = c.next().unwrap_or("");
                        let mut e = d.chars();
                        let f;
                        let realname = if e.next().unwrap_or(' ') == ':' {
                            f = format!(
                                "{} {} {}",
                                e.as_str(),
                                c.next().unwrap_or(""),
                                c.next().unwrap_or("")
                            );
                            f.as_str().trim()
                        } else {
                            d
                        };
                        user.username = Some(username.to_string());
                        user.realname = Some(realname.to_string());
                    }
                    "PING" => {}
                    _ => {}
                };
                println!("{:?}", user);
            }
            Err(e) => {
                users.lock().unwrap().remove(&ip);
                eprintln!("Error reading from stream: {}", e);
                return;
            }
        }
    }
}
