
// implmenting threading to the program 
use std::thread;
// std::net - Networking primitives for TCP/UDP communication.
// TcpListener and TcpStream provide functionality for communication over TCP.
// TcpListener, A TCP socket server, listening for connections.
// TcpStream, A TCP stream between a local and a remote socket.
// Shutdown, Possible values which can be passed to the TcpStream::shutdown method.
use std::net::{TcpListener, TcpStream, Shutdown};

// used to help input and output funcitons
use std::io::{Read, Write};


//need to create the other fucntion . 

fn main() {

    // setup tcp listener on port 6666
    // unwrap is a wayto handle errors, if an error is passed the program will panic 
    let listener = TcpListener::bind("0.0.0.0:6666").unwrap();

    println!("Server up on port 6666");

    // accept connections and process them serially
    for stream in listener.incoming() {

        match stream{
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                thread::spawn(move|| {
                    handle_client(stream)
                }); 
            }
            Err(e) => {
                println!("Error: {}", e);
                /* connection failed */
            }
        }
        
        
    }
    // close the socket server
    // Outside of the loop
    drop(listener);
}

fn handle_client(mut stream: TcpStream) {
    // using a 50 byte buffer 
    // Buffer Initialization:
    let mut data = [0 as u8; 50];

    match stream.read(&mut data) {
        Ok(size) if size > 0 => {

            //println!("#Debug# Raw bytes: {:?}", &data[0..size]);

            //Convert recieved bytes to a string a print
            if let Ok(received) = String::from_utf8(data[0..size].to_vec()) {
                println!("*> Recieved: {}", received.trim());
            } else { 
                println!("!> Receved non-UTF-8 data"); 
            }
            
            // Send a response to the client
            println!(".> Sending Server Response");
            let msg = b"Server Hello";
            
            // Attempt to send all bytes over the tcp steam. if write_all returns an error enter IF statment. 
            if let Err(e) = stream.write_all(msg) {
                println!("!> Failed to send response: {}", e);
            }
        }

        Ok(_) => {
            // Client closed the connection
            println!("!> Client closed the connection");
        }
        Err(e) => {
            //Handle read errors
            println!("!> Error reading from the client: {}", e);
        }

    } 

    if let Err(e) = stream.shutdown(Shutdown::Both) {
        println!("!> Error shutting down conenction: {}", e);
    }
}
