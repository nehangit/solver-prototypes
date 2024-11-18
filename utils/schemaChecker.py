from jsonschema import validate, ValidationError

REQUEST_SCHEMA = {
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
                "required": ["availableBalance", "trusted"]
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
                    "feePolicies": {    # refactor?
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
                "required": ["uid", "sellToken", "buyToken", "sellAmount", "buyAmount", "kind", "partiallyFillable", "class"] # kind: "sell" or "buy", class: "limit" or "market"
            }
        },
        "liquidity": {
            "type": "array",
            "items": {
                "type": "object",
                # "properties": {}, # TODO
                # "required": [] # TODO
                }
        },
        "effectiveGasPrice": {"type": "string"},
        "deadline": {"type": "string", "format": "date-time"},
        "surplusCapturingJitOrderOwners": {
            "type": "array",
            "items": {"type": "string"}
        }
    },
    "required": ["tokens", "orders", "liquidity", "effectiveGasPrice", "deadline", "surplusCapturingJitOrderOwners"]
}

NOTIFY_SCHEMA = {
    "type": "object",
    "properties": {
        "auctionId": {"type": "string"},
        "solutionId": {"type": "number"},
        "kind": {
            "type": "string",
            "enum": [
                "timeout", "emptySolution", "duplicatedSolutionId", "simulationFailed",
                "invalidClearingPrices", "missingPrice", "invalidExecutedAmount",
                "nonBufferableTokensUsed", "solverAccountInsufficientBalance", "success",
                "revert", "driverError", "cancelled", "fail", "postprocessingTimedOut"
            ]
        }
    },
    "required": ["auctionId", "solutionId", "kind"]
}


