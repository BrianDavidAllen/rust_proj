extern crate iron;
extern crate router;
extern crate sqlite3;
extern crate urlencoded;

use router::Router;
use sqlite3::State;
use std::str::FromStr;
use urlencoded::UrlEncodedBody;

#[macro_use]
extern crate mime;
use iron::prelude::*;
use iron::status;

//The base of this web service comes from Programming Rust:Fast, Safe Systems Develpment pg 38-45
fn main() {
    let mut router = Router::new();

    router.get("/", get_form, "root");
    router.post("/gcd", post_gcd, "gcd");

    println!("Serving	on	http://localhost:3000...");
    Iron::new(router).http("localhost:3001").unwrap();
}

fn get_form(_request: &mut Request) -> IronResult<Response> {
    let mut response = Response::new();

    response.set_mut(status::Ok);
    response.set_mut(mime!(Text/Html; Charset=Utf8));
    response.set_mut(r#"
	<title>Eve Plan B Route Planner</title>
	<form action="/gcd"	method="post">
		From:<input	type="text"	name="n"/>
		<select name="high_sec">
			<option value="to_high_sec">To high sec</option>
			<option value="to_system">To system</option>
		</select>	
		<input type="text" name="n"/>
		<br></br>
		Number of paths:<input type = "text" name="num_paths"/>
		<br></br>
		Max Jumps: <input type ="text" name="jumps"/>
		
		<button	type="submit">Compute GCD</button>
	</form>
	"#);

    Ok(response)
}

fn post_gcd(request: &mut Request) -> IronResult<Response> {
    let mut response = Response::new();

    let form_data = match request.get_ref::<UrlEncodedBody>() {
        Err(e) => {
            response.set_mut(status::BadRequest);
            response.set_mut(format!("Error parsing form data: {:?}\n", e));
            return Ok(response);
        }
        Ok(map) => map,
    };

    let unparsed_number = match form_data.get("n") {
        None => {
            response.set_mut(status::BadRequest);
            response.set_mut(format!("Form data has no 'n' parameters\n"));
            return Ok(response);
        }
        Some(nums) => nums,
    };

    let mut numbers = Vec::new();
    for unparsed in unparsed_number {
        match u64::from_str(&unparsed) {
            Err(_) => {
                response.set_mut(status::BadRequest);
                response.set_mut(format!(
                    "Value for 'n' parameter not a number: {:?}\n",
                    unparsed
                ));
                return Ok(response);
            }
            Ok(n) => {numbers.push(n);}
        }
    }

    let mut d = numbers[0];
    for m in &numbers[1..] {
        d = gcd(d, *m);
    }

    response.set_mut(status::Ok);
    response.set_mut(mime!(Text/Html; Charset=Utf8));
    response.set_mut(format!("From {:?} to {}\n", numbers, d));
    Ok(response)
}
fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t
        }
        m %= n
    }
    n
}

fn get_kills_by_name(a_system: &str, mut systems: Vec<String>) {
    //open connection to eve.db
	let connection = sqlite3::open("./eve_system.db").unwrap();
    //use a_system to select system from eve.db
	let mut statement = connection.prepare("Select * FROM systems WHERE system_id = 30002187").unwrap();
	
	while let State::Row = statement.next().unwrap() {
		println!("system_id = {}", statement.read::<String>(0).unwrap());
		println!("system_name = {}", statement.read::<String>(1).unwrap());
		println!("security_status = {}", statement.read::<String>(2).unwrap());
		println!("kills = {}", statement.read::<String>(3).unwrap());
		println!("connections = {}", statement.read::<String>(4).unwrap());

	}
	//add system info to vec
	//return vec
}


fn get_kills_by_id(a_system: usize, mut systems: Vec<String>) {
    //open connection to eve.db
    //use a_system to select system from eve.dc
	//add system info to vec
	//return vec
}

