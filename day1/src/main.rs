use std::fs; 
use std::collections::HashMap;
fn main() {
    let path = "./file/input.txt";
    let input  = String::from_utf8(fs::read(path).unwrap()).unwrap();
    let array: Vec<&str> = input.split("\n\n").collect(); 
    let mut cal_hash: HashMap<u32, Vec<u32>> = HashMap::new(); 
    for food in array.iter()
    {
        let food_cals : Vec<&str> = food.split("\n").collect();
        let mut calories : Vec<u32> = Vec::new();
        
        for food_cal in food_cals.iter()
        {
            let calorie : u32 = food_cal.parse().unwrap();
            calories.push(calorie);  
        }
        let cal_tot: u32 = calories.iter().sum();
        cal_hash.insert(cal_tot, calories); 
    } 

    let fat_elf = cal_hash.iter().max_by_key(|entry| entry.1).unwrap();
    println!("The strongest elf is carrying {} calories", fat_elf.0);

    let mut sum : u32 = 0; 
    for _ in 0..3
    {   
        let idx; 
        
        {
            let fat_elfs = cal_hash.iter().max_by_key(|entry| entry.0).unwrap().0;
            idx = *fat_elfs; 
            sum += idx; 
        }
        cal_hash.remove(&idx);
    }
    
    println!("The Big 3 elfs are carrying {:?} calories ", sum); 

    
}
