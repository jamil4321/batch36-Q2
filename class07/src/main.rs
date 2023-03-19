#[derive(Debug)]
enum Transmition{
    Automatic(u8,u8),
    Manual(u8,u8,bool),
    Hybrid
}


// enum Coin{
//     Head,
//     Tale
// }


#[derive(Debug)]
struct CAR{
    name:String,
    modal:u16,
    transmition:Transmition
}



impl CAR{
    fn from(name:String,
        modal:u16,transmition:Transmition)->Self{
            Self{
                name,
                modal,
                transmition
            }
        }
        fn print(&self){
            println!("{:#?}",self)
         }
        fn transmission_data(&self){
            match self.transmition {
                Transmition::Automatic(gears,speed) =>{
                    println!("{} , {}",gears,speed)
                },
                _=>{
                    println!("No Found")
                }
            }
        }
    }
    

fn main(){

    let car_01 = CAR::from(String::from("Mehran"),1990,Transmition::Automatic(5,100));
    car_01.print();
    car_01.transmission_data();
    
}

