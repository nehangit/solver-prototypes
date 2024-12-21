# Automated Batch Auction Solver for CoW Protocol

## Motivation

In traditional finance, orders are typically collected in order books. Think of stock exchanges or futures markets. A trade happens whenever a buy order of one token is matched by a sell order of the other, i.e., if there exists an exchange rate that satisfies the limit prices stated in the respective orders.

However, decentralized finance (DeFi) presents a unique challenge. Instead of trading in US Dollars, for example, there are hundreds of thousands of different tradable tokens. In such a setting, one separate order book is required for every token pair combination. This may significantly limit liquidity for less frequently traded token pairs and lead to large bid-ask spreads and low trading volumes.

One solution to this problem is the batch auction. Orders are collected into a batch and executed at once in discrete time intervals. This leads to greater liquidity and removes arbitrage opportunities, yielding benefits to all users of batch auction protocols.

## Background on CoW Protocol and Solvers

### What is the CoW Protocol?

The **CoW Protocol** (Coincidence of Wants Protocol) is a decentralized system for trading crypto assets efficiently and securely. It connects users with matching trade needs directly or combines trades to minimize costs, avoiding the need for traditional intermediaries like exchanges.

Rather than placing orders by signing a raw transaction that executes directly on-chain (i.e. as happens on Uniswap or SushiSwap), CoW Protocol users place orders by signing an "intent to trade" message that specifies parameters like the assets and amounts they would like to trade. The intent is a signed message which allows the solvers to execute a trade on behalf of the user using their specified assets and amounts.

There are a number of financial and technical advantages to intent-based trading. When a user places a trade directly on-chain, the execution path is set in stone and the trader receives whatever price (+/- slippage) the AMM or aggregator they're trading on gives them. In contrast, batch auction protocols such as CoW group and process many intents together to find optimal matches and execution paths, which reduces transaction costs.

Thanks to its intent-based architecture, CoW Protocol delegates the job of finding the optimal execution path to professional third parties known as solvers. Solvers not only scan all available on-chain liquidity (similar to a DEX aggregator) they also provide extra price improvements in several ways:

- **Coincidence of Wants (CoW)**: Direct P2P (peer-to-peer) matching for two or more users who have expressed opposite buy and sell intents for the same token pair. This optimization, enabled by batch auctions, allows users to bypass liquidity provider (LP) fees and reduces the gas fees for trading as orders don't need to interact with more smart contracts.
- **Private Market Maker Inventory**: Many solvers have access to off-chain liquidity, integration with private market makers, or private liquidity of their own. This allows them to fill certain trades at better prices than what may be available through on-chain AMMs at any given time.
- **MEV (Maximum Extractable Value) Protection**: MEV refers to the profit that miners, validators, or arbitrageurs can extract by reordering, including, or excluding transactions within a block before it's finalized. CoW Protocol, users are never exposed to MEV bots. Instead, solvers settle trades on behalf of the user, taking on all MEV risk. In addition, solvers submit all orders in a batch at a uniform clearing price, meaning there is no advantage to re-ordering trades (the basis of all MEV exploitation).

### What Exactly Are Solvers?

**Solvers** are entities within the CoW Protocol that compete to execute trades in the best way possible. When you submit a trade request, Solvers:

1. Analyze available trading options.
2. Consider all markets and matching opportunities.
3. Propose the most cost-effective solution.

For each batch auction, there is a competition between the active solvers to find the best optimization for the same objective function (maximizing "surplus" across the executed trades, see Technical Details section below). The Solver that finds the optimal set of trades/transfers wins and executes their solution, ensuring efficiency and fairness. The Solver that wins is rewarded by the CoW protocol in COW tokens, incentivizing them to compete to find better prices and win the right to execute user intents.

### Why Are Solvers Important?

- **Optimal Pricing**: They ensure you get the best price by exploring all available paths.
- **Reduced Costs**: Solvers optimize trades to minimize fees and maximize value.
- **Competition-Driven**: Multiple Solvers competing ensures the best solution is chosen.

By integrating batch auctions and leveraging Solvers, the CoW Protocol delivers a robust and innovative framework for best price crypto trading.

### CoW Protocol Order Flow

![CoW Protocol Order Flow](./images/cowflow.png)

---

## Project Overview

Our project focuses on creating our own solver from scratch, using Rust and hosted on Heroku, and entering the solver competition, in order to facilitate trades as specified by the CoW Protocol. Our solver match orders to available liquidity, choosing efficient and consistent clearing prices to guarantee a fair and efficient batch auction.

Additionally, we created a JavaScript visualization to display solutions created by our solver or other people's solvers. This will provide a clean overview of what a solver deemed to be the most fair and surplus-maximizing solution for a batch auction.

---

## Technical Details

### Surplus Calculation, Objective Function, and Clearing Prices

![Optimization Problem](./images/optimization.png)

### Provided Liquidity Immediate Matching (Luke's Solver)

### Gecko Terminal Best On Chain Liquidity Solver (Philip's Solver)

This solver uses the [GeckoTerminal API](https://www.geckoterminal.com/dex-api) to fetch pools

### Bonus: Nonlinear Programming

### Graph Visualization

### Architecture

- **Programming Languages**:
- **Python**: For high-level logic, testing, and performance analysis.
- **Server**: Hosted on Heroku, implementing CoW protocol-required endpoints.
- **Optimization Algorithms**: Includes Bellman-Ford for graph-based optimization.
- **Testing and Simulation**: Locally simulate batch auctions and compare solutions.

### Tools and Libraries

- Open-source solvers for CoW protocol:
  - [Solver Template](https://github.com/cowprotocol/solver-template-py)
  - [Searcher CoWswap](https://github.com/autistic-symposium/searcher-cowswap-py)
- CoW Protocol Documentation:
  - [CoW Docs](https://docs.cow.fi/cow-protocol)
  - [The Problem](https://docs.cow.fi/cow-protocol/reference/core/auctions/the-problem)

---

## Server Requirements

The Heroku server must:

1. Provide a health check endpoint returning `True`.
2. Accept a JSON file with:
   - Orders
   - Available AMM liquidity in the current batch
3. Respond with a JSON file containing:
   - Order routing solutions
   - Pricing details

### Local Testing

1. Use CoW protocol's shadow competition tools.
2. Simulate batch auctions and verify surplus calculations.

---

## Usage Instructions

### Cloning the Repository

```bash
git clone <repository-url>
cd <repository-folder>
```

### Setting Up the Environment

1. Install Python dependencies:
   ```bash
   pip install -r requirements.txt
   ```
2. Deploy the server locally or on Heroku:
   ```bash
   heroku deploy
   ```

### Running the Solver

1. Start the server:
   ```bash
   python main.py
   ```
2. Test endpoints using sample batch auction data.

### Visualizations

Run the visualization script to see real-time graph transactions:

```bash
python visualize.py
```

---

## Our Team

## Acknowledgments

- [CoW Protocol Documentation](https://docs.cow.fi/cow-protocol)
- [Solver Template Repository](https://github.com/cowprotocol/solver-template-py)
- [Searcher CoWswap Repository](https://github.com/autistic-symposium/searcher-cowswap-py)
