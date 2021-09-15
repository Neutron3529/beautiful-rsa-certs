use beautiful_rsa_certs::{WorkSpace,ErrorStack};
fn main()->Result<(),ErrorStack> {
    println!("stage 0 finished.");
    println!("{}",WorkSpace::show_pem(WorkSpace::new().generate_beautiful_private_key("++ThisIsAReallyBeautifulEnding+++A",2048,127,true,true)?).unwrap());
    assert_eq!(2 + 2, 4);
    Ok(())
}
