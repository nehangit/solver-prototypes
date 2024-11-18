from flask import Flask, request, jsonify
from utils.schemaChecker import checkRequestSchemaValid, checkNotifySchemaValid
from utils.dataFormatter import inputGrapher
from cowsol.src.apis.orders import OrdersApi
from cowsol.src.strategies.spread_solver import SpreadSolverApi
from cowsol.src.util.os import load_config, set_output, save_output, log_info

app = Flask(__name__)

# Get schema for solve result, and notify input
# Codes: 200
@app.route("/notify", methods=["POST"])
def notify():
    data = request.get_json()
    if not checkNotifySchemaValid(data):
        return jsonify({"error": "Invalid JSON data."}), 400 # pointless
    print(f"Auction ID: {data['auctionId']}\nSolution ID: {data['solutionId']}\nNotification: {data['kind']}")
    return jsonify({"message": "Received notification"}), 200

# Codes: 200, 400, 429 (The solver cannot keep up. It is too busy to handle more requests.), 500
@app.route("/solve", methods=["POST"])
def solve():
    data = request.get_json()
    if not checkRequestSchemaValid(data):
        return jsonify({"error": "Invalid JSON data."}), 400
    inputGrapher(data["orders"], data["tokens"])

    # Copied and pasted (TODO: Refactor):
    oa = OrdersApi(data)

    result = {
        'amms': {},
        'orders': {}
    }

    for order_num, order in oa.orders.items():

        order = oa.parse_order_for_spread_trade(order, order_num)
        amms = oa.parse_amms_for_spread_trade(order)

        # Skip if the input order is invalid.
        if not order or not amms:
            continue

        # Create a solver instance for each order.
        solver = SpreadSolverApi(amms)
        solution = solver.solve(order)

        # Update results for orders instance.
        result['amms'].update(solution['amms'])
        result['orders'].update(solution['orders'])

    return jsonify(result) # Success
    
    

if __name__ == '__main__':
    app.run(port=8080)