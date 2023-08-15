import redis
import json
import os

r = redis.Redis(host=os.environ["REDIS_HOST"], port=6379, decode_responses=True)

f = open('cocktails.json')

cocktails = json.load(f)

for cocktail in cocktails:
    print(cocktail)
    cocktail["name"]=cocktail["name"].lower()
    try:
        r.hmset(cocktail["name"], {"colors": json.dumps(cocktail["colors"]), "glass": cocktail["glass"], "category": cocktail["category"], "ingredients": json.dumps(cocktail["ingredients"]), "garnish": cocktail["garnish"], "preparation": cocktail["preparation"]})
    except:
        try:
            r.hmset(cocktail["name"], {"colors": json.dumps(cocktail["colors"]), "glass": cocktail["glass"], "category": cocktail["category"], "ingredients": json.dumps(cocktail["ingredients"]), "garnish": "", "preparation": cocktail["preparation"]})
        except:
            try:
                r.hmset(cocktail["name"], {"colors": json.dumps(cocktail["colors"]), "glass": cocktail["glass"], "category": cocktail["category"], "ingredients": json.dumps(cocktail["ingredients"]), "garnish": cocktail["garnish"], "preparation": cocktail["preparation"]})
            except:
                try:
                    r.hmset(cocktail["name"], {"colors": json.dumps(cocktail["colors"]), "glass": cocktail["glass"], "category": "", "ingredients": json.dumps(cocktail["ingredients"]), "garnish": "", "preparation": cocktail["preparation"]})
                except:
                    try:
                        r.hmset(cocktail["name"], {"colors": json.dumps(cocktail["colors"]), "glass": cocktail["glass"], "category": cocktail["category"], "ingredients": json.dumps(cocktail["ingredients"]), "garnish": cocktail["garnish"], "preparation": ""})
                    except:
                        r.hmset(cocktail["name"], {"colors": json.dumps(cocktail["colors"]), "glass": cocktail["glass"], "category": cocktail["category"], "ingredients": json.dumps(cocktail["ingredients"]), "garnish": "", "preparation": ""})
                


g = open('Ingredients.json')

ingredients = json.load(g)

for ingredient in ingredients:
    ingredient2 = ingredient.lower()
    print(ingredient2)
    r.hset("INGREDIENTS", ingredient2, json.dumps(ingredients[ingredient]))