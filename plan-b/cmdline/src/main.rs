// Copyright © 2018 Po Huit
// [This program is licensed under the "MIT License"]
// Please see the file LICENSE in the source
// distribution of this software for license terms.

// Plan B: EVE route planner with options
// Command-line demo client
extern crate noisy_float;
use noisy_float::prelude::*;

extern crate plan_b;
use plan_b::map::*;
use plan_b::search::*;

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

// Modified function for finding shortest route to high sec
fn find_route_sec(map: &Map, start: &str, goal_sec: R64) -> Vec<SystemId> {
    let start_id = find_system(&map, start);
    shortest_route_sec(&map, start_id, goal_sec)
        .expect(&format!("no route found from {} to high sec", start))
}

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
