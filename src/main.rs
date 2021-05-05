use std::fs::File;
use std::io::Read;
use duckbill::duckbill;

fn main() -> duckbill::DuckResult<()> {
    println!("Hello, world!");

    let mut f = File::open("UBQBILLS.527")?;

    let bills = duckbill::BillFile::new(&mut f)?;

    println!("header: {} {:?}", bills.get_header().len(), bills.get_header());

    println!("footer: {} {:?}", bills.get_footer().len(), bills.get_footer());

    let s = b"52-1188750-1";
    println!("bill: {} {:?}", bills.chop_single_bill(&s).unwrap().len(), bills.chop_single_bill(&s));
    println!("bill: {} {:?}", bills.chop_to_end(&s).unwrap().len(), bills.chop_to_end(&s));

    Ok(())
}
