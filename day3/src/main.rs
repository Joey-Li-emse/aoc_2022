use std::{fs, collections::HashSet}; 
fn main() {
    let path = "./file/input.txt"; 
    let content = String::from_utf8(fs::read(path).unwrap()).unwrap(); 
    let bags : Vec<&str>= content
        .trim()
        .split("\n")
        .collect(); 

//     let test = 
// "vJrwpWtwJgWrhcsFMMfFFhFp
// jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
// PmmdzqPrVvPwwTWBwg
// wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
// ttgJtRGJQctTZtZT
// CrZsJsPPZsGzwwsLwLmpwMDw"; 

    // let bags : Vec<&str>= bags
    //     .trim()
    //     .split("\n")
    //     .collect(); 

    let mut max = 0; 
    let mut score:u32 = 0; 
    let mut double = Vec::new();
    for bag in bags.iter()
    {      
        //let mut score = 0; 
        let half = bag.len()/2;
        let items1 = bag[0..half].to_string(); 
        let items2 = bag[half..].to_string();
        let mut bag_double = Vec::new();
        

        println!("For a Bag {:?} {:?}", items1, items2); 
        for item in items1.chars()
        {
            if items2.contains(item) && !bag_double.contains(&item)
            {
                bag_double.push(item); 
            }
        }
        double.extend(bag_double);
    }

    for item in double.iter()
    {
        if item.is_lowercase()
        {   
            score += ((*item as u8) - 96) as u32; 
            println!("the letter : {:?} has value {}", item,  ((*item as u8) - 96) as u32);
        }
        else if item.is_uppercase()
        {
            score += ((*item as u8) - 38) as u32; 
            println!("the letter : {:?} has value {}", item,  ((*item as u8) - 38) as u32);
        }
        else {
            println!("Unknown char");
            break; 
        }
    }
    println!("The max in the bags is : {}", score);

    let mut group_list = Vec::new(); 
    let mut group = Vec::new();

    for (i, bag) in bags.iter().enumerate()
    {   
        group.push(bag);
        if (i % 3 == 2)
        {   
            group_list.push(group.clone()); 
            group.clear();
        }
    }
    drop(group); 

    let mut sum = 0; 
    let mut same = Vec::new(); 
    for group in group_list.iter()
    {
        let mem1 = group[0].chars().collect::<HashSet<_>>();
        let mem2 = group[1].chars().collect::<HashSet<_>>();
        let mem3 = group[2].chars().collect::<HashSet<_>>(); 

        let tmp = mem1.intersection(&mem2).cloned().collect::<HashSet<_>>();
        let common = mem3.intersection(&tmp).cloned().collect::<Vec<_>>();
        same.extend(common); 
    }

    for item in same.iter()
    {
        if item.is_lowercase()
        {   
            sum += ((*item as u8) - 96) as u32; 
            println!("the letter : {:?} has value {}", item,  ((*item as u8) - 96) as u32);
        }
        else if item.is_uppercase()
        {
            sum += ((*item as u8) - 38) as u32; 
            println!("the letter : {:?} has value {}", item,  ((*item as u8) - 38) as u32);
        }
        else {
            println!("Unknown char");
            break; 
        }
    }

    println!("{:?}", sum);
}
