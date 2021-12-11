/*
* Knowit Julekalender 2021
* Day 2
* @olafsm  
*/
use geoutils::Location;

fn main() {
    let mut locations:Vec<Location> = String::from(include_str!("../input.txt"))
        .lines()
        .map(|line| {
            let p:Vec<&str> = line
                .split("Point(")
                .nth(1)
                .unwrap()
                .split_whitespace()
                .collect();
            Location::new(
                p[1][..p[1].len()-1].parse::<f64>().unwrap(),
                p[0].parse::<f64>().unwrap(),
            )
        })
        .collect();
    let north_pole = Location::new(90.0,0.0);
    let mut current_loc = north_pole;
    let mut total_distance:f64 = 0.0;
    
    while !locations.is_empty() {
        let mut min_distance:f64 = f64::MAX;
        let mut min_index = 0;
        
        for (i,location) in locations.iter().enumerate() {
            let dist = location.haversine_distance_to(&current_loc).meters();
            if dist < min_distance {
                min_index = i;
                min_distance = dist;
            }
        }
        current_loc = locations.remove(min_index);
        total_distance += min_distance;
    }
    total_distance += current_loc.haversine_distance_to(&north_pole).meters();

    eprintln!("total_distance = {:?}", total_distance/1000.0);
}
