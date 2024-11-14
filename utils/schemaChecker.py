from flask import jsonify
from jsonschema import validate, ValidationError

SCHEMA = {
    "type": "object",
    "properties": {
        "id": {"type": "string"},
        "tokens": {
            "type": "object",
            "additionalProperties": {
                "type": "object",
                "properties": {
                    "decimals": {"type": "integer"},
                    "symbol": {"type": "string"},
                    "referencePrice": {"type": "string"},
                    "availableBalance": {"type": "string"},
                    "trusted": {"type": "boolean"}
                },
                "required": ["decimals", "symbol", "referencePrice", "availableBalance", "trusted"]
            }
        },
        "orders": {
            "type": "array",
            "items": {
                "type": "object",
                "properties": {
                    "uid": {"type": "string"},
                    "sellToken": {"type": "string"},
                    "buyToken": {"type": "string"},
                    "sellAmount": {"type": "string"},
                    "fullSellAmount": {"type": "string"},
                    "buyAmount": {"type": "string"},
                    "fullBuyAmount": {"type": "string"},
                    "feePolicies": {
                        "type": "array",
                        "items": {
                            "type": "object",
                            "properties": {
                                "kind": {"type": "string"},
                                "maxVolumeFactor": {"type": "number"},
                                "factor": {"type": "number"},
                                "quote": {
                                    "type": "object",
                                    "properties": {
                                        "sell_amount": {"type": "string"},
                                        "buy_amount": {"type": "string"},
                                        "fee": {"type": "string"}
                                    },
                                    "required": ["sell_amount", "buy_amount", "fee"]
                                }
                            },
                            "required": ["kind", "factor"]
                        }
                    },
                    "validTo": {"type": "integer"},
                    "kind": {"type": "string"},
                    "receiver": {"type": "string"},
                    "owner": {"type": "string"},
                    "partiallyFillable": {"type": "boolean"},
                    "preInteractions": {"type": "array"},
                    "postInteractions": {"type": "array"},
                    "sellTokenSource": {"type": "string"},
                    "buyTokenDestination": {"type": "string"},
                    "class": {"type": "string"},
                    "appData": {"type": "string"},
                    "signingScheme": {"type": "string"},
                    "signature": {"type": "string"}
                },
                "required": ["uid", "sellToken", "buyToken", "sellAmount", "buyAmount"]
            }
        },
        "liquidity": {
            "type": "array",
            "items": {"type": "object"}
        },
        "effectiveGasPrice": {"type": "string"},
        "deadline": {"type": "string", "format": "date-time"},
        "surplusCapturingJitOrderOwners": {
            "type": "array",
            "items": {"type": "string"}
        }
    },
    "required": ["id", "tokens", "orders", "effectiveGasPrice", "deadline"]
}

def checkSchemaValid(requestJson) -> bool:
    try:
        validate(instance=requestJson, schema=SCHEMA)
        return True
    except ValidationError as e:
        return False
