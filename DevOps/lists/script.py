import redis
import json
import os

r = redis.Redis(host=os.environ["REDIS_HOST"], port=6379, decode_responses=True)

f = open('Cocktails.json')

cocktails = json.load(f)

for cocktail in cocktails:
    print(cocktail)
    r.hmset(cocktail, {"glass": cocktails[cocktail]["glass"], "ingredients": json.dumps(cocktails[cocktail]["ingredients"]), "preparation": cocktails[cocktail]["preparation"]})

g = open('Ingredients.json')

ingredients = json.load(g)

for ingredient in ingredients:
    ingredient = ingredient.lower()
    r.hset("INGREDIENTS", ingredient, json.dumps(ingredients[ingredient]))