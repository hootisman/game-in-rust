use bird_game_rust::run;
pub fn main(){
    pollster::block_on(run());
}
