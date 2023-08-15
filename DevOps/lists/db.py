import json

with open('cocktails.json', 'r') as json_file:
    data = json.load(json_file)

dic_drinks = {}
dic_ing = {}

for drink in data:
    dri = []
    for drin in drink["ingredients"]:
        try:
            dri = dri + [drin["ingredient"].lower()]
        except:
            Warning
    dr = dri
    try:
        drink["colors"] = drink["colors"].split(",")
    except:
        drink["colors"] = drink["colors"]
    try:
        dic_drinks[drink["name"].lower()] = {"colors": drink["colors"], "glass":drink["glass"], "ingredients": dr, "preparation": drink["preparation"]}
    except:
        dic_drinks[drink["name"].lower()] = {"colors": drink["colors"], "glass":drink["glass"], "ingredients": dr, "preparation": "No preparation!"}
save_file = open("../db/cocktails2.json", "w")  
json.dump(dic_drinks, save_file, indent = 2)
