struct BUS {
    name: String,
    no_of_seat: u8,
    modal: u16,
    tank_size: u16,
    distance_cover: u16,
}
impl BUS {
    // assotiave self
    fn init(name: String, no_of_seat: u8, modal: u16, tank_size: u16, distance_cover: u16) -> Self {
        Self {
            name,
            no_of_seat,
            modal,
            tank_size,
            distance_cover,
        }
    }
    // method self 
    fn millage(&self, tank_size: u16) -> u16 {
        self.distance_cover / tank_size
    }
}

// tank distance_cover
fn main() {
    let obj_bus_02 = BUS::init(String::from("Mazda"), 44, 1998, 23, 500);
    println!(
        "The {} milage is {}",
        obj_bus_02.name,
        obj_bus_02.millage(100)
    )
}
