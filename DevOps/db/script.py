import redis
import json
import os

r = redis.StrictRedis(host=os.environ["REDIS_HOST"], port=6379, password="Parola", decode_responses=True)

f = open('cocktails3.json')

cocktails = json.load(f)

for cocktail in cocktails:
    cocktail2 = cocktail.lower()
    r.hmset(cocktail2, {"colors": json.dumps(cocktails[cocktail]["colors"]), "glass": cocktails[cocktail]["glass"], "ingredients": json.dumps(cocktails[cocktail]["ingredients"])})
    r.hmset(cocktail2,{ "preparation": cocktails[cocktail]["preparation"], "image":cocktails[cocktail]["image"]})
g = open('ingredients.json')
ingredients = json.load(g)

for ingredient in ingredients:
    ingredient2 = ingredient.lower()
    r.hset("INGREDIENTS", ingredient2, json.dumps(ingredients[ingredient]))