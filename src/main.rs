use std::io;
use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::thread;
use std::fs;
use std::file;

fn client_handler(mut stream: TcpStream) -> io::Result<()> {
    println!("Client connected with info:\n{:?}", stream);
    
    // Reading from user initially
    let mut buf = [0;256];
    let bytes_read = stream.read(&mut buf)?;
    let client_in = String::from_utf8_lossy(&buf).replace(&['\r', '\n', '\u{0}'][..], "");
    println!("From user: {}", bytes_read);
    println!("To string: {}", String::from_utf8_lossy(&buf));
        
    // If the client sends nothing, send gophermap.
    if client_in.eq("") {
        // Getting info from text file
        let gophermap: String = fs::read_to_string("./resources/gophermap")
            .expect("Failed to  read file.");

        // Sending the client the gophermap
        stream.write_all(gophermap.as_bytes())?;
    }

    else if client_in.to_lowercase().contains(".png") {
        let filename = format!("./resources/{}",client_in);
        let image = fs::read(filename);
        match image {
            Ok(image)=> {
                // Sending the client the gophermap
                stream.write_all(&image)?;

            },
            Err(e)=> {
                println!("File not found\n{:?}", e);
            }
        }
    }

    // If the client is looking for a text file, find and send that text file.
    else {
        // Getting info from text file
        let filename = format!("./resources/{}", client_in);
        let gophermap = fs::read_to_string(&filename);

        // Send if file exists, tell the user it does not exist otherwise.
        match gophermap {
            Ok(gophermap)=> {
                // Sending the client the gophermap
                stream.write_all(gophermap.as_bytes())?;

            },
            Err(e)=> {
                println!("File not found\n{:?}", e);
                let err_message = format!("The file {} does not exist.", filename);
                stream.write_all(err_message.as_bytes())?;
            }
        }
    }

    println!("Finished sending, closing connection.");
    Ok(())
}

fn main() {
    println!("Starting Server...");

    // Creating and binding port
    let receiver_listener = TcpListener::bind("127.0.0.1:70").expect("Failed to bind");

    // Creating thread vector
    let mut thread_vec: Vec<thread::JoinHandle<()>> = Vec::new();

    // Start listening to incoming connections and handle them.
    for stream in receiver_listener.incoming() {
        let stream = stream.expect("Failed to accept client");
        let handle = thread::spawn(move || {client_handler(stream).unwrap()});
        thread_vec.push(handle);
    }

    for handle in thread_vec {
        // Killing threads after use
        handle.join().unwrap();
    }
}