SOLUTION_SCHEMA = {
    "type": "object",
    "required": [
    "solutions"
    ],
    "properties": {
    "solutions": {
        "type": "array",
        "items": {
        "type": "object",
        "required": [
            "id",
            "prices",
            "trades",
            "interactions"
        ],
        "properties": {
            "id": {
            "type": "number"
            },
            "prices": {
            "type": "object",
            "additionalProperties": {
                "type": "string",
            }
            },
            "trades": {
            "type": "array",
            "items": {
                "oneOf": [
                {
                    "type": "object",
                    "required": [
                    "kind",
                    "order"
                    ],
                    "properties": {
                    "kind": {
                        "type": "string",
                        "enum": [
                        "fulfillment"
                        ]
                    },
                    "order": {
                        "allOf": [
                        {
                            "type": "string",
                        }
                        ]
                    },
                    "fee": {
                        "description": "The sell token amount that should be taken as a fee for this\ntrade. This only gets returned for limit orders and only refers\nto the actual amount filled by the trade.\n"
                    },
                    "executedAmount": {
                        "allOf": [
                        {
                            "type": "string",
                        }
                        ]
                    }
                    }
                },
                {
                    "required": [
                    "kind",
                    "order",
                    "executedAmount",
                    "fee"
                    ],
                    "properties": {
                    "kind": {
                        "type": "string",
                        "enum": [
                        "jit"
                        ]
                    },
                    "executedAmount": {
                        "allOf": [
                        {
                            "type": "string",
                        }
                        ]
                    },
                    "fee": {
                        "allOf": [
                        {
                            "type": "string",
                        }
                        ]
                    },
                    "order": {
                        "allOf": [
                        {
                            "type": "object",
                            "required": [
                            "sellToken",
                            "buyToken",
                            "receiver",
                            "sellAmount",
                            "buyAmount",
                            "validTo",
                            "appData",
                            "kind",
                            "sellTokenBalance",
                            "buyTokenBalance",
                            "signingScheme",
                            "signature"
                            ],
                            "properties": {
                            "sellToken": {
                                "type": "string",
                            },
                            "buyToken": {
                                "type": "string",
                            },
                            "receiver": {
                                "type": "string",
                            },
                            "sellAmount": {
                                "type": "string",
                            },
                            "buyAmount": {
                                "type": "string",
                            },
                            "validTo": {
                                "type": "integer"
                            },
                            "appData": {
                                "description": "32 bytes of arbitrary application specific data that can be added to an\norder. This can also be used to ensure uniqueness between two orders\nwith otherwise the exact same parameters.\n",
                            },
                            "kind": {
                                "type": "string",
                                "enum": [
                                "sell",
                                "buy"
                                ]
                            },
                            "sellTokenBalance": {
                                "type": "string",
                                "enum": [
                                "erc20",
                                "internal",
                                "external"
                                ]
                            },
                            "buyTokenBalance": {
                                "type": "string",
                                "enum": [
                                "erc20",
                                "internal"
                                ]
                            },
                            "signingScheme": {
                                "type": "string",
                                "enum": [
                                "eip712",
                                "ethSign",
                                "preSign",
                                "eip1271"
                                ]
                            },
                            "signature": {
                                "type": "string",
                            }
                            }
                        }
                        ]
                    }
                    }
                }
                ]
            }
            },
            "preInteractions": {
            "type": "array",
            "items": {
                "type": "object",
                "properties": {
                "target": {
                    "type": "string",
                },
                "value": {
                    "type": "string",
                },
                "callData": {
                    "type": "array",
                    "items": {
                    "type": "string",
                    },
                }
                }
            }
            },
            "interactions": {
            "type": "array",
            "items": {
                "allOf": [
                {
                    "type": "object",
                    "properties": {
                    "internalize": {
                        "type": "boolean"
                    }
                    }
                },
                {
                    "oneOf": [
                    {
                        "type": "object",
                        "required": [
                        "kind",
                        "id",
                        "inputToken",
                        "outputToken",
                        "inputAmount",
                        "outputAmount"
                        ],
                        "properties": {
                        "kind": {
                            "type": "string",
                            "enum": [
                            "liquidity"
                            ]
                        },
                        "id": {
                            "type": "number"
                        },
                        "inputToken": {
                            "type": "string",
                        },
                        "outputToken": {
                            "type": "string",
                        },
                        "inputAmount": {
                            "type": "string",
                        },
                        "outputAmount": {
                            "type": "string",
                        }
                        }
                    },
                    {
                        "type": "object",
                        "required": [
                        "kind",
                        "target",
                        "value",
                        "callData",
                        "inputs",
                        "outputs"
                        ],
                        "properties": {
                        "kind": {
                            "type": "string",
                            "enum": [
                            "custom"
                            ]
                        },
                        "target": {
                            "type": "string",
                        },
                        "value": {
                            "type": "string",
                        },
                        "callData": {
                            "type": "string",
                        },
                        "allowances": {
                            "type": "array",
                            "items": {
                            "type": "object",
                            "required": [
                                "token",
                                "spender"
                            ],
                            "properties": {
                                "token": {
                                "type": "string",
                                },
                                "spender": {
                                "type": "string",
                                },
                                "amount": {
                                "type": "string",
                                }
                            }
                            }
                        },
                        "inputs": {
                            "type": "array",
                            "items": {
                            "type": "object",
                            "required": [
                                "token",
                                "amount"
                            ],
                            "properties": {
                                "token": {
                                "type": "string",
                                },
                                "amount": {
                                "type": "string",
                                }
                            }
                            }
                        },
                        "outputs": {
                            "type": "array",
                            "items": {
                            "type": "object",
                            "required": [
                                "token",
                                "amount"
                            ],
                            "properties": {
                                "token": {
                                "type": "string",
                                },
                                "amount": {
                                "type": "string",
                                }
                            }
                            }
                        }
                        }
                    }
                    ]
                }
                ]
            }
            },
            "postInteractions": {
            "type": "array",
            "items": {
                "type": "object",
                "properties": {
                "target": {
                    "type": "string",
                },
                "value": {
                    "type": "string",
                },
                "callData": {
                    "type": "array",
                    "items": {
                    "type": "string",
                    },
                }
                }
            }
            },
            "gas": {
            "type": "integer",
            }
        }
        }
    }
    }
}

def checkNotifySchemaValid(requestJson) -> bool:
    try:
        validate(instance=requestJson, schema=NOTIFY_SCHEMA)
        return True
    except ValidationError as e:
        return False

def checkRequestSchemaValid(requestJson) -> bool:
    try:
        validate(instance=requestJson, schema=REQUEST_SCHEMA)
        return True
    except ValidationError as e:
        return False
