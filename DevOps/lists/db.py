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
            print(dri)
    dr = dri
    try:
        dic_drinks[drink["name"].lower()] = {"colors": drink["colors"], "glass":drink["glass"], "ingredients": dr, "preparation": drink["preparation"]}
    except:
        dic_drinks[drink["name"].lower()] = {"colors": drink["colors"], "glass":drink["glass"], "ingredients": dr, "preparation": "No preparation!"}

save_file = open("Cocktails2.json", "w")  
json.dump(dic_drinks, save_file, indent = 2)
