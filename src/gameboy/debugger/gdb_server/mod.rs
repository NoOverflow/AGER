use std::{
    fmt,
    io::{self, BufRead, BufReader, Error, Read, Write},
    net::{TcpListener, TcpStream},
    sync::{Arc, Mutex},
    thread::{self, JoinHandle},
};

use gdk::glib::PropertyGet;
use rdb_packet::RdbPacket;

use crate::{gameboy::{bin_utils::BinUtils, registers::Registers, Gameboy}, views::logic::game};

mod rdb_packet;

pub struct GdbServer {
    listener: Option<TcpListener>,
    client: Option<TcpStream>,
    gameboy: Arc<Mutex<Gameboy>>,
}

impl GdbServer {
    fn bind(&mut self) -> Result<(), std::io::Error> {
        match TcpListener::bind("0.0.0.0:3333") {
            Ok(listener) => {
                self.listener = Some(listener);
            }
            Err(e) => {
                eprintln!("Failed to bind GDB server to port 3333: {}", e);
                return Err(e);
            }
        };
        Ok(())
    }

    pub fn new(gameboy: Arc<Mutex<Gameboy>>) {
        thread::spawn(move || {
            let mut gdb_server: Self = Self {
                listener: None,
                client: None,
                gameboy,
            };

            println!("GDB server started, pausing emulation to await GDB client connection on port 3333.");
            gdb_server.gameboy.lock().unwrap().debugger.state.paused = true;
            if gdb_server.bind().is_err() {
                return;
            }
            // Accept a single GDB client, since we can only handle one at a time.
            let client: Result<TcpStream, Error> = gdb_server.accept();

            match client {
                Ok(stream) => {
                    gdb_server.client = Some(stream);
                }
                Err(_) => {
                    return;
                }
            }
            println!("GDB client connected.");
            gdb_server.read_packet();
        });
    }

    fn build_registry(registers: &Registers) -> String {
        return format!(
            "{:04x}{:04x}{:04x}{:04x}{:04x}{:08x}{:0>24}",
            BinUtils::u16_from_u8s(registers.f.into(), registers.a),
            BinUtils::u16_from_u8s(registers.c, registers.b),
            BinUtils::u16_from_u8s(registers.e, registers.d),
            BinUtils::u16_from_u8s(registers.l, registers.h),
            registers.sp.rotate_left(8),
            registers.pc.rotate_left(8),
            ""
        );
    }

    fn build_memory(packet: &RdbPacket, gameboy: Arc<Mutex<Gameboy>>) -> String {
        println!("Data: {} {}", packet.data[0], packet.data[1]);
        let start_address = usize::from_str_radix(&packet.data[0][1..packet.data[0].len()], 16);
        let length = usize::from_str_radix(packet.data[1].as_str(), 16);

        if start_address.is_err() || length.is_err() {
            println!("Incorrectly formatted memory read packet");
            return String::new()
        }
        println!("Read {:02x} bytes from {:02x}", length.clone().unwrap(), start_address.clone().unwrap());
        GdbServer::read_memory(start_address.unwrap(), length.unwrap(), gameboy)
    }

    fn read_memory(address: usize, length: usize, gameboy: Arc<Mutex<Gameboy>>) -> String {
        let mut ret: String = String::with_capacity(length * 2 + 1);
        let memory_slice: Vec<u8> = gameboy.lock().unwrap().mem_map.read_u8_slice(address, length);

        ret.push_str("b,");
        for i in 0..length {
            ret.push_str(format!("{:02x}", memory_slice[i]).as_str());
        }
        ret
    }

    fn treat_packet(&mut self, packet: &RdbPacket) -> io::Result<usize> {
        let command = packet.data[0].as_str();
        let empty_command_responses =
            ["qTStatus", "vMustReplyEmpty", "Hg0", "qfThreadInfo", "qL", "qC", "Hc", "p", "vCont?"];

        for empty_command in empty_command_responses {
            if command.starts_with(empty_command) {
                return self.send_response(format!("+{}", RdbPacket::packetify("")))
            }
        }
        if command.starts_with("x") {
            return self.send_response(format!("+{}", RdbPacket::packetify(GdbServer::build_memory(packet, self.gameboy.clone()).as_str())));
        }
        match packet.data[0].as_str() {
            "qSupported" => self.send_response(format!("+{}", RdbPacket::packetify("hwbreak+"))),
            "qAttached" => self.send_response(format!("+{}", RdbPacket::packetify("1"))),
            "g" => self.send_response(format!(
                "+{}",
                RdbPacket::packetify(
                    GdbServer::build_registry(&self.gameboy.lock().unwrap().cpu.registers).as_str()
                )
            )),
            "?" => self.send_response(format!("+{}", RdbPacket::packetify("S05"))),
            _ => {
                eprintln!("Unknown command: {}", packet.data[0]);
                self.send_response("-".to_string());
                Ok(0)
            }
        }
    }

    fn send_response(&mut self, response: String) -> io::Result<usize> {
        println!("Sending response: {}", response);
        self.client
            .as_ref()
            .unwrap()
            .write(response.bytes().collect::<Vec<u8>>().as_slice())
    }

    fn read_packet(&mut self) {
        loop {
            let mut rx_buf = vec![0; u16::MAX as usize];
            let mut reader = BufReader::new(self.client.as_ref().unwrap());

            match reader.read(&mut rx_buf) {
                Ok(length) => {
                    rx_buf.truncate(length);
                    let raw_packet = String::from_utf8(rx_buf).unwrap();
                    let sub_packets: Vec<&str> = raw_packet.split("$").collect();

                    for sub_packet in sub_packets {
                        // Ignore empty subpackets, or confirmation packets
                        if sub_packet.is_empty()
                            || sub_packet.starts_with("+")
                            || sub_packet.starts_with("-")
                        {
                            continue;
                        }
                        match RdbPacket::parse(sub_packet) {
                            Ok(packet) => {
                                self.treat_packet(&packet);
                            }
                            Err(_) => {
                                eprintln!("Couldn't read packet");
                            }
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Failed to read GDB packet: {}, couldn't read socket.", e);
                    break;
                }
            }
        }
    }

    fn accept(&self) -> Result<TcpStream, Error> {
        match self.listener.as_ref().unwrap().accept() {
            Ok((stream, _)) => Ok(stream),
            Err(e) => {
                eprintln!("Failed to accept GDB connection: {}", e);
                Err(e)
            }
        }
    }
}
