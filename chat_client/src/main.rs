use std::io::{self, ErrorKind, Read, Write};
use std::net::TcpStream;
use std::sync::mpsc::{self, TryRecvError};
use std::thread;
use std::time::Duration;

const LOCAL: &str = "127.0.0.1:6000";
const MSG_SIZE: u8 = 32;

fn main() {
    let mut client = TcpStream::connect(LOCAL).expect("stream failed to connect");
    client
        .set_nonblocking(true)
        .expect("failed to initiaate non-blocking");

    let (tx, rx) = mpsc::channel::<String>();

    thread::spawn(move || loop {
        let mut buff = vec![0, MSG_SIZE];
        match client.read_exact(&mut buff) {
            Ok(_) => {
                let msg = buff.into_iter().take_while(|&x| x != 0).collect::<Vec<_>>();

                // let msg = String::from_utf8(msg).expect("invalid utf-8 message");

                println!("message received {:?}", msg);
            }
            Err(ref err) if err.kind() == ErrorKind::WouldBlock => (),
            Err(_) => {
                break;
            }
        }

        match rx.try_recv() {
            Ok(msg) => {
                let mut buff = msg.clone().into_bytes();
                buff.resize(MSG_SIZE as usize, 0);
                client
                    .write_all(&buff)
                    .expect("unable to write socket failed");
                println!("message send {:?}", msg);
            }
            Err(TryRecvError::Empty) => (),
            Err(TryRecvError::Disconnected) => break,
        }

        thread::sleep(Duration::from_millis(100));
    });

    println!("write a message:");

    loop {
        let mut buff = String::new();

        io::stdin()
            .read_line(&mut buff)
            .expect("reading from stdin failed");

        let msg = buff.trim().to_string();

        if msg == ":quit" || tx.send(msg).is_err() {
            break;
        }
    }
    println!("bye!");
}
