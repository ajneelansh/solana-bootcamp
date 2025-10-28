struct Car {
    model: String,
    year: u32
}
impl Car {
    fn calculate_price(&self) -> u32 {
        let i;
        if self.model=="S" {
            i = 50000
        }
        else {
            i = 40000
        } 
        return i*self.year; 
    }
}




fn main() {
    let lambo : Car = Car {
       model : String::from("S"),
       year : 2003
    };

    print!("{}",lambo.calculate_price());
}
