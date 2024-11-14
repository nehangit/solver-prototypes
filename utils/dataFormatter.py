import networkx as nx

#https://docs.cow.fi/cow-protocol/reference/core/auctions/schema
def inputGrapher(orders, tokens):
    graph = nx.DiGraph()
    
    for order in orders:
        uid = order["uid"]
        sell_token = order["sellToken"]
        buy_token = order["buyToken"]
        sell_amount = int(order["sellAmount"])
        buy_amount = int(order["buyAmount"])
        
        graph.add_node(uid, sell_token=sell_token, buy_token=buy_token, sell_amount=sell_amount, buy_amount=buy_amount)
    
    for uid1, data1 in graph.nodes(data=True):
        for uid2, data2 in graph.nodes(data=True):
            if uid1 != uid2 and data1["buy_token"] == data2["sell_token"]:
                sell_ref_price = int(tokens[data1["sell_token"]]["referencePrice"])
                buy_ref_price = int(tokens[data1["buy_token"]]["referencePrice"])
                
                weight = (data1["buy_amount"] / 10**tokens[data1["buy_token"]]["decimals"]) * (buy_ref_price / sell_ref_price)
                
                graph.add_edge(uid1, uid2, weight=weight)
    
    return graph