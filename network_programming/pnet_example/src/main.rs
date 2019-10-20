// Network Programming with Rust
// Chapter 3: pnet_example

extern crate pnet;

use pnet::datalink::{self, NetworkInterface};
use pnet::datalink::Channel::Ethernet;
use pnet::packet::ethernet::{EtherTypes, EthernetPacket};
use pnet::packet::ipv4::Ipv4Packet;
use pnet::packet::tcp::TcpPacket;
use pnet::packet::ip::IpNextHeaderProtocol;
use pnet::packet::Packet;

use std::env;

// Handles a single ethernet packet

fn main() {
    println!("Hello, world!");
}
