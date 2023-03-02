use std::fs; 

const RK : u32 = 1;
const PP : u32 = 2;
const SC : u32 = 3; 

const WIN : u32 = 6; 
const LOS : u32 = 0; 
const DRW : u32 = 3; 


#[derive(PartialEq, Debug)]
enum Play{
    Rock,
    Paper,
    Scissor,
    Unknown, 
}

fn main() {
    let path = "./file/input.txt";
    let content : String = String::from_utf8(fs::read(path).unwrap()).unwrap();
    let content: Vec<&str>= content.split("\n").collect(); 

    let mut strat : Vec<Vec<&str>> = Vec::new();  

    for element in content.iter()
    {
        let duel : Vec<&str> = element.split(" ").collect(); 
        strat.push(duel); 
    }
    
    let mut score_1:u32 = 0; 
    let mut score_2:u32 = 0; 
    for duel in strat{
        let npc = npc_convert(duel[0]);
        let me = pla_convert(duel[1]);
        let outcome = outcome(duel[1]); 
        let winning_play = to_win(&npc, outcome); 
        
        score_1 += me;   
        score_1 += who_win((npc, me));

        score_2 += outcome; 
        score_2 += winning_play; 
    }

    println!("At the first part we win {} points !",score_1);
    println!("At the second part we win {} points !",score_2);

}



fn who_win(duel : (Play, u32)) -> u32{
    if        (duel.0 == Play::Paper && duel.1 == 2) ||
              (duel.0 == Play::Rock && duel.1 == 1) || 
              (duel.0 == Play::Scissor && duel.1 == 3){
        return DRW
    }
    else if   (duel.0 == Play::Paper && duel.1 == 3) ||
              (duel.0 == Play::Rock && duel.1 == 2) || 
              (duel.0 == Play::Scissor && duel.1 == 1){
        return WIN
    }
    LOS
}

fn to_win(npc : &Play, outcome : u32) -> u32{
    if  (npc == &Play::Paper && outcome == 6)||
        (npc == &Play::Rock && outcome == 0)||
        (npc == &Play::Scissor && outcome == 3){
            SC
        }
    else if  (npc == &Play::Paper && outcome == 0)||
    (npc == &Play::Rock && outcome == 3)||
    (npc == &Play::Scissor && outcome == 6){
        return RK
    }
    else if  (npc == &Play::Paper && outcome == 3)||
    (npc == &Play::Rock && outcome == 6)||
    (npc == &Play::Scissor && outcome == 0){
        return PP
    }
    else{
        panic!("shouldn'tbe happening");
    }
    
}

fn outcome(out: &str) -> u32{
    match out{
        "X" => LOS,
        "Y" => DRW,
        "Z" => WIN,
        _ => 0, 
    }
}


fn npc_convert(play : &str) -> Play
{
    match play{
        "A" => Play::Rock,
        "B" => Play::Paper,
        "C" => Play::Scissor, 
        _   => Play::Unknown,
    }
}

fn pla_convert(play : &str) -> u32
{
    match play{
        "X" => RK,
        "Y" => PP,
        "Z" => SC, 
        _   => 0,
    }
}

#[cfg(test)]
mod tests
{
    use crate::npc_convert;
    use super::*; 
    #[test]
    fn npc_convert_test()
    {   
        let play = npc_convert("A");
        assert_eq!(play, Play::Rock); 
    }
}