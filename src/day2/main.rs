use std::include_str;

fn score_event(event: &str) -> usize {
    match event {
        "A X" => 4,
        "B X" => 1,
        "C X" => 7,
        "A Y" => 8,
        "B Y" => 5,
        "C Y" => 2,
        "A Z" => 3,
        "B Z" => 9,
        "C Z" => 6,
        _ => panic!()
    }
}
fn part_1(){
    let total_score = include_str!("input").lines().map(|x| score_event(x)).sum::<usize>();
    println!("The total score would be: {}.", total_score);
}
fn rig_the_game<'a>(event: &'a str) -> &'a str{
    match event {
        "A X" => "A Z",
        "B X" => "B X",
        "C X" => "C Y",
        "A Y" => "A X",
        "B Y" => "B Y",
        "C Y" => "C Z",
        "A Z" => "A Y",
        "B Z" => "B Z",
        "C Z" => "C X",
        _ => panic!()
    }
}
fn part_2(){
    let total_score = include_str!("input").lines().map(|x| score_event(rig_the_game(x))).sum::<usize>();
    println!("The total score after rigging the game would be: {}.", total_score);

}

fn main() {
    part_1();
    part_2();
}