extern crate openssl;
pub use openssl::{pkey::{PKey,Private},rsa::Rsa,error::ErrorStack,bn::{BigNumContext,BigNum,BigNumRef}};
extern crate base64;
pub struct WorkSpace(BigNumContext);
impl WorkSpace{
    /// create a new instance, may panic.
    pub fn new()->Self {
        WorkSpace(BigNumContext::new().unwrap_or_else(|x|{panic!("fatal error in creating a new WorkSpace, abort.\n{:?}",x)}))
    }

    /// create a new instance, return a result, useful if `new` always panic.
    pub fn init()->Result<Self,ErrorStack> {
        Ok(WorkSpace(BigNumContext::new()?))
    }

    /// get a prime in a desired bit size, just calling the code from openssl. bitlevel and rem is used
    /// to generate the correct q that p*q % bitlevel == rem, thus a beautiful tail is generated.
    pub fn get_prime(bits:i32,safe:bool,bitlevel:Option<&BigNumRef>,rem:Option<&BigNumRef>)->Result<BigNum,ErrorStack>{
        let mut ret=BigNum::new()?;
        let now = std::time::Instant::now();
        println!("get{}prime is called., bits={}, {:?}, {:?}{}",if safe {" safe "} else{" "},bits,bitlevel,rem,if safe{", it might takes a lot of time to finish."}else{"."});
        ret.generate_prime(bits, safe, bitlevel, rem)?;
        println!("cost {} ns ... got {}",now.elapsed().as_nanos(),ret.to_hex_str()?);
        Ok(ret)
    }

    /// This is not a SAFE cert, since the second prime is related to the first prime.
    /// But, that cert is beautiful, that's enough.
    /// Here, `beautiful` should be a vaild base64 string, it could only contains `0-9` `A-Z` `a-z` plus `+` and `/`.
    /// otherwise the program could panic.
    /// `n_suffix_div_2` is used to rearrange the output to ensure the willing beautiful chars would occur, rather than encoded by base64 in another way.
    /// it is good to keep n_suffix less than 4294967296, otherwise an overflow would occur thus the program may perform something you don't like.
    /// if you want to change suffix larger than 4 bytes, you could modify the `beautiful` string directly.
    pub fn generate_beautiful_private_key(&mut self, beautiful:&str, bits:i32, n_suffix_div_2:u32,safe_first:bool,safe_second:bool)->Result<Rsa<Private>,ErrorStack>{
        let vec=base64::decode(beautiful).unwrap();
        let end=n_suffix_div_2*2+1;
        let suffix_shl= ( 1 + (end > 256) as i32+(end > 65536) as i32 +(end > 16777216) as i32 ) * 8;
        let one=BigNum::from_u32(1)?;

        let bitlevel=&one << (suffix_shl+8*vec.len() as i32);
        let mut suffix=BigNum::from_u32(0)?;
        vec.into_iter().for_each(|x|{suffix=&suffix<<8;suffix.add_word(x as u32).unwrap()});
        suffix=&suffix<<suffix_shl;
        suffix.add_word(end)?;

        let mut pinv=BigNum::new()?;
        let mut rem=BigNum::new()?;
        let p=Self::get_prime(bits,safe_first,None,None)?;
        pinv.mod_inverse(&p,&bitlevel,&mut self.0)?;
        rem.mod_mul(&pinv,&suffix,&bitlevel,&mut self.0)?;
        let q=Self::get_prime(bits,safe_second,Some(&bitlevel),Some(&rem))?;
        let mut phi=BigNum::new()?;
        let n=&p*&q;
        let pm1=&p-&one;
        let qm1=&q-&one;
        phi.checked_mul(&pm1,&qm1,&mut self.0)?;
        let e=BigNum::from_u32(65537)?;
        let mut d=BigNum::new()?;
        d.mod_inverse(&e,&phi,&mut self.0)?;
        let mut dmp1=BigNum::new()?;
        let mut dmq1=BigNum::new()?;
        let mut iqmp=BigNum::new()?;
        dmp1.mod_inverse(&d,&pm1,&mut self.0)?;
        dmq1.mod_inverse(&d,&qm1,&mut self.0)?;
        iqmp.mod_inverse(&q,&p,&mut self.0)?;
        Rsa::from_private_components(n,e,d,p,q,dmp1,dmq1,iqmp)
    }
    pub fn show_pem(p:Rsa<Private>)->Result<String,ErrorStack>{
        let pkey = PKey::from_rsa(p)?;
        let pub_key: Vec<u8> = pkey.public_key_to_pem()?;
        let pri_key: Vec<u8> = pkey.private_key_to_pem_pkcs8()?;
        Ok(std::str::from_utf8(pub_key.as_slice()).unwrap().to_string()+"\n\n"+&std::str::from_utf8(pri_key.as_slice()).unwrap().to_string())
    }
}
#[cfg(test)]
mod tests {
    use crate::ErrorStack;
    #[test]
    fn it_works()->Result<(),ErrorStack> {
        println!("stage 0 finished.");
        crate::WorkSpace::new().generate_beautiful_private_key("ABeaAAAA",1024,1,true,false)?;
        assert_eq!(2 + 2, 4);
        Ok(())
    }
}
