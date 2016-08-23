extern crate rand;
//extern crate mio;

use std::io;
use std::thread;
use std::cmp::Ordering;
use rand::Rng;

//use mio::udp::UdpSocket;

use std::net::SocketAddr;
//use std::net::UdpSocket;
use std::net;
use std::net::UdpSocket;
use std::error::Error;
//use std::io::net::ip::{Ipv4Addr, SocketAddr};
//use std::io::net::udp::UdpSocket;

// telnet
/*
 A TELNET connection is a Transmission Control Protocol (TCP)
   connection used to transmit data with interspersed TELNET control
   information.

   The TELNET Protocol is built upon three main ideas:  first, the
   concept of a "Network Virtual Terminal"; second, the principle of
   negotiated options; and third, a symmetric view of terminals and
   processes.
*/

// Rust's approach of separating data (struct & enum) from behaviour (impl) and using trait to group those behaviours for polymorphism

fn socket(listen_on: net::SocketAddr) -> net::UdpSocket {
    let attempt = net::UdpSocket::bind(listen_on);
    let mut socket;
    match attempt {
        Ok(sock) => {
            println!("Bound sock to {}", listen_on);
            socket = sock;
        },
        Err(err) => panic!("Could not bind: {}", err)
    }
    socket
}

fn main() {

    
/*
    let addr = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 0));
    let udp_socket = UdpSocket::bound(&addr).unwrap();

    let ipaddr = udp_socket.local_addr().unwrap();
    println!("udp_socket on port: {}", ipaddr);


   
    
    // spawn() accepts a close, which executes in a new thread - it returns the handle
    let handle = thread::spawn(move || {
        

        return "asdf"
    });

    // wait for completion and unwrap result
    println!("{}", handle.join().unwrap());


    println!("Guess ze numbr");

    // get rand num
    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Input: ");
        
        // Create mutable guess variable
        let mut guess = String::new();

        // read from stdin into our mutable variable
        io::stdin().read_line(&mut guess)
            .expect("Failed to readline");

        // annotate guess into a unsigned 32bit integer
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num, // if ok
            Err(_) => continue, // continue if err
        };
        

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less    => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal   => { 
                println!("Correct!");
                break;
            }
        }
    }
    */
}


fn patterns(x: i64) -> i32 {
    println!("x is: {}", x);


    // let statement is a ´´pattern´´, not a variable name (similar to unpacking in py?)
    let (x, y) = (1, 2);

    let x1: i32 = 5;


    x1 / x
}