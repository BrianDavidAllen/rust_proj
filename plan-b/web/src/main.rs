
extern crate serde;
extern crate serde_json;

extern crate iron;
use urlencoded::UrlEncodedBody;

extern crate router;
use router::Router;

extern crate sqlite3;
use sqlite3::State;

extern crate urlencoded;

extern crate plan_b;
use plan_b::map::*;
use plan_b::search::*;

extern crate noisy_float;
use noisy_float::prelude::*;

#[macro_use]
extern crate mime;
use iron::prelude::*;
use iron::status;

#[macro_use]
extern crate serde_derive;


#[derive(Clone, Debug, Serialize, Deserialize)]
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
    let mut systems_data = Vec::<System_data>::new();
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

fn get_route(_request: &mut Request) -> IronResult<Response> {
    let mut response = Response::new();

    response.set_mut(status::Ok);
    response.set_mut(mime!(Text/Html; Charset=Utf8));
    //set get data to be simple form --Brian Allen 
    response.set_mut(r#"
	<title>Eve Plan B Route Planner</title>
	<form action="/find_route"	method="post">
		From:<input	type="text"	name="systems"/>
		<select name="route_finder">
			<option value="system">To system</option>
            <option value="high_sec">To high sec</option>
            <option value="trade_hub">To major trade hub</option>
            <option value="trade_hub_minor">To minor trade hub</option>
		</select>	
		<input type="text" name="systems"/>
		<br></br>
		Number of paths:<input type = "text" value = "Not implemented yet" name="num_paths"/>
		<br></br>
		Max Jumps: <input type ="text" value ="Not implemented yet" name="jumps"/>
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
    //fetch map before calling route or route_sec 
    let map = Map::fetch().expect("could not open map");
    let mut systems = Vec::new();
    for system in input_systems {
        systems.push(system.to_string());
    }

    //Check to see where the route is to
    let route_finder = form_data.get("route_finder").unwrap();
    if route_finder[0] == "high_sec".to_string() {
        //call the search function for high_sec
        let route_sec = find_route_sec(&map, &systems[0], r64(0.5));
        let route_with_kills = get_kills_by_route(route_sec);
        let route_with_kills_json = serde_json::to_string(&route_with_kills).unwrap();
        //place holder till high_sec function is done
        response.set_mut(status::Ok);
        response.set_mut(mime!(Text/Html; Charset=Utf8));
        //All HTML and JavaScript made possible by W3schools
        //Set formated response. Parse route in HTML table --Brian Allen 
        response.set_mut(format!("Shortest path from {:?} to high sec is
                        <html>
                        <head>
                        <style>
                        table, td {{
                            border: 1px solid black;
                        }}
                        </style>
                        </head>
                        <body>
                        <table id=\"system_table\">
                        </table> 
                        <script>
                            var systems = {};
                            var table = document.getElementById(\"system_table\");
                            for (i in systems) {{
                                var row = table.insertRow(i);
                                var cell1 = row.insertCell(0);
                                var cell2 = row.insertCell(1);
                                var system_str = systems[i].system_name;
                                var system_result = system_str.link(\"http://evemaps.dotlan.net/system/\" + system_str);
                                cell1.innerHTML = system_result;
                                cell2.innerHTML = systems[i].system_kills;
                            }}
                            var table = document.getElementById(\"system_table\");
                            var header = table.createTHead();
                            var row = header.insertRow(0);
                            var cell = row.insertCell(0);
                            var row_2 = header.insertRow(1);
                            var cell_2 = row.insertCell(1);
                            cell.innerHTML = \"<b>System Name</b>\";
                            cell_2.innerHTML = \"<b>Recent Kills</b>\";
                        </script> 
                        <form action = \"/\" method = \"get\"><button type = \"submit\">I'll have another!</button></form>
                        </body>
                        </html>
                        ",systems[0], route_with_kills_json));
        return Ok(response)
    }
    if route_finder[0] == "trade_hub_minor".to_string(){
        //call the search function for trade hubs
        let route_hub_minor = find_route_hub_minor(&map, &systems[0]);
        let route_with_kills = get_kills_by_route(route_hub_minor);
        let route_with_kills_json = serde_json::to_string(&route_with_kills).unwrap();
        //place holder till high_sec function is done
        response.set_mut(status::Ok);
        response.set_mut(mime!(Text/Html; Charset=Utf8));
        //All HTML and JavaScript made possible by W3schools
        //Set formated response. Parse route in HTML table --Brian Allen 
        response.set_mut(format!("Shortest path from {:?} to high sec is
                        <html>
                        <head>
                        <style>
                        table, td {{
                            border: 1px solid black;
                        }}
                        </style>
                        </head>
                        <body>
                        <table id=\"system_table\">
                        </table> 
                        <script>
                            var systems = {};
                            var table = document.getElementById(\"system_table\");
                            for (i in systems) {{
                                var row = table.insertRow(i);
                                var cell1 = row.insertCell(0);
                                var cell2 = row.insertCell(1);
                                var system_str = systems[i].system_name;
                                var system_result = system_str.link(\"http://evemaps.dotlan.net/system/\" + system_str);
                                cell1.innerHTML = system_result;
                                cell2.innerHTML = systems[i].system_kills;
                            }}
                            var table = document.getElementById(\"system_table\");
                            var header = table.createTHead();
                            var row = header.insertRow(0);
                            var cell = row.insertCell(0);
                            var row_2 = header.insertRow(1);
                            var cell_2 = row.insertCell(1);
                            cell.innerHTML = \"<b>System Name</b>\";
                            cell_2.innerHTML = \"<b>Recent Kills</b>\";
                        </script> 
                        <form action = \"/\" method = \"get\"><button type = \"submit\">I'll have another!</button></form>
                        </body>
                        </html>
                        ",systems[0], route_with_kills_json));
        return Ok(response)
    }
 
    if route_finder[0] == "trade_hub".to_string(){
        //call the search function for trade hubs
        let route_hub_major = find_route_hub_major(&map, &systems[0]);
        let route_with_kills = get_kills_by_route(route_hub_major);
        let route_with_kills_json = serde_json::to_string(&route_with_kills).unwrap();
        //place holder till high_sec function is done
        response.set_mut(status::Ok);
        response.set_mut(mime!(Text/Html; Charset=Utf8));
        //All HTML and JavaScript made possible by W3schools
        //Set formated response. Parse route in HTML table --Brian Allen 
        response.set_mut(format!("Shortest path from {:?} to high sec is
                        <html>
                        <head>
                        <style>
                        table, td {{
                            border: 1px solid black;
                        }}
                        </style>
                        </head>
                        <body>
                        <table id=\"system_table\">
                        </table> 
                        <script>
                            var systems = {};
                            var table = document.getElementById(\"system_table\");
                            for (i in systems) {{
                                var row = table.insertRow(i);
                                var cell1 = row.insertCell(0);
                                var cell2 = row.insertCell(1);
                                var system_str = systems[i].system_name;
                                var system_result = system_str.link(\"http://evemaps.dotlan.net/system/\" + system_str);
                                cell1.innerHTML = system_result;
                                cell2.innerHTML = systems[i].system_kills;
                            }}
                            var table = document.getElementById(\"system_table\");
                            var header = table.createTHead();
                            var row = header.insertRow(0);
                            var cell = row.insertCell(0);
                            var row_2 = header.insertRow(1);
                            var cell_2 = row.insertCell(1);
                            cell.innerHTML = \"<b>System Name</b>\";
                            cell_2.innerHTML = \"<b>Recent Kills</b>\";
                        </script> 
                        <form action = \"/\" method = \"get\"><button type = \"submit\">I'll have another!</button></form>
                        </body>
                        </html>
                        ",systems[0], route_with_kills_json));
        return Ok(response)
    }

    //put systems as string into vector to call find_route --Brian Allen 
    let route = find_route(&map, &systems[0], &systems[1]);
    let route_with_kills = get_kills_by_route(route);
    let route_with_kills_json = serde_json::to_string(&route_with_kills).unwrap();
    
    //send back response --Brian Allen
    response.set_mut(status::Ok);
    response.set_mut(mime!(Text/Html; Charset=Utf8));
    response.set_mut(format!("Shortest path from {:?} to {:?} is
                        <html>
                        <head>
                        <style>
                        table, td {{
                            border: 1px solid black;
                        }}
                        </style>
                        </head>
                        <body>
                        <table id=\"system_table\">
                        </table> 
                        <script>
                            var systems = {};
                            var table = document.getElementById(\"system_table\");
                            for (i in systems) {{
                                var row = table.insertRow(i);
                                var cell1 = row.insertCell(0);
                                var cell2 = row.insertCell(1);
                                var system_str = systems[i].system_name;
                                var system_result = system_str.link(\"http://evemaps.dotlan.net/system/\" + system_str);
                                cell1.innerHTML = system_result;
                                cell2.innerHTML = systems[i].system_kills;
                            }}
                            var table = document.getElementById(\"system_table\");
                            var header = table.createTHead();
                            var row = header.insertRow(0);
                            var cell = row.insertCell(0);
                            var row_2 = header.insertRow(1);
                            var cell_2 = row.insertCell(1);
                            cell.innerHTML = \"<b>System Name</b>\";
                            cell_2.innerHTML = \"<b>Recent Kills</b>\";
                        </script> 
                        <form action = \"/\" method = \"get\"><button type = \"submit\">I'll have another!</button></form>
                        </body>
                        </html>", systems[0], systems[1], route_with_kills_json));
    Ok(response)
}

fn find_route(map: &Map, start: &str, goal: &str) -> Vec<SystemId> {
    let start_id = find_system(&map, start);
    let goal_id = find_system(&map, goal);
    shortest_route(&map, start_id, goal_id)
        .expect(&format!("no route found from {} to {}", start, goal))
}
//Modified function for finding shortest route to high sec --Brian Allen
fn find_route_sec(map: &Map, start: &str, goal_sec: R64) -> Vec<SystemId> {
    let start_id = find_system(&map, start);
    shortest_route_sec(&map, start_id, goal_sec)
        .expect(&format!("no route found from {} to high sec", start))
}
//Modified function for find shortest route to major trade hub --Brian Allen 
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

fn find_system(map: &Map, name: &str) -> SystemId {
    map.by_name(name)
        .expect(&format!("could not find {} in map", name))
        .system_id
}
//The base of this web service comes from Programming Rust:Fast, Safe Systems Develpment pg 38-45
fn main() {
    let mut router = Router::new();

    router.get("/", get_route, "root");
    router.post("/find_route", post_route, "gcd");

    println!("Serving	on	http://localhost:3005...");
    Iron::new(router).http("localhost:3005").unwrap();
}

