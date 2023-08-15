import redis
import json
import os

r = redis.Redis(host=os.environ["REDIS_HOST"], port=6379, decode_responses=True)

f = open('Cocktails.json')

cocktails = json.load(f)

for cocktail in cocktails:
    cocktail2 = cocktail.lower()
    print(cocktail2)
    r.hmset(cocktail2, {"glass": cocktails[cocktail]["glass"], "ingredients": json.dumps(cocktails[cocktail]["ingredients"]), "preparation": cocktails[cocktail]["preparation"]})

g = open('Ingredients.json')

ingredients = json.load(g)

for ingredient in ingredients:
    ingredient2 = ingredient.lower()
    print(ingredient2)
    r.hset("INGREDIENTS", ingredient2, json.dumps(ingredients[ingredient]))