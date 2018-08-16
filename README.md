# **Rust_proj: Plan-b extended **
Copyright © 2018 Po Huit
Copyright © 2018 EVE-escapes 
Copyright © 2018 Brian Allen 

## Authors:
#### Rust_proj:</br>
*Brian Allen: briallen@pdx.edu* 
####  Plan B:</br>
*Po Huit: eve.po.huit@gmail.com*
#### Eve Escapes:</br>
*Brian Allen: briallen@pdx.edu*
*Gavin Balakrishnan: gavin4@pdx.edu*  
*Israel Bond: ibond@pdx.edu*  
*Samuel Strba: sstrba@pdx.edu*
### Built under a MIT General Public License      
To be *very* clear, the only code I wrote for this project will be found within:
* The web folder within Plan B (Where I did most of the work)
*  Modified a few several functions within cmdlin main.rs
*  Modified several sections within plan_b search.rs and map.rs
* Slight modification to updateDB.py (I believe it was using collecting NPC structure kills. 
* Left orginal plan-b README.md with updates in italics in the plan-b folder

*Changes (I believe all)  have been marked by comments with my name as a flag --Brian Allen*


Software contributions
* __Plan B__
	* This entire project is made possible by __Po Huit: eve.po.huit@gmail.com__. The original plan was to extend Po's shortest route finder to find multiple routes. Unfortunately, I was not able to extend this functionality. Nor was I able to get the original web frame working (Rust's nightly, pear, and rocket were jerks) so I replaced the web framework-- sorry Po. 
* __Eve-Escapes__ 
	* Used systems database and scirpt used to scrape kills from zkillboard to keep database current.  
* Web framework with: Iron and Mime (The basis for the framework was taken from [Programming Rust: Fast, Safe Systems Development](https://www.google.com/shopping/product/17725533124351171698?q=rust+programming+book&biw=1440&bih=780&prds=paur:ClkAsKraX5c9ja952G6dUlE3TbQWBOrhOYbxjvVZxJ9qvTL3LC5GTtBNy7uLFd2pUrmHAztusuCjZblTrVsbygVdpWRu31KZ42DMWFTOQ18Zuf31W8weSjr9hRIZAFPVH72aIwskYbjY1ngRwFYmUK61SCFA2A&sa=X&ved=0ahUKEwj49umHm_LcAhXKyVMKHTEHDzIQ8wII1AI) pg 38-45.
  * Web interface will output a form on a get request to choose the type of route.
  * On post, web interface will output the shortest path of type chose in a html table.
	  * Table will contain names of systems to travel in order along with the kills of each system. Each system in the table is linked to its corresponding Dotlan page. 
  * Web interface will find the shortest route from system to system, system to high security, system to major trade hub, and system to minor trade hub. For inputs use systems names. Ex. Amarr. 
     
## Build and install
* Once the repository is downloaded, make sure you've installed sqlite3, python3, and Rust.
* run the updateDB.py by using "python3 updateDB.py" to update the databse. Script will continually run and print to screen when data is updated. 
* While updateDB.py is running, run "python3 path_generator.py 'sys_from' sys_to'". 
* Ex "python3 path_generator.py Amarr Jita" will return the shortest path with no kills from Amarr to Jita and print them to screen. 
* The independent .html files can be run locally produce a web page--unfortunatley we were not able to get a web service up and running. 


The files used in this project are Python3.6 with Networkx library, CSS, HTML, JavaScript, while using JSON files for formatted input.

## Acknowledgments

* Shout out to po huit and his github for putting together a json dump of the eve universe. https://github.com/PoHuit/plan-b
* Shout out to Networkx for putting having such an easy to use graph and algorithms. https://networkx.github.io/ 
* A final thanks to the https://github.com/zKillboard/RedisQ for putting together the website for getting zkill data.
<!--stackedit_data:
eyJoaXN0b3J5IjpbMTU3ODcxOTMzMF19
-->