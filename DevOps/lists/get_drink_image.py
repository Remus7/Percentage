import requests
import json
from time import sleep
def get_image(search_term):
    access_key = '_uR4KUdc76jVMmG5oTct3ghQEBsCKubvBEUZQGHoHFs'
    url = f'https://api.unsplash.com/search/photos/?query="{search_term} drink"&client_id={access_key}'

    response = requests.get(url)
    print(response.status_code, url)
    data = response.json()

    if 'results' in data and len(data['results']) > 0:
        width = 500
        height = 500
        first_image_url = data['results'][0]['urls']['regular'].split("?")[0]+f"?w={width}&h={height}&fit=crop"
        return first_image_url
    else:
        print("No images found.")
with open("../db/cocktails3.json", "r") as read_from_here:
    dictionary = json.load(read_from_here)
i=0
for cocktail in dictionary:
    i+=1
    
    if i > 40: 
        dictionary[cocktail]["image"] = get_image(cocktail)
        with open("../db/cocktails3.json", "w") as write_in_here:
            write_in_here.write(json.dumps(dictionary))
        


