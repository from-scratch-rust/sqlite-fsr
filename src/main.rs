use sqlite_fsr::run;
pub mod varint;
fn main()  {
    let args = std::env::args().collect::<Vec<String>>();
    let output = run(&args);
    match output {
        Ok(o) => println!("{}", o),
        Err(e) => println!("ERROR: {}", e)
    }
    
}