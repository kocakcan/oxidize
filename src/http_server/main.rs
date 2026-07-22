/// Building a Single-Threaded Web Server
///
/// The two main protocols involved in web servers are Hypertext Transfer Protocol (HTTP) and
/// Transmission Control Protocol (TCP). Both protocols are request-response protocols, meaning a
/// client initiates a request and a server listens to the requests and provides a response to the
/// client. The contents of those requests and responses are defined by the protocols.
///
/// TCP is the lower-level protocol that describes the details of how information gets from one
/// server to another but doesn't specify what that information is. HTTP builds on top of TCP by
/// defining the contents of the requests and responses. It's technically possible to use HTTP with
/// other protocols, but in the vast majority of cases, HTTP sends its data over TCP.
///
/// Listening to the TCP Connection
///
/// Our web server needs to listen to a TCP connection. The standard library offers a std::net
/// module that lets us do this.
///
/// In the address, the section before the colon is an IP address representing your computer (this
/// is the same on every computer), and 7878 is the port. This port isn't normally accepted on this
/// port, so our server is unlikely to conflict with any other web server you might have running on
/// your machine.
///
/// The bind function in this scenario works like the new function in that it will return a new
/// TcpListener instance. The function is called bind because, in networking, connecting to a port
/// to listen to is known as "binding to a port."
///
/// The bind function returns a Result<T, E>, which indicates that it's possible for binding to
/// fail, for example, if we ran two instances of our program and so had wo programs listening to
/// the same port. Because we're writing a basic server just for learning purposes, we won't worry
/// about handling these kinds of errors; instead, we use unwrap to stop the program if errors
/// happen.
///
/// The incoming method on TcpListener returns an iterator that gives us a sequence of streams
/// (more specifically, streams of type TcpStream). A single stream represents an open connection
/// between the client and the server. Connection is the name for the full request and response
/// process in which a client connects to the server, the server generates a response, and the
/// server closes the connection. As such, we will read from the TcpStream to see what the client
/// sent and then write our response to the stream to send data back to the client. Overall, this
/// for loop will process each connection in turn and produce a series of streams for us to handle.
///
/// Reading the Request
///
/// To separate the concerns of first getting a connection and then taking some action with the
/// connection, we'll start a new function for processing connections. In this new
/// handle_connection function, we'll read data from the TCP stream and print it so that we can see
/// the data being sent from the browser.
use std::{
    io::{BufReader, prelude::*},
    net::{TcpListener, TcpStream},
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    println!("Request: {http_request:#?}");
}
