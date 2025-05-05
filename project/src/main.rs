mod read_csv;
mod cluster;
use crate::read_csv::DataFrame;
fn main() {
    let mut test_data = DataFrame::new(59622,9);
    let types: Vec<u32> = vec![1 as u32,1 as u32,1 as u32,4 as u32,4 as u32,4 as u32,4 as u32,3 as u32,4 as u32]; 
    let _ = test_data.read_csv("emissions_data.csv", &types);
    //test_data.print();  

    //let mut france_data = test_data.get_country_data("France"); 
    //france_data.unwrap().print();

    //let year_data = test_data.get_year_data(2021);
    //year_data.unwrap().print();
    //let unwrapped = year_data.unwrap();

    cluster::plot_scatter(&test_data.get_year_data(2018).expect("help"), "CO2 Emission(Tons)", "Population(2022)", "test1.png");

    //let mut basketball_data = DataFrame::new(5, 6);
    //let types: Vec<u32> = vec![1 as u32, 4 as u32, 3 as u32, 4 as u32, 4 as u32, 2 as u32];
    //let _ = basketball_data.read_csv("data.csv", &types);
    //println!("Printing data.csv");
    //basketball_data.print();
}
