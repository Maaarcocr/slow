extern crate regex;
#[macro_use] extern crate lazy_static;
mod writer;
mod slow;
mod request;

use request::*;
use slow::*;
use writer::*;

fn homepage(request: Request, mut writer: Writer) {
    writer.add_header("Cache", "No");
    writer.add_header("Okay", "ciao");
    writer.write("Home Page");
}

fn marco(request: Request, mut writer: Writer) {
    writer.add_header("Ciao", "CIAAAAO");
    writer.write("Marco");
}

fn main() {
    let mut slow = Slow::new();
    slow.add_handler("/", homepage);
    slow.add_handler("/marco", marco);
    slow.start("80");
}
