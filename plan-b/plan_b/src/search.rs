// Copyright © 2018 Po Huit
// [This program is licensed under the "MIT License"]
// Please see the file LICENSE in the source
// distribution of this software for license terms.

//! Search functionality for Plan B.

extern crate ndarray;


//Again needed noisy floats for Ord and Eq on waypoints --Brian Allen 
extern crate noisy_float;
use search::noisy_float::prelude::*;

use self::ndarray::Array2;
    
use std::collections::VecDeque;
use std::collections::HashMap;

use map::*;

/// Results from a `diameter()` calculation.
pub struct DiameterInfo {
    /// Diameter of EVE.
    pub diameter: usize,
    /// List of endpoints with shortest route of
    /// length equal to diameter.
    pub longest: Vec<(SystemId, SystemId)>,
}

/// An entry in the all-pairs shortest-path table.
#[derive(Clone, Copy)]
pub struct Hop {
    /// System id of next hop.
    pub system_id: SystemId,
    /// Distance from start to here.
    pub dist: usize,
}

/// Table of all-pairs shortest paths.
pub type APSPTable = Array2<Option<Hop>>;

/// An intermediate step in the BFS shortest path search.
#[derive(Clone, PartialOrd, Ord, PartialEq, Eq)]
struct Waypoint {
    /// Distance from start.
    dist: usize,
    /// System id of current system.
    cur: SystemId,
    /// Next hop back toward parent, if not at start.
    parent: Option<SystemId>,
    ///Added noisy_float for highsec search --Brian Allen  
    sec: R64,
}

impl Waypoint {
    /// Create a new waypoint; mild syntactic sugar.
    // Added sec value to waypoint
    fn new(dist: usize, cur: SystemId, parent: Option<SystemId>, sec: R64)
           -> Waypoint
    {
        Waypoint{dist, cur, parent, sec}
    }
}

// Single-source shortest path via Breadth-First Search.
// Returns a waypoint map for further processing.
fn bfs(map: &Map, start: SystemId, goal: Option<SystemId>)
            -> HashMap<SystemId, Waypoint>
{
    // Set up data structures and run the search.
    let mut q = VecDeque::with_capacity(map.systems_ref().len());
    let mut closed = HashMap::new();
    //Added sec value to waypoint init --Brian Allen 
    q.push_back(Waypoint::new(0, start, None, r64(0.1)));
    loop {
        // Examine best waypoint.
        let waypoint = match q.pop_front() {
            Some(waypoint) => waypoint,
            None => return closed,
        };
        if closed.contains_key(&waypoint.cur) {
            continue;
        }
        closed.insert(waypoint.cur, waypoint.clone());

        // If we have found the goal, we are done.
        if goal == Some(waypoint.cur) {
            return closed;
        }

        // Open the children of the current system.
        let map_info = map.by_system_id(waypoint.cur);
        for child in map_info.stargates.iter() {
            let child_waypoint = Waypoint::new(
                waypoint.dist + 1,
                *child,
                Some(waypoint.cur),
                map_info.sec_status,
            );
            q.push_back(child_waypoint);
        }
    }
}

/// Return a shortest route if one exists.
pub fn shortest_route(map: &Map, start: SystemId, goal: SystemId)
                      -> Option<Vec<SystemId>>
{
    // Find single-source shortest paths from start up to goal.
    let waypoints = bfs(map, start, Some(goal));

    // Set up state and walk route.No longer have a goal system. Walk the route through the start.
    let cur = waypoints.get(&goal)?;
    let mut route = Vec::with_capacity(cur.dist as usize);
    let mut next_stop = cur.parent;
    route.push(cur.cur);
    while let Some(system_id) = next_stop {
        route.push(system_id);
        let cur = waypoints[&system_id].clone();
        next_stop = cur.parent;
    }

    // Route was walked in reverse order. Reverse and return
    // it.
    route.reverse();
    Some(route)
}

