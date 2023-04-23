# Note: Python 3.9+ code compatibility
# import HTTP fetching library
import requests
# declare API URL as constant
API_URL = "https://statsapi.web.nhl.com/api/v1"
# fetch player stats for Ovechkin, ask server to serve in
# JSON format
response = requests.get(API_URL + "/people/8471214/stats?stats=gameLog", params={"Content-Type": "application/json"})
data = response.json()
