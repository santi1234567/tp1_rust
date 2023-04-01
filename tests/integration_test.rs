extern crate tp1_rust;
use crate::tp1_rust::game::play_game;
use crate::tp1_rust::utils::read_file;
#[test]
fn test_game() {
    let res = read_file(&"tables/game_B.txt");
    assert!(res.is_ok());
    let lines = res.unwrap();
    let res = play_game(lines);
    assert!(res.is_ok());
    let game_result = res.unwrap();
    assert_eq!(game_result, "B");
}