// Single-source shortest path via Breadth-First Search. Changed return type to tuple to get the end system --Brian Allen 
// Returns a waypoint map for further processing.
fn bfs_sec(map: &Map, start: SystemId, goal: R64)
            -> (HashMap<SystemId, Waypoint>, SystemId)
{
    // Set up data structures and run the search.
    let mut q = VecDeque::with_capacity(map.systems_ref().len());
    let mut closed = HashMap::new();
    q.push_back(Waypoint::new(0, start, None, r64(0.0)));
    loop {
        // Examine best waypoint.
        let waypoint = match q.pop_front() {
            Some(waypoint) => waypoint,
            None => return (closed, start),
        };
        if closed.contains_key(&waypoint.cur) {
            continue;
        }
        closed.insert(waypoint.cur, waypoint.clone());

        // If we have found the goal, we are done.
        //Changed for finding highsec --Brian ALlen 
        if  waypoint.sec == goal || waypoint.sec > goal {
            println!("sec end point is {:?}", waypoint.cur);
            //Not sure why, but had to return the parent waypoint or route would be one too long -- Brian Allen 
            return (closed, waypoint.parent.unwrap());
        }

        // Open the children of the current system.
        //Add security value to children --Brian Allen 
        let map_info = map.by_system_id(waypoint.cur);
        for child in map_info.stargates.iter() {
            let child_waypoint = Waypoint::new(
                waypoint.dist + 1,
                *child,
                Some(waypoint.cur),
                map_info.sec_status,
            );
            q.push_back(child_waypoint);
        }
    }
}

/// Return a shortest route if one exists. 
//Modified function to find high sec. Had to do goofy things with tuples to make it work --Brian Allen 
pub fn shortest_route_sec(map: &Map, start: SystemId, goal: R64)
                      -> Option<Vec<SystemId>>
{
    // Find single-source shortest paths from start up to goal.
    let waypoints = bfs_sec(map, start, goal);

    // Set up state and walk route.
    let cur = waypoints.0.get(&waypoints.1)?;
    let mut route = Vec::with_capacity(cur.dist as usize);
    let mut next_stop = cur.parent;
    route.push(cur.cur);
    while let Some(system_id) = next_stop {
        route.push(system_id);
        let cur = waypoints.0[&system_id].clone();
        next_stop = cur.parent;
    }

    // Route was walked in reverse order. Reverse and return it.
    route.reverse();
    Some(route)
}

// Single-source shortest path via Breadth-First Search. Hardcoded to find Major hubs. Need a flag system for searches..
// Returns a waypoint map for further processing.
fn bfs_hub_major(map: &Map, start: SystemId)
            -> (HashMap<SystemId, Waypoint>, SystemId)
{
    // Set up data structures and run the search.
    let mut q = VecDeque::with_capacity(map.systems_ref().len());
    let mut closed = HashMap::new();
    q.push_back(Waypoint::new(0, start, None, r64(0.0)));
    loop {
        // Examine best waypoint.
        let waypoint = match q.pop_front() {
            Some(waypoint) => waypoint,
            None => return (closed, start),
        };
        if closed.contains_key(&waypoint.cur) {
            continue;
        }
        closed.insert(waypoint.cur, waypoint.clone());

        // If we have found the goal, we are done.
        if  waypoint.cur.0 == 30000142 || waypoint.cur.0 == 30002187 || waypoint.cur.0 == 30002510 || 
                waypoint.cur.0 == 30002659 || waypoint.cur.0 == 30002053 {
            return (closed, waypoint.cur);
        }

        // Open the children of the current system.
        let map_info = map.by_system_id(waypoint.cur);
        for child in map_info.stargates.iter() {
            let child_waypoint = Waypoint::new(
                waypoint.dist + 1,
                *child,
                Some(waypoint.cur),
                map_info.sec_status,
            );
            q.push_back(child_waypoint);
        }
    }
}

/// Return a shortest route if one exists. Modified function to find Major trade hub
pub fn shortest_route_hub_major(map: &Map, start: SystemId)
                      -> Option<Vec<SystemId>>
{
    // Find single-source shortest paths from start up to goal.
    let waypoints = bfs_hub_major(map, start);

    // Set up state and walk route.
    let cur = waypoints.0.get(&waypoints.1)?;
    let mut route = Vec::with_capacity(cur.dist as usize);
    let mut next_stop = cur.parent;
    route.push(cur.cur);
    while let Some(system_id) = next_stop {
        route.push(system_id);
        let cur = waypoints.0[&system_id].clone();
        next_stop = cur.parent;
    }

    // Route was walked in reverse order. Reverse and return
    // it.
    route.reverse();
    Some(route)
}

