
// implmenting threading to the program 
use std::thread;
// std::net - Networking primitives for TCP/UDP communication.
// TcpListener and TcpStream provide functionality for communication over TCP.
// TcpListener, A TCP socket server, listening for connections.
// TcpStream, A TCP stream between a local and a remote socket.
// Shutdown, Possible values which can be passed to the TcpStream::shutdown method.
use std::net::{TcpListener, TcpStream, Shutdown};

// used to help input and output funcitons
use std::io::{self, Read, Write};


//need to create the other fucntions

fn main() {

    // setup tcp listener on port 6666
    // unwrap is a way to handle errors, if an error is passed the program will panic 
    let listener = TcpListener::bind("0.0.0.0:6666").unwrap();

    println!("Server up on port 6666");

    // accept connections and process them serially
    for stream in listener.incoming() {

        match stream{

            Ok(stream) => {
                
                // Returns the socket address of the remote peer of this TCP connection.
                println!("New connection: {}", stream.peer_addr().unwrap());

                // Creating a thread.
                // inported std::thread
                // Threads are ment to communicate with channels. 
                // using 'move ||' gives ownership of values to a thread. 
                thread::spawn(move|| {
                    handle_client(stream)
                }); 

            }
            Err(e) => {

                println!("Error: {}", e);
                // connection failed 
            }
        }
        
        
    }
    // close the socket 
    // Outside of the loop
    drop(listener);
}

fn handle_client(mut stream: TcpStream) {

    println!(">> Connected to the client. Ready to send commands:");

    // Create buffer with 50 bytes
    let mut data = [0 as u8; 50];

    // loop to send commands
    loop {

        let mut input = String::new();

        //prompt for user input
        println!(".> Send a command to the client: ");

        // Call to the error first
        if let Err(e) = io::stdin().read_line(&mut input) {
            println!("!> Failed to read input: {}", e);
            continue;
        }

        let command = input.trim(); // trim newline

        // exit the loop if command is 'Exit'
        // eq_ignore_ascii_case ignores case ( exit , ExIt, EXIT ) 
        if command.eq_ignore_ascii_case("exit") {
            println!(".> exit sent. Closing connection.");
            break;
        }

        // Send the command to the client
        if let Err(e) = stream.write_all(command.as_bytes()) {
            println!("!> Failed to send command: {}", e);
            break; // exit loop if the connection is broken
        }
        println!(".> Sent command: '{}'", command);

        //Wait for response from client
        match stream.read(&mut data) {

            // DEBUG println!("#Debug# Raw bytes: {:?}", &data[0..size]);

            // if bytes sent are greater than 0 
            // from_utf8_lossy 
            Ok(size) if size > 0 => {

                // lossy converts between bytes and slice of bytes in u8
                // also trims newline or whitespace. 
                println!("*> Response from client: '{}'", String::from_utf8_lossy(&data[0..size]).trim()); 
                
            }
            Ok(_) => {
                println!("!> Client closed the connection.");
                break;
            }
            Err(e) => {
                println!("!> Error reading response: {}", e);
                break;
            }

        }


    } 

    if let Err(e) = stream.shutdown(Shutdown::Both) {
        println!("!> Error shutting down conenction: {}", e);
    }
}
