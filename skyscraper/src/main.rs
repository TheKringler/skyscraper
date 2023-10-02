use std::env;
fn main() {
    let args: Vec<String> = env::args().collect();
    let grid_size = match args.get(1){
        Some(val) => match val.parse::<u64>(){
            Ok(result) => result,
            Err(e) => {
                println!("Unable to parse command line argument: {}! Needs to be positive integer!", e);
                return; 
            }
        },
        None => {
            println!("Not enough arguments provided!");
            return;
        }
    };
    if grid_size <= 2{
        println!("Grid width and height should be bigger than 2");
        return;
    }
    println!("Grid size: {}", grid_size);


}

pub fn create_game(grid_size: u64){
    let mut grid = vec![0, grid_size * grid_size];

}

pub fn load_game(){

}


mod tests{
    #[test]
    fn test_1(){
        let grid = [[0, 4, 1, 2, 2, 0],
                                    [2, 1, 4, 3, 2, 3],
                                    [3, 2, 3, 4, 1, 2],
                                    [2, 3, 1, 2, 4, 1],
                                    [1, 4, 2, 1, 3 ,2],
                                    [0, 1, 3, 3, 2, 0]];
        let grid_size = 4;
    }
}
