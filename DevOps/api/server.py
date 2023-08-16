from flask import Flask, jsonify
import redis
import os
import json

app = Flask(__name__)

r = redis.StrictRedis(host=os.environ['REDIS_HOST'], port=6379,password="Parola", decode_responses=True)

@app.route("/get_drinks/<ingredient>", methods=["GET"])
def get_drinks_by_i(ingredient):
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
        image = r.hget(drink, "image")
        dic_drinks[drink] = {"colors": colors, "glass":glass, "ingredients":ingredients, "preparation":preparation, "image":image}
    return jsonify(dic_drinks)

@app.route("/get_ingredients/<drink>", methods=["GET"])
def get_ingredients_by_d(drink):
    drink = drink.lower()
    colors = r.hget(drink, "colors")
    glass = r.hget(drink, "glass")
    ingredients = json.loads(r.hget(drink, "ingredients"))
    preparation = r.hget(drink, "preparation")
    image = r.hget(drink, "image")
    dic_drink = {drink:{"colors": colors,"glass":glass, "ingredients":ingredients, "preparation":preparation, "image":image}}
    return jsonify(dic_drink)

@app.route("/get_drinks", methods=["GET"])
def get_drinks():
    list_drinks = []
    for key in r.keys("*"):
        if key!= "INGREDIENTS":
            drink = key.lower()
            colors = json.loads(r.hget(drink, "colors"))
            glass = r.hget(drink, "glass")
            ingredients = json.loads(r.hget(drink, "ingredients"))
            preparation = r.hget(drink, "preparation")
            image = r.hget(drink, "image")
            dic_drink = {"name": drink, "colors": colors,"glass":glass, "ingredients":ingredients, "preparation":preparation, "image":image}
            list_drinks = list_drinks + [dic_drink]
            print(key)
    return jsonify(list_drinks)

@app.route("/get_ingredients", methods=["GET"])
def get_ingredients():
    list_ingredients = []
    for key in r.hgetall("INGREDIENTS"):
        list_ingredients = list_ingredients + [key]
    return jsonify(list_ingredients)

if __name__ == "__main__":
    app.run(host="0.0.0.0", port=5000, debug=True)