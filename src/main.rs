use std::io;
use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::thread;
use std::fs;


// Function to parse the gophermap to allow for outside connections.
fn parse_map(gmap: String, connect_addr: String) -> String {
    // Replace all instances of localhost to the correct ip address
    // This could be made better, using 0 and 1 and otherwise to correctly assign stuff
    gmap.replace("localhost", &connect_addr)
}


fn client_handler(mut stream: TcpStream) -> io::Result<()> {
    // Showing the client connection
    println!("Client connected with info:\n{:?}", stream);
    
    // Reading from user initially
    let mut buf = [0;256];
    let _bytes_read = stream.read(&mut buf)?;
    let client_in = String::from_utf8_lossy(&buf).replace(&['\r', '\n', '\u{0}'][..], "");
    println!("From user: {}", &client_in);
        
    // If the client sends nothing, send gophermap.
    if client_in.eq("") {
        // Getting info from text file
        let file: String = fs::read_to_string("./resources/gophermap")
            .expect("Failed to  read file.");
        
        // Parse the gophermap
        let conn_ip: String = stream.local_addr()
            .expect("Failed to get IP")
            .to_string()
            .replace(':',"\t");
        let gophermap: String = parse_map(file, conn_ip);

        // Sending the client the gophermap
        stream.write_all(gophermap.as_bytes())?;
    }
    // If the client is looking for a text file, find and send that text file.
    else {
        // Getting info from text file
        if client_in.contains("../..") {
            let error = String::from("iDirectory surfing not allowed, sorry.");
            stream.write_all(error.as_bytes())?;
            log(format!("Client from IP {} attempted to directory surf.", client_ip))
                .expect("Failed to log to file.");
        }
        else {
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
    }
    
    // Exiting the function after packets are sent
    println!("Finished sending, closing connection.");
    Ok(())
}

fn main() {
    println!("Starting Server...");

    // Creating and binding port
    let receiver_listener = TcpListener::bind("192.168.1.154:70").expect("Failed to bind");

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
