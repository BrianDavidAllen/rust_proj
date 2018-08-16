# Copyright © 2018 EVE-escapes
# [This program is licensed under the "MIT License"]
# Please see the file LICENSE in the source
# distribution of this software for license terms.

import json
import sqlite3
import urllib.request
import time
from datetime import datetime
from datetime import timedelta

'''
updates sqlite database with kill data from CCP and Zkillboard via reisq.zkillboard
credit for zkillboard data goes redisQ https://github.com/zKillboard/RedisQ
If you run into issues with the sqlite3 database being locked use command fuser eve_system.db to see what process you need to kill. 
'''

#set up refresh time for CCP data. Their severs only update once an hour
update_time = datetime.now() + timedelta(hours = 1)

with urllib.request.urlopen('https://esi.tech.ccp.is/latest/universe/system_kills/') as h1:
	universe_kills = json.loads(h1.read().decode('utf-8'))

print("Info from CCP")
#connect to local .db file
db = sqlite3.connect('eve_system.db')
c = db.cursor()

#updates kills 
for system in universe_kills:
	c.execute('''UPDATE systems SET kills = ? WHERE system_id = ?''', (system['ship_kills'], system['system_id']))                        
	db.commit()

print("ccp update")
#run continuously 
while True: 
#Check to see if it's time to update
	if datetime.now() >= update_time:
		update_time = datetime.now() + timedelta(hours = 1)
		print("update timer reset")
	
#open connection to CCP to get kill data
		with urllib.request.urlopen('https://esi.tech.ccp.is/latest/universe/system_kills/') as h1:
			universe_kills = json.loads(h1.read().decode('utf-8'))

#updates kills from CCP
		for system in universe_kills:
			c.execute('''UPDATE systems SET kills = ? WHERE system_id = ?''', (system['ship_kills'], system['system_id']))                        
			db.commit()

#update kills from Zkillboard data via redisq
	else:
		webUrl = urllib.request.urlopen('https://redisq.zkillboard.com/listen.php')
		data = webUrl.read() 
		jData = json.loads(data.decode('utf-8'))
		if jData['package'] is None:
			time.sleep(60)
			print("waiting for new data..")
		try :
			jData['package']['killmail']['victim']['character_id']
			c.execute('''UPDATE systems SET kills = kills + ? WHERE system_id = ?''', ( 1, jData['package']['killmail']['solar_system_id']))
			db.commit()
			print("zKill")
		except:
			print("Zkill data with no victime character ID")
			continue
