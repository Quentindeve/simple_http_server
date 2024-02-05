pub mod cli;
pub mod file_utils;
pub mod http;

use std::{
    io::{Read, Write},
    net::TcpListener,
    path::PathBuf,
};

use cli::ServerCli;
use structopt::StructOpt;

use crate::http::{HttpContentType, HttpProtocolVersion, HttpRequest, HttpResponse, HttpStatus};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = ServerCli::from_args();
    println!("Cli: {:?}", cli);

    let bind_addr = format!("{}:{}", cli.bind, cli.port);
    let tcp_socket = TcpListener::bind(bind_addr)?;

    loop {
        match tcp_socket.accept() {
            Ok((mut socket, addr)) => {
                println!("Accepted client from {:?}", addr);

                // TODO: Not supposed to be fixed-size
                let mut buf = [0; 2048];
                socket.read(&mut buf)?;

                let request = HttpRequest::try_from(&buf)?;

                let response =
                    match file_utils::get_file(PathBuf::from(cli.root.clone()), request.route) {
                        Ok(content) => HttpResponse::new(
                            HttpProtocolVersion::Http11,
                            HttpStatus::Ok,
                            HttpContentType::Html,
                            content,
                        ),
                        Err(_) => HttpResponse::new(
                            HttpProtocolVersion::Http11,
                            HttpStatus::NotFound,
                            HttpContentType::Html,
                            String::from("<h1>Not found</h1>"),
                        ),
                    }
                    .build();

                socket.write_all(&response)?;
                socket.flush()?;
            }
            Err(err) => eprintln!("Couldn't connect to client: {:?}", err),
        }
    }
}
