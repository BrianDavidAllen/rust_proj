# **Rust_proj: Plan-b extended **
Copyright © 2018 Po Huit</br>
Copyright © 2018 EVE-escapes</br> 
Copyright © 2018 Brian Allen </br>

## Authors:
#### Rust_proj: Plan-B Extended</br>
*Brian Allen: briallen@pdx.edu* 
#### Plan B: https://github.com/PoHuit/plan-b </br>
*Po Huit: eve.po.huit@gmail.com* 
#### Eve Escapes:  https://github.com/israelbond/EVE-escapes </br>
*Brian Allen: briallen@pdx.edu* </br>
*Gavin Balakrishnan: gavin4@pdx.edu* </br>
*Israel Bond: ibond@pdx.edu* </br>
*Samuel Strba: sstrba@pdx.edu* </br>
### Built under a MIT General Public License 
To be *very* clear, the only code I wrote for this project will be found within:
* The web folder within Plan B (Where I did most of the work)
* Modified a few several functions within cmdlin main.rs
* Modified several sections within plan_b search.rs and map.rs
* Slight modification to updateDB.py (I believe it was using collecting NPC structure kills. 
* Left original plan-b README.md with changes I made in italics in the plan-b folder

*Changes (I believe all) have been marked by comments with my name as a flag --Brian Allen*

Software contributions
__Plan B__
* This entire project is made possible by __Po Huit: eve.po.huit@gmail.com__. The original plan was to extend Po's shortest route finder to find multiple routes. Unfortunately, I was not able to extend this functionality. Nor was I able to get the original web frame working (Rust's nightly, pear, and rocket were jerks) so I replaced the web framework-- sorry Po. 
* Added functionality to find the shortest route from a given system to a high security system
* Added functionality to find the shortest route from a given system to a major trade hub (Jita, Amarr, etc)
* Added functionality to find the shortest route from a given system to a minor trade hub (Oursulaert, Tash-Murkon Prime, and Agil)
* Added rust test for each new funcionallity 
__Eve-Escapes__ 
* Used systems database and scirpt used to scrape kills from zkillboard to keep database current. updateDB.py
__Rust_Proj:Plan-B Extended__ 
* Web framework with: Iron and Mime (The basis for the framework was taken from [Programming Rust: Fast, Safe Systems Development](https://www.google.com/shopping/product/17725533124351171698?q=rust+programming+book&biw=1440&bih=780&prds=paur:ClkAsKraX5c9ja952G6dUlE3TbQWBOrhOYbxjvVZxJ9qvTL3LC5GTtBNy7uLFd2pUrmHAztusuCjZblTrVsbygVdpWRu31KZ42DMWFTOQ18Zuf31W8weSjr9hRIZAFPVH72aIwskYbjY1ngRwFYmUK61SCFA2A&sa=X&ved=0ahUKEwj49umHm_LcAhXKyVMKHTEHDzIQ8wII1AI) pg 38-45.
* Web interface will output a form on a get request to choose the type of route.
* On post, web interface will output the shortest path of type chose in a HTML table.
   * Table will contain names of systems to travel in order, along with the kills of each system. Each system in the table is linked to its corresponding Dotlan page. 
* Web interface will find the shortest route from system to system, system to high security, system to major trade hub, and system to minor trade hub. For inputs use systems names. Ex. Amarr.

## Still in Progess:
* There is quite a bit of reused code in the project. Starting cleaning it up with match statements and but broke it. I will get to it sometime before fall.
* Creating a flag system for breadth first search functions to condense the amount of functions throughout search.js
* Need to find a crate that I can manage the HTML page on the post and get request. In lining the HTML and JS as I did is not ideal.
* Po, if you want to enable multiple routes sometime in October, let me know and I will make time. 

## Build and Install
**Rust_proj:  Plan-B Extended, has only been tested in on a linux Debian enviroment**
* Once the repository is downloaded, make sure you've installed sqlite3, python3, and Rust.
* run the updateDB.py (inside the kill_data folder) by using "python3 updateDB.py" to update the databse. Script will continually run and print to screen when data is updated. You will need to open another window to run the web service or cmdline.
* While updateDB.py is running, do a cargo run --release within the web folder. A message will print with the local host address. Follow the prompts on the web-page to find the route desired. 
* Please see the Plan-B README.md for more information on how to use the cmdline version.

## Current Issues
* Need to wait about three minutes after starting up updateDB.py to finishing load the data dump from CCP or you will get a database is locked error.
* Haven't had the time to test if all the kill data presented from the web service is accurate. I beleive the fix I made the other night will take care of it. 

## Things I learned
* I learned that rust doesn't want to Ord or Eq floats **thanks noisy_float crate**
* Learned how to implment a BFS search via Po's code.
* Learned much quit a bit on how to use the Iron frameworks
* Learned how to attach links with javascript (took much longer than I'd like to admit).
* Learned how much faster a rust runs not in debug mode. 
* Finally learned how to deal with rust Strings
* I'm sure more was learned but I can't think of it now. 
* Rust is fickle but fun and safe!

## Acknowledgments
* Big thanks to Po Huit and his github for putting together the meat of this project. The search functionality would not have been possible without plan-b. https://github.com/PoHuit/plan-b
* Thanks to the guys over at Eve-escapes. Glad I had the chance to reuse what we made last term. https://github.com/israelbond/EVE-escapes 
* Thank you Programming Rust: Fast, Safe Systems Development for teaching me about this fickle yet lovable language. Particularity the web service help. 
* Thank you W3Schools for making HTML and JS the amazing tutorials 
* Thanks to the https://github.com/zKillboard/RedisQ for putting together the website for getting zkill data.
* Thank you Dotlan.com for providing such great system data.
* Thank you stack overflow for the help with using the rust rand crate.
* And, Finally, thank you to all the developers of the oodles and oodles of rust crates used in this project (You’ll see when you compile.)
<!--stackedit_data:
eyJoaXN0b3J5IjpbMTU3ODcxOTMzMF19
-->

