mod chlg_2477;

fn main() {
    println!("Hello, world!");

    //let roads = vec![vec![0, 1, 2], vec![1, 2, 4], vec![2, 0, 8], vec![1, 0, 16]];
    //let seats = 3;

    let roads = vec![vec![0, 1, 1], vec![0, 2, 5], vec![1, 2, 1], vec![2, 3, 1]];
    let seats = 4;

    let result = chlg_2477::minimum_fuel_cost(roads, seats);
    println!("result: {}", result);
}
