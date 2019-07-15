use std::net::*;
use std::io::*;
use std::string::*;

fn main() {
    start_client_polling();

    std::thread::sleep(std::time::Duration::from_millis(10000));

    let listener = TcpListener::bind("127.0.0.1:5555").unwrap();

    loop {
        match listener.accept() {
            Ok((sock, addr)) => {
                std::thread::spawn(move || {
                    println!("Server thread start");

                    println!("{:?} Connected", addr);

                    let mut reader = BufReader::new(sock);

                    //let mut my_str = String::new();
                    let mut buf = vec![];

                    loop {
                        let bytes_read = reader.read_until(b'|', &mut buf).expect("Cannot read line.");
                        //let bytes_read = reader.read_line(&mut my_str).expect("Cannot read line.");

                        if bytes_read > 0 {
                            println!("Got : {:?}", String::from_utf8(buf.clone()).unwrap());

                            buf.clear();
                        }
                    }

                    println!("Server thread terminated !");
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
                println!("Connection failed");
            }

            std::thread::sleep(std::time::Duration::from_millis(2000));
        }
    });
}

fn client_connect() -> std::io::Result<()> {
    //let mut conn = TcpStream::connect_timeout(&SocketAddr::from(([127, 0, 0, 1], 5555)), std::time::Duration::from_secs(10))?;
    let mut conn = TcpStream::connect("localhost:5555")?;

    conn.write_all(b"Hello hello|")?;
    std::thread::sleep(std::time::Duration::from_millis(2000));

    conn.write_all(b"Hello 2|")?;
    std::thread::sleep(std::time::Duration::from_millis(2000));

    conn.write_all(b"Hello 3|")?;
    std::thread::sleep(std::time::Duration::from_millis(2000));

    return Ok(());
}