// Single-source shortest path via Breadth-First Search. Hardcoded values to find minor hubs
// Returns a waypoint map for further processing.
fn bfs_hub_minor(map: &Map, start: SystemId)
            -> (HashMap<SystemId, Waypoint>, SystemId)
{
    // Set up data structures and run the search.
    let mut q = VecDeque::with_capacity(map.systems_ref().len());
    let mut closed = HashMap::new();
    q.push_back(Waypoint::new(0, start, None, r64(0.0)));
    loop {
        // Examine best waypoint.
        let waypoint = match q.pop_front() {
            Some(waypoint) => waypoint,
            None => return (closed, start),
        };
        if closed.contains_key(&waypoint.cur) {
            continue;
        }
        closed.insert(waypoint.cur, waypoint.clone());

        // If we have found the goal, we are done.
        if  waypoint.cur.0 == 30004969 || waypoint.cur.0 == 30001671 || waypoint.cur.0 == 30003862 {
            return (closed, waypoint.cur);
        }

        // Open the children of the current system.
        let map_info = map.by_system_id(waypoint.cur);
        for child in map_info.stargates.iter() {
            let child_waypoint = Waypoint::new(
                waypoint.dist + 1,
                *child,
                Some(waypoint.cur),
                map_info.sec_status,
            );
            q.push_back(child_waypoint);
        }
    }
}

/// Return a shortest route if one exists. Modified function to find minor trade hubs.
pub fn shortest_route_hub_minor(map: &Map, start: SystemId)
                      -> Option<Vec<SystemId>>
{
    // Find single-source shortest paths from start up to goal.
    let waypoints = bfs_hub_minor(map, start);

    // Set up state and walk route.
    let cur = waypoints.0.get(&waypoints.1)?;
    let mut route = Vec::with_capacity(cur.dist as usize);
    let mut next_stop = cur.parent;
    route.push(cur.cur);
    while let Some(system_id) = next_stop {
        route.push(system_id);
        let cur = waypoints.0[&system_id].clone();
        next_stop = cur.parent;
    }

    // Route was walked in reverse order. Reverse and return
    // it.
    route.reverse();
    Some(route)
}

/// Compute the diameter of New Eden, with other interesting
/// info.
pub fn diameter(map: &Map) -> DiameterInfo {
    // Collect needed info.
    let systems = map.systems_ref();
    let hops = apsp(map);
    let n = systems.len();

    // Reconstruct max diameter and incrementally update
    // max endpoints.
    let mut diameter = 0;
    let mut longest = Vec::new();
    for i in 0..n {
        for j in i+1..n {
            if let Some(hop) = hops[[i, j]] {
                let dist = hop.dist;
                if dist > diameter {
                    diameter = dist;
                    longest.clear();
                }
                if dist == diameter {
                    let iid = systems[i].system_id;
                    let jid = systems[j].system_id;
                    longest.push((iid, jid));
                }
            }
        }
    }

    // Return the accumulated info.
    DiameterInfo{diameter, longest}
}

/// Compute an all-pairs shortest-path route table.
pub fn apsp(map: &Map) -> APSPTable {
    // Set up necessary info.
    let systems = map.systems_ref();
    let n = systems.len();
    let mut hops: Array2<Option<Hop>> =
        Array2::from_elem((n, n), None);

    // Use iterated single-source shortest-path search to update
    // all hops.
    for start in systems {
        let j = start.system_index;
        let routes = bfs(map, start.system_id, None);
        for waypoint in routes.values() {
            if waypoint.parent.is_none() {
                continue;
            }
            let cur = map.by_system_id(waypoint.cur);
            let i = cur.system_index;
            match hops[[i, j]] {
                Some(hop) if hop.dist <= waypoint.dist => continue,
                _ => hops[[i, j]] = Some( Hop{
                    system_id: waypoint.parent.expect("apsp: walked off map"),
                    dist: waypoint.dist,
                }),
            }
        }
    }

    // Return the constructed table.
    hops
}
