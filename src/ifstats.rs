//  This file is part of the ppnd distribution, which can be found at 
//  <https://github.com/gdtrfb55/ppnd>.
//
//  Copyright (C) 2021 Jack Browning.
//
//  This program is FREE SOFTWARE: you can redistribute it and/or modify
//  it under the terms of the GNU General Public License as published by
//  the Free Software Foundation, either version 3 of the License, or
//  (at your option) any later version.
//
//  This program is distributed in the hope that it will be useful,
//  but WITHOUT ANY WARRANTY; without even the implied warranty of
//  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
//  GNU General Public License for more details.
//
//  You should have received a copy of the GNU General Public License
//  along with this program. If not, see <https://www.gnu.org/licenses/>.

struct RXStats {
    octets: u64,
    packets: u64,
    errors: u64,
    drops: u64,
    fifo: u64,
    compressed: u64,
    frames: u64,
    multicast: u64
}

impl RXStats {
    fn new(stats: &[u64]) -> RXStats {
        RXStats {
            octets: stats[0],
            packets: stats[1],
            errors: stats[2],
            drops: stats[3],
            fifo: stats[4],
            compressed: stats[5],
            frames: stats[6],
            multicast: stats[7]
        }
    }
}

struct TXStats {
    octets: u64,
    packets: u64,
    errors: u64,
    drops: u64,
    fifo: u64,
    compressed: u64,
    collisions: u64,
    carrier: u64
}

impl TXStats {
    fn new(stats: &[u64]) -> TXStats {
        TXStats {
            octets: stats[0],
            packets: stats[1],
            errors: stats[2],
            drops: stats[3],
            fifo: stats[4],
            compressed: stats[5],
            collisions: stats[6],
            carrier: stats[7]
        }
    }
}

pub struct IFStats {
    pub name: String,
    width: usize,
    rx: RXStats,
    tx: TXStats
}

use crate::bytescale::{Scale, scale_bytes};

impl IFStats {
    fn new(name: String, width: usize, rx: RXStats, tx: TXStats) -> IFStats {
        IFStats { name, width, rx, tx }
    }

    pub fn print(mut self: Self, divisor: &Scale, precision: usize) {
        let scaled_rx = scale_bytes(self.rx.octets, divisor, precision);
        let scaled_tx = scale_bytes(self.tx.octets, divisor, precision);
        let rx_len = scaled_rx.len();
        let tx_len = scaled_tx.len();
        if rx_len > self.width { self.width = rx_len };
        if tx_len > self.width { self.width = tx_len };

        println!("\n{}\n", self.name);
        println!("RX Bytes      {:>2$}  |  TX Bytes      {:>2$}", scaled_rx, scaled_tx, self.width);
        println!("RX Packets    {:>2$}  |  TX Packets    {:>2$}", self.rx.packets, self.tx.packets, self.width);
        println!("RX Errors     {:>2$}  |  TX Errors     {:>2$}", self.rx.errors, self.tx.errors, self.width);
        println!("RX Drops      {:>2$}  |  TX Drops      {:>2$}", self.rx.drops, self.tx.drops, self.width);
        println!("RX FIFO       {:>2$}  |  TX FIFO       {:>2$}", self.rx.fifo, self.tx.fifo, self.width);
        println!("RX Compressed {:>2$}  |  TX Compressed {:>2$}", self.rx.compressed, self.tx.compressed, self.width);
        println!("RX Frames     {:>2$}  |  TX Collisions {:>2$}", self.rx.frames, self.tx.collisions, self.width);
        println!("RX Multicast  {:>2$}  |  TX Carrier    {:>2$}", self.rx.multicast, self.tx.carrier, self.width);
    }
}

fn parse(netdev_line: &str) -> Vec<&str> {
    netdev_line
        .split_whitespace()
        .collect()
}

fn field_to_u64(field: &str) -> Result<u64, String> {
    if let Ok(f) = field.parse::<u64>() { return Ok(f) };
    Err("error parsing /proc/net/dev interface data".to_string())
}

fn convert(if_fields: &Vec<&str>) -> Result<(usize, Vec<u64>), String> {
    let mut len: usize;
    let mut width: usize = 0;
    let mut stats: Vec<u64> = Vec::with_capacity(if_fields.len());
    for f in if_fields {
        len = f.len();
        if len > width { width = len };
        stats.push(field_to_u64(f)?);
    }
    Ok((width, stats))
}

pub fn new(netdev_line: &str) -> Result<IFStats, String> {
    let mut if_fields = parse(netdev_line);
    let if_name = if_fields.remove(0);
    let (width, raw_stats) = convert(&if_fields)?;
    let rx_stats = RXStats::new(&raw_stats[..8]);
    let tx_stats = TXStats::new(&raw_stats[8..]);
    Ok(IFStats::new(if_name.to_string(), width, rx_stats, tx_stats))
}
