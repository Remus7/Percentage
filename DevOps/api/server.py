from flask import Flask, jsonify
import redis
import os
import json

app = Flask(__name__)

r = redis.Redis(host=os.environ['REDIS_HOST'], port=6379, decode_responses=True)

@app.route("/get_drinks/<ingredient>", methods=["GET"])
def get_drinks(ingredient):
    ingredient = ingredient.lower()
    dic_drinks = {}
    drinks = r.hget("INGREDIENTS", ingredient)
    if drinks != None:
        drinks = json.loads(drinks)
    else:
        return jsonify({"error":"no drinks found"})
    for drink in drinks:
        drink = drink.lower()
        colors = r.hget(drink, "colors")
        glass = r.hget(drink, "glass")
        ingredients = json.loads(r.hget(drink, "ingredients"))
        preparation = r.hget(drink, "preparation")
        dic_drinks[drink] = {"colors": colors, "glass":glass, "ingredients":ingredients, "preparation":preparation}
    return jsonify(dic_drinks)

@app.route("/get_ingredients/<drink>", methods=["GET"])
def get_drink(drink):
    drink = drink.lower()
    colors = r.hget(drink, "colors")
    glass = r.hget(drink, "glass")
    ingredients = json.loads(r.hget(drink, "ingredients"))
    preparation = r.hget(drink, "preparation")
    dic_drink = {drink:{"colors": colors,"glass":glass, "ingredients":ingredients, "preparation":preparation}}
    return jsonify(dic_drink)

if __name__ == "__main__":
    app.run(host="0.0.0.0", port=5000, debug=True)