mod gzip;

use std::fs::File;
use std::io;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::path::Path;
use std::sync::{Arc, Mutex};

struct FileMessage{
    filename: String,
    file_size: u64,
    file_data:Vec<u8>,
}

fn encode_file_message(msg: FileMessage) -> Vec<u8> {
    let mut encoded_message = Vec::<u8>::new();
    encoded_message.push(msg.filename.len() as u8);
    encoded_message.extend_from_slice(&msg.filename.as_bytes());
    encoded_message.extend_from_slice(&msg.file_size.to_le_bytes());
    encoded_message.extend_from_slice(&msg.file_data);
    encoded_message
}

fn decode_file_message(data:&[u8]) -> FileMessage{
    let filename_length = data[0] as usize;
    let filename = String::from_utf8_lossy(&data[1..1+filename_length]).into_owned();
    let file_size = u64::from_le_bytes(data[1 + filename_length..9 + filename_length].try_into().unwrap());
    let file_data = data[9+filename_length..].to_vec();
    FileMessage{
        filename,
        file_size,
        file_data,
    }
}

pub fn handle_client(stream: Arc<Mutex<TcpStream>>) {
    let mut buffer = Vec::new();
    loop {
        let mut temp_buffer = [0; 1024];
        match stream.lock().unwrap().read(&mut temp_buffer) {
            Ok(bytes_read) if bytes_read > 0 => {
                buffer.extend_from_slice(&temp_buffer[..bytes_read]);
                if bytes_read < 1024 {
                    break;
                }
            }
            Ok(_) | Err(_) => break,
        }
    }
    let received_message = decode_file_message(&buffer);
    let path = Path::new(&received_message.filename);
    if let Some(file_name) = path.file_name() {
        if let Some(file_name_str) = file_name.to_str() {
            save_file(file_name_str, &received_message.file_data).expect("文件保存失败");
        } else {
            println!("无效的文件名");
        }
    } else {
        println!("路径无效");
    }
}

pub fn send_file(stream: &mut TcpStream, filename: &str) -> io::Result<()> {
    let mut file_data = Vec::new();
    file_data.extend_from_slice(gzip::compress_to_buffer(filename).unwrap().as_slice());

    let file_message = FileMessage {
        filename: filename.to_owned(),
        file_size: file_data.len() as u64,
        file_data,
    };

    let encoded_message = encode_file_message(file_message);

    stream.write_all(&encoded_message)?;

    Ok(())
}

fn save_file(filename: &str, data: &[u8]) -> io::Result<()> {
    let file = format!("successfully-{}",filename);
    let mut file = File::create(file)?;
    let binding = gzip::decompress_from_buffer(data).unwrap();
    let original_data = binding.as_slice();

    file.write_all(original_data)?;

    Ok(())
}
