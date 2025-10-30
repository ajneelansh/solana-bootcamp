use std::fmt::Error;

#[derive(Debug)]
struct Swap {
    a: u32,
    b: u32
}

trait Serialize {
    fn serialize(&self) -> Vec<u8>;
}

trait Deserialise {
    fn deserialise(v:&[u8]) ->  Result<Swap,Error>;
}

impl Deserialise for Swap {
    fn deserialise(v: &[u8]) ->  Result<Swap,Error> {
        if v.len() < 8{
            return Err(Error);
        }
        else {
          let a = u32::from_be_bytes([v[0],v[1],v[2],v[3]]);
          let b= u32::from_be_bytes([v[4],v[5],v[6],v[7]]);

          return Ok(Swap {
           a:a,
           b:b

          });
        }
    }
}

impl Serialize for Swap {
    fn serialize(&self) -> Vec<u8>{

       let mut v = vec![];
       v.extend_from_slice(&self.a.to_be_bytes());
       v.extend_from_slice(&self.b.to_be_bytes());
       return v;

    }
}

fn main(){
    let s = Swap{
        a: 2,
        b:4
    };
    let v = s.serialize();
    print!("{:?}",v);

    let x = Swap::deserialise(&v).unwrap();
    print!("{:?}",x);

}
