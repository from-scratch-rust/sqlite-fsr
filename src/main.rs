use sqlite_fsr::run;
fn main()  {
    let args = std::env::args().collect::<Vec<String>>();
    let output = run(&args);
    match output {
        Ok(o) => println!("{}", o),
        Err(e) => println!("ERROR: {}", e)
    }
    
}