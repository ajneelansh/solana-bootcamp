pub struct Square {
    side : i32,
}

impl Square {

    pub fn new(side : i32) -> Self {
        Self{side}
    }

    pub fn can_hold(&self,sq: &Square) -> bool {
       self.side > sq.side
    }
}

#[cfg(test)]
mod test {
    use super::*;


    #[test]
    fn can_hold_smaller(){
        let square1 = Square::new(5);
        let square2 = Square::new(4);
        assert!(square1.can_hold(&square2))
    }

}

