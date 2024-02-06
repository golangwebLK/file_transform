use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;
use file_transform::{handle_client, send_file};
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[clap(author, version, about)]
struct Args {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command{
    Client {
        #[clap(short,long)]
        send: Option<String>,
        #[clap(long)]
        host: Option<String>,
        #[clap(short,long)]
        port: Option<String>,
    },
    Server{
        #[clap(long)]
        host: Option<String>,
        #[clap(short,long)]
        port: Option<String>,
    },
}

fn main() {
    let args = Args::parse();
    match &args.command {
        Command::Client {
            send,
            host,
            port
        } =>{
            let server = format!("{}:{}",host.clone().unwrap(),port.clone().unwrap());
            let mut stream = TcpStream::connect(server).unwrap();
            if let Some(filename) = send{
                match send_file(&mut stream,filename) {
                    Ok(_rs) =>{
                        println!("发送成功！")
                    }
                    Err(e) => panic!("文件发送失败：{}",e)
                }
            }
        },
        Command::Server {
            host,
            port
        } =>{
            let server = format!("{}:{}",host.clone().unwrap(),port.clone().unwrap());
            let listener = TcpListener::bind(server.as_str()).expect("ip绑定失败");
            //incoming返回此侦听器上接收的连接的迭代器。
            // 返回的迭代器永远不会返回 None，也不会生成对等方的 SocketAddr 结构。遍历它等效于在循环中调用 TcpListener：：accept。
            for stream in listener.incoming(){
                match stream {
                    Ok(stream) =>{
                        let stream = Arc::new(Mutex::new(stream));
                        let stream_clone = Arc::clone(&stream);
                        thread::spawn(||{
                            handle_client(stream_clone);
                        });
                    }
                    Err(e) => {
                        eprintln!("tcp 连接失败: {}", e);
                    }
                }
            }
        },
    }
}


