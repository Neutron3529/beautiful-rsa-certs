extern crate openssl;
pub use openssl::{pkey::{PKey,Private},rsa::Rsa,error::ErrorStack,bn::{BigNumContext,BigNum,BigNumRef}};
pub struct WorkSpace(BigNumContext);
pub fn decode(s:&str)->Vec<u8>{
  let mut ret=Vec::with_capacity((s.len()+1)*3/4);
  let iter=s.as_bytes().chunks_exact(4);
  let v=|x:u8,shl:i8|->u32 {(match x{
    b'A'..=b'Z'=>(x-b'A'),
    b'a'..=b'z'=>(x-b'G'),//(x-b'a'+26)//(x-b'a'+b'Z'-b'A'+1) as u32
    b'0'..=b'9'=>x+4,
    b'+'|b'-'=>62,
    b'/'|b'_'=>63,
    _=>0
  } as u32) << shl};
  let x=iter.remainder();
  iter.for_each(|x|{let c=v(x[0],18)|v(x[1],12)|v(x[2],6)|v(x[3],0);ret.extend([(c>>16) as u8,(c>>8) as u8,c as u8])});
  let c=v(*x.get(0).unwrap_or(&0),24)|v(*x.get(1).unwrap_or(&0),16)|v(*x.get(2).unwrap_or(&0),8)|v(*x.get(3).unwrap_or(&0),0);
  match x.len(){
    4=>ret.extend([(c>>16) as u8,(c>>8) as u8,c as u8]),
    3=>ret.extend([(c>>16) as u8,(c>>8) as u8]),
    1..=2=>ret.push((c>>24) as u8),
    _=>(),
  }
  *ret.last_mut().unwrap()|=1;
  ret
}
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
    pub fn generate_beautiful_private_key(&mut self, beautiful:&str, bits:i32, safe_first:bool,safe_second:bool)->Result<Rsa<Private>,ErrorStack>{
        let vec=decode(beautiful);
        let one=BigNum::from_u32(1)?;

        let bitlevel=&one << (8*vec.len() as i32);
        let mut suffix=BigNum::from_u32(0)?;
        vec.into_iter().for_each(|x|{suffix=&suffix<<8;suffix.add_word(x as u32).unwrap()});

        let mut pinv=BigNum::new()?;
        let mut rem=BigNum::new()?;
        let p=Self::get_prime(bits,safe_first,None,None)?;
        pinv.mod_inverse(&p,&bitlevel,&mut self.0)?;
        rem.mod_mul(&pinv,&suffix,&bitlevel,&mut self.0)?;
        let q=Self::get_prime(bits,safe_second,Some(&bitlevel),Some(&rem))?;
        self.private_key_from_p_q(&p.to_dec_str()?,&q.to_dec_str()?)
    }

    /// generate private key from exist p and q, do not check if one of them is prime or not.
    pub fn private_key_from_p_q(&mut self, p:&str,q:&str)->Result<Rsa<Private>,ErrorStack>{
        let p=BigNum::from_dec_str(p)?;
        let q=BigNum::from_dec_str(q)?;
        let one=BigNum::from_u32(1)?;
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

    /// show pem of public key and private key.
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
        crate::WorkSpace::new().generate_beautiful_private_key("ABeaAAAAAA",1024,true,false)?;
        Ok(())
    }
}
