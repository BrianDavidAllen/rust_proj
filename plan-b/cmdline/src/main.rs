// Copyright © 2018 Po Huit
// [This program is licensed under the "MIT License"]
// Please see the file LICENSE in the source
// distribution of this software for license terms.

// Plan B: EVE route planner with options
// Command-line demo client
extern crate noisy_float;
use noisy_float::prelude::*;

extern crate sqlite3;
use sqlite3::State;

extern crate plan_b;
use plan_b::map::*;
use plan_b::search::*;

extern crate rand;
use rand::Rng;

// Look up the given system name in the map, and panic if
// not found. This should be cleaned up.
fn find_system(map: &Map, name: &str) -> SystemId {
    map.by_name(name)
        .expect(&format!("could not find {} in map", name))
        .system_id
}

// Find a shortest route by name, or panic if none exists.
fn find_route(map: &Map, start: &str, goal: &str) -> Vec<SystemId> {
    let start_id = find_system(&map, start);
    let goal_id = find_system(&map, goal);
    shortest_route(&map, start_id, goal_id)
        .expect(&format!("no route found from {} to {}", start, goal))
}
///Left af the find_route_blahs in cmd line and web interface seperate --Brian Allen 
// Modified function for finding shortest route to high sec
fn find_route_sec(map: &Map, start: &str, goal_sec: R64) -> Vec<SystemId> {
    let start_id = find_system(&map, start);
    shortest_route_sec(&map, start_id, goal_sec)
        .expect(&format!("no route found from {} to high sec", start))
}

// Modified function for finding shortest route to major trade hub --Brian Allen 
fn find_route_hub_major(map: &Map, start: &str) -> Vec<SystemId> {
    let start_id = find_system(&map, start);
    shortest_route_hub_major(&map, start_id)
        .expect(&format!("no route found from {} to high sec", start))
}

//Modified function for find shortest route to major trade hub --Brian Allen 
fn find_route_hub_minor(map: &Map, start: &str) -> Vec<SystemId> {
    let start_id = find_system(&map, start);
    shortest_route_hub_minor(&map, start_id)
        .expect(&format!("no route found from {} to high sec", start))
}

#[test]
// Check for correct computation of a long route.
fn short_route_north_south() {
    let map = Map::fetch().expect("could not open map");
    let route = find_route(&map, "B-GC1T", "2UK4-N");
    assert_eq!(80, route.len());
}

#[test]
//check that route_sec and route outputs are equal with same input --Brian Allen 
fn test_route_sec() {
    let map = Map::fetch().expect("could not open map");
    let mut rng = rand::thread_rng();
    let systems = map.systems_ref();    
    let number = systems.len();
    let mut rand_system: usize = rng.gen::<usize>();
    rand_system = rand_system % number;
    let system = &systems[rand_system];
    
    let first_route = find_route_sec(&map, &system.name, r64(0.5));

    //had to use database to get from system_id to name.
    let connection = sqlite3::open("../kill_data/eve_system.db").expect("Error opening data base");
    let mut end = String::new();
    let mut statement = connection
            .prepare("Select * FROM systems WHERE system_id = ?")
            .expect("error with statemtn");
        statement
            .bind(1, first_route[first_route.len() -1].0 as i64)
            .expect("error binding statement");
        while let State::Row = statement.next().unwrap() {
            end = statement
                .read::<String>(1)
                .expect("Error getting system_name");
        }
    let second_route = find_route(&map, &system.name, &end);

    assert_eq!(first_route, second_route);
}

#[test]
//check that route_sec and route outputs are equal with same input --Brian Allen 
fn test_route_major() {
    let map = Map::fetch().expect("could not open map");
    let mut rng = rand::thread_rng();
    let systems = map.systems_ref();    
    let number = systems.len();
    let mut rand_system: usize = rng.gen::<usize>();
    rand_system = rand_system % number;
    let system = &systems[rand_system];
    
    let first_route = find_route_hub_major(&map, &system.name);

    //had to use database to get from system_id to name.
    let connection = sqlite3::open("../kill_data/eve_system.db").expect("Error opening data base");
    let mut end = String::new();
    let mut statement = connection
            .prepare("Select * FROM systems WHERE system_id = ?")
            .expect("error with statemtn");
        statement
            .bind(1, first_route[first_route.len() -1].0 as i64)
            .expect("error binding statement");
        while let State::Row = statement.next().unwrap() {
            end = statement
                .read::<String>(1)
                .expect("Error getting system_name");
        }
    let second_route = find_route(&map, &system.name, &end);

    assert_eq!(first_route, second_route);
}
 #[test]
fn test_route_minor() {
    let map = Map::fetch().expect("could not open map");
    let mut rng = rand::thread_rng();
    let systems = map.systems_ref();    
    let number = systems.len();
    let mut rand_system: usize = rng.gen::<usize>();
    rand_system = rand_system % number;
    let system = &systems[rand_system];
    
    let first_route = find_route_hub_minor(&map, &system.name);

    //had to use database to get from system_id to name.
    let connection = sqlite3::open("../kill_data/eve_system.db").expect("Error opening data base");
    let mut end = String::new();
    let mut statement = connection
            .prepare("Select * FROM systems WHERE system_id = ?")
            .expect("error with statemtn");
        statement
            .bind(1, first_route[first_route.len() -1].0 as i64)
            .expect("error binding statement");
        while let State::Row = statement.next().unwrap() {
            end = statement
                .read::<String>(1)
                .expect("Error getting system_name");
        }
    let second_route = find_route(&map, &system.name, &end);

    assert_eq!(first_route, second_route);
}
// Command-line Plan B. */
fn main() {
    // Set up the map.
    let map = Map::fetch().expect("could not open map");

    // Get and process the first argument.
    let mut args = std::env::args();
    let start = (&mut args).skip(1).next().expect("no source");
    if start == "--diameter" {
        // Run the diameter calculation and display the result.
        let diameter_info = diameter(&map);
        println!("diameter {}", diameter_info.diameter);
        for (start, end) in diameter_info.longest {
            let start = &map.by_system_id(start).name;
            let end = &map.by_system_id(end).name;
            println!("{} → {}", start, end);
        }
        return;
    }
    //Block for getting highsec route
    if start == "--highsec" {
        //Flag for finding shortest path to highsec.
        let goal = r64(0.5);
        let start_sec = &(&mut args).next().expect("no source");
        let route = find_route_sec(&map, &start_sec, goal);
        for system_id in route {
          let system = map.by_system_id(system_id);
          println!("{}", system.name);
        }
        return;
    }
    //Block for geting major trade hub route
    //delete start_sec use start?
    if start == "--major_hub" {
        //Flag for finding shortest path to highsec.
        let start_sec = &(&mut args).next().expect("no source");
        let route = find_route_hub_major(&map, &start_sec);
        for system_id in route {
          let system = map.by_system_id(system_id);
          println!("{}", system.name);
        }
        return;
    }

    //Block for getting minor trade hub route 
    if start == "--minor_hub" {
        //Flag for finding shortest path to highsec.
        let start_sec = &(&mut args).next().expect("no source");
        let route = find_route_hub_minor(&map, &start_sec);
        for system_id in route {
          let system = map.by_system_id(system_id);
          println!("{}", system.name);
        }
        return;
    }

    

    // Get the destination, find the route and display it.
    let goal = (&mut args).next().expect("no destination");
    let route = find_route(&map, &start, &goal);
    for system_id in route {
        let system = map.by_system_id(system_id);
        println!("{}", system.name);
    }
}
