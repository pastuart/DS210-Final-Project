mod read_csv;
use crate::read_csv::DataFrame;
fn main() {
    //let mut test_data = DataFrame::new(59622,9);
    let types: Vec<u32> = vec![1 as u32,1 as u32,1 as u32,4 as u32,4 as u32,4 as u32,4 as u32,1 as u32,1 as u32]; 
    //let _ = test_data.read_csv("emissions_data.csv", &types);
    //test_data.print();
    //println!("Test");

    let mut basketball_data = DataFrame::new(5, 6);
    //let types: Vec<u32> = vec![1 as u32, 4 as u32, 3 as u32, 4 as u32, 4 as u32, 2 as u32];
    let _ = basketball_data.read_csv("emissions_data.csv", &types);
    println!("Printing data.csv");
    basketball_data.print();
}
