use beautiful_rsa_certs::{WorkSpace,ErrorStack};
fn main()->Result<(),ErrorStack> {
    println!("stage 0 finished.");
    println!("{}",WorkSpace::show_pem(WorkSpace::new().generate_beautiful_private_key("////Do+you+really+think+this+is+a+vaild+rsa+public+key//Actually+it+is+just+a+toy+which+happened+to+work+with+openssh////",2048,false,false)?).unwrap());
    assert_eq!(2 + 2, 4);
    Ok(())
}
