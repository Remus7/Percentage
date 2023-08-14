from flask import Flask, jsonify
import redis
import os
import json

app = Flask(__name__)

r = redis.Redis(host=os.environ['REDIS_HOST'], port=6379, decode_responses=True)

@app.route("/get_drinks/<ingredient>", methods=["GET"])
def get_drinks(ingredient):
    ingredient = ingredient.lower()
    list_drinks = []
    drinks = r.hget("INGREDIENTS", ingredient)
    if drinks != None:
        drinks = json.loads(drinks)
    else:
        return jsonify({"error":"no drinks found"})
    for drink in drinks:
        drink = drink.lower()
        glass = r.hget(drink, "glass")
        ingredients = json.loads(r.hget(drink, "ingredients"))
        preparation = r.hget(drink, "preparation")
        list_drinks.append({"name": drink, "glass":glass, "ingredients":ingredients, "preparation":preparation})
    return jsonify(list_drinks)

if __name__ == "__main__":
    app.run(host="0.0.0.0", port=5000, debug=True)