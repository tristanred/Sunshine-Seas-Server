use std::net::*;
use std::io::*;
use std::string::*;

mod commands;
use commands::*;

fn main() {
    start_client_polling();

    std::thread::sleep(std::time::Duration::from_millis(5000));

    let listener = TcpListener::bind("127.0.0.1:5555").unwrap();

    let x = HelloCommand::new();

    loop {
        match listener.accept() {
            Ok((sock, addr)) => {
                std::thread::spawn(move || {
                    println!("[S] Server thread start");
                    println!("[S] {:?} Connected", addr);

                    // Default of 30 seconds to test. Maybe use 10s ?
                    let default_timeout = std::time::Duration::from_secs(10);

                    sock.set_read_timeout(Some(default_timeout))
                        .expect("[S] Unable to set read timeout");

                    let mut reader = BufReader::new(sock);

                    //let mut my_str = String::new();
                    let mut buf = vec![];

                    loop {
                        let bytes_read_result = reader.read_until(b'|', &mut buf);

                        match bytes_read_result {
                            Ok(bytes_read) => {
                                if bytes_read == 0
                                {
                                    println!("[S] Input finished ?");

                                    break;
                                }

                                let msg_ident = &buf[0..4];
                                println!("TEst = {:?}", msg_ident);
                                let ident_string = String::from_utf8(msg_ident.to_vec()).expect("SomethingSomething");

                                if ident_string == "HELO"
                                {
                                    let msg = commands::HelloCommand::from_client_message(&buf[4..]).unwrap();

                                    println!("[S] Build message = {:?}", msg);
                                }

                                println!("[S] Got : {:?}", String::from_utf8(buf.clone()).unwrap());

                                buf.clear();
                            },
                            Err(e) => {
                                println!("[S] {:?}", e);

                                break;
                            }
                        }
                    }

                    println!("[S] Server thread terminated !");
                });
            },
            Err(_) => panic!()
        }
    }
}

fn start_client_polling() {
    std::thread::spawn(|| {
        loop {
            let connect_result = client_connect();

            if connect_result.is_err()  {
                println!("[C] Connection failed");
            }

            std::thread::sleep(std::time::Duration::from_millis(2000));
        }
    });
}

fn client_connect() -> std::io::Result<()> {
    //let mut conn = TcpStream::connect_timeout(&SocketAddr::from(([127, 0, 0, 1], 5555)), std::time::Duration::from_secs(10))?;
    let mut conn = TcpStream::connect("localhost:5555")?;

    conn.write_all(b"HELO_THIS-IS-CONTENT|")?;
    std::thread::sleep(std::time::Duration::from_millis(2000));

    conn.write_all(b"HELO_THIS-IS-ALSO-CONTENT|")?;
    std::thread::sleep(std::time::Duration::from_millis(2000));

    conn.write_all(b"HELO_LAST-CONTENT|")?;
    std::thread::sleep(std::time::Duration::from_millis(2000));

    return Ok(());
}
