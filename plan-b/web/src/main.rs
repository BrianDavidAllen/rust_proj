extern crate iron;
extern crate router;
extern crate sqlite3;
extern crate urlencoded;
extern crate plan_b;

use plan_b::map::*;
use plan_b::search::*;
use router::Router;
use sqlite3::State;
use std::str::FromStr;
use urlencoded::UrlEncodedBody;

#[macro_use]
extern crate mime;
use iron::prelude::*;
use iron::status;

#[derive(Clone, Debug)]
pub struct System_data{
    pub system_name : String,
    pub system_kills : String,
}

impl System_data{
    pub fn new(name: String, kills: String)-> System_data{
        System_data{system_name: name, system_kills: kills}
    }
}
//Use database of current kills from Eve-escapes --Brian Allen 
fn get_kills_by_route(mut systems: Vec<SystemId>) -> Vec<System_data> {
    let mut systems_data = Vec::new();
    let mut a_system_data = System_data::new("Butt".to_string(),0.to_string());
    //open connection to eve.db
	let connection = sqlite3::open("../kill_data/eve_system.db").expect("Error opening data base");
    //use a_system to select system from eve.db
    for system in systems{
	    let mut statement = connection.prepare("Select * FROM systems WHERE system_id = ?").expect("error with statemtn");
        statement.bind(1, system.0 as i64).expect("error binding statement");
        while let State::Row = statement.next().unwrap() {
            a_system_data.system_name = statement.read::<String>(1).expect("Error getting system_name");
            a_system_data.system_kills = statement.read::<String>(3).expect("Error getting system_kills");
        }
        systems_data.push(a_system_data.clone());
    }
    systems_data
}

fn find_route(map: &Map, start: &str, goal: &str) -> Vec<SystemId> {
    let start_id = find_system(&map, start);
    let goal_id = find_system(&map, goal);
    shortest_route(&map, start_id, goal_id)
        .expect(&format!("no route found from {} to {}", start, goal))
}

fn find_route_sec(map: &Map, start: &str, goal_sec: i64) -> Vec<SystemId> {
    let start_id = find_system(&map, start);
    //let goal_id = find_system(&map, goal);
    shortest_route_sec(&map, start_id, goal_sec)
        .expect(&format!("no route found from {} to high sec", start))
}

fn find_system(map: &Map, name: &str) -> SystemId {
    map.by_name(name)
        .expect(&format!("could not find {} in map", name))
        .system_id
}

fn get_route(_request: &mut Request) -> IronResult<Response> {
    let mut response = Response::new();

    response.set_mut(status::Ok);
    response.set_mut(mime!(Text/Html; Charset=Utf8));
    response.set_mut(r#"
	<title>Eve Plan B Route Planner</title>
	<form action="/find_route"	method="post">
		From:<input	type="text"	name="systems"/>
		<select name="route_finder">
			<option value="system">To system</option>
            <option value="high_sec">To high sec</option>
            <option value="trade_hub">To trade hub</option>
		</select>	
		<input type="text" name="systems"/>
		<br></br>
		Number of paths:<input type = "text" name="num_paths"/>
		<br></br>
		Max Jumps: <input type ="text" name="jumps"/>
        </br>
		<button	type="submit">Find Route</button>
	</form>
	"#);

    Ok(response)
}

fn post_route(request: &mut Request) -> IronResult<Response> {
    let mut response = Response::new();

    let form_data = match request.get_ref::<UrlEncodedBody>() {
        Err(e) => {
            response.set_mut(status::BadRequest);
            response.set_mut(format!("Error parsing form data: {:?}\n", e));
            return Ok(response);
        }
        Ok(map) => map,
    };


    //Parse the To and From systems -- Brian Allen 
    let input_systems = match form_data.get("systems") {
        None => {
            response.set_mut(status::BadRequest);
            response.set_mut(format!("Form data has no from system!\n"));
            return Ok(response)
        }
        Some(from_sys) => from_sys,
    };

    //Check to see where the route is to
    let high_sec = form_data.get("route_finder").unwrap();
    if high_sec[0] == "high_sec".to_string() {
        //call the search function for high_sec
        //place holder till high_sec function is done
        response.set_mut(status::Ok);
        response.set_mut(mime!(Text/Html; Charset=Utf8));
        response.set_mut(format!("To high_sec is not implemented yet"));
        return Ok(response)
    }
 
    //put systems as string into vector to call find_route --Brian Allen 
    let mut systems = Vec::new();
    for system in input_systems {
        systems.push(system.to_string());
    }
    //open map and find route --Thank you Po Huit 
    let map = Map::fetch().expect("could not open map");
    let route = find_route(&map, &systems[0], &systems[1]);
    let route_with_kills = get_kills_by_route(route);
    
    //send back response 
    response.set_mut(status::Ok);
    response.set_mut(mime!(Text/Html; Charset=Utf8));
    //response.set_mut(r#"<form action = "/" method = "get"><button type = "submit">I'll have another!</button></form>"#);
    response.set_mut(format!("Shortest path from {:?} to {:?} is {:?}\n 
                            <form action = \"/\" method = \"get\"><button type = \"submit\">I'll have another!</button></form>
                            ", systems[0], systems[1], route_with_kills));
    Ok(response)
}
//The base of this web service comes from Programming Rust:Fast, Safe Systems Develpment pg 38-45
fn main() {
    let mut router = Router::new();

    router.get("/", get_route, "root");
    router.post("/find_route", post_route, "gcd");

    println!("Serving	on	http://localhost:3004...");
    Iron::new(router).http("localhost:3004").unwrap();
}

