use std::collections::{BTreeSet, HashSet};
use std::fmt::format;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};

#[derive(Clone, Copy, Debug)]
struct Bin {
    pub from: u64,
    pub to: u64,
}

fn standardize(pan: u64) -> u64 {
    #![feature(int_log)]
    let len = pan.to_string().len() as u32;
    let shift = 10_u64.pow(6 - len);
    pan * shift
}


impl Bin {
    pub fn new(from: u64, to: u64) -> Bin {
        let (from, to) = Bin::standardize(from, to);
        Bin { from, to }
    }

    fn standardize(from: u64, to: u64) -> (u64, u64) {
        #![feature(int_log)]
        let len = from.to_string().len() as u32;
        let shift = 10_u64.pow(6 - len);
        (
            from * shift,
            to * shift + shift - 1
        )
    }

    pub fn to_string(&self) -> String {
        format!("{} - {}", self.from, self.to)
    }
}

struct BinTable {
    pub table: Vec<(u64, Vec<Bin>)>,
}

impl BinTable {
    pub fn from(bin_ranges: Vec<Bin>) -> BinTable {
        let mut all: BTreeSet<u64> = bin_ranges.clone().into_iter().map(|ft| vec![ft.from, ft.to]).flatten().collect();
        all.insert(0);
        let b1: Vec<u64> = all.into_iter().collect();

        let mut table: Vec<(u64, Vec<Bin>)> = b1.into_iter().map(|b| (b, Vec::new())).collect();
        table.push((u64::MAX, Vec::new()));

        for bin in bin_ranges {
            for t in 0..(table.len() - 2) {
                if bin.from <= table[t].0 && bin.to >= table[t + 1].0 {
                    table[t].1.push(bin);
                }
            }
        }
        BinTable { table }
    }

    pub fn to_pretty_string(&self) -> String {
        let mut output = String::new();
        for bin in &self.table {
            output = format!("{}\n{}", output, bin.0);
            for bins in &bin.1 {
                output = format!("{}\n    {}", output, bins.to_string());
            }
        }
        output
    }

    pub fn locate(&self, pan: u64, search_index: usize) -> Option<Vec<Bin>> {
        if search_index > self.table.len() - 2 { return Some(self.table[self.table.len() - 1].1.clone()); }
        match (pan >= self.table[search_index].0, pan <= self.table[search_index + 1].0) {
            (true, true) => Some(self.table[search_index].1.clone()),
            (true, false) => self.locate(pan, search_index / 2 + search_index),
            (false, true) => self.locate(pan, search_index / 2),
            (false, false) => None,
        }
    }
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}


#[cfg(test)]
mod tests {
    use crate::{Bin, BinTable, standardize};

    #[test]
    fn it_works() {
        let bin1 = Bin::new(100, 200);
        let bin2 = Bin::new(100, 300);
        let bin3 = Bin::new(200, 300);
        let bin4 = Bin::new(150, 250);
        let bin5 = Bin::new(150, 200);

        let table = vec![bin1, bin2, bin3, bin4, bin5];
        let bin_table = BinTable::from(table);

        println!("{}", bin_table.to_pretty_string());

        let pan = standardize(195456);
        println!("{} -> {:?}", pan, bin_table.locate(pan, bin_table.table.len()/2) );


        let pan = standardize(345);
        println!("{} -> {:?}", pan, bin_table.locate(pan, bin_table.table.len()/2) );

        let pan = standardize(20963);
        println!("{} -> {:?}", pan, bin_table.locate(pan, bin_table.table.len()/2) );
    }
}