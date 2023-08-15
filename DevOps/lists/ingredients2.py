import json

with open('../db/cocktails2.json', 'r') as json_file:
    data = json.load(json_file)

ingredients_dic = {}

for cocktail in data:
    for ingredient in data[cocktail]["ingredients"]:
        ingredients_dic[ingredient] = []
for cocktail in data:
    for ingredient in data[cocktail]["ingredients"]:
        ingredients_dic[ingredient].append(cocktail)

with open("../db/ingredients.json", "w") as save_here:
    save_here.write(json.dumps(ingredients_dic))