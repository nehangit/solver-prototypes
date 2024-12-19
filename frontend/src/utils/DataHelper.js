export const translateData = (data) => {
    const defOps = [
        {
            type: 'addNode',
            id: '1',
            label: 'Node 1',
            position: { x: 50, y: 50 },
        },
        {
            type: 'addNode',
            id: '2',
            label: 'Node 2',
            position: { x: 200, y: 50 },
        },
        { type: 'addEdge', source: '1', target: '2', label: 'Edge 1-2' },
        {
            type: 'addNode',
            id: '3',
            label: 'Node 3',
            position: { x: 125, y: 200 },
        },
        { type: 'addEdge', source: '2', target: '3', label: 'Edge 2-3' },
        { type: 'deleteNode', id: '1' },
        { type: 'addEdge', source: '3', target: '2', label: 'Edge 3-2' },
        { type: 'deleteEdge', id: 'e2-3' },
    ];

    if (data["asset_changes"]) {
        return generateOperationsFromAssetChanges(data);
    }
    
    if (data["trades"] && data["auction_orders"] && data["liquidity_interactions"]) {
        return generateOperationsFromAuctionSolution(data);
    }

    return defOps;
};

const generateOperationsFromAssetChanges = (assetChanges) => {
    const nodesSet = new Set();
    const operations = [];
    assetChanges = assetChanges["asset_changes"];
    assetChanges.forEach((change) => {
        const { from, to, token_info, amount, type } = change;

        if (from && !nodesSet.has(from)) {
            nodesSet.add(from);
            operations.push({
                type: "addNode",
                id: from,
                label: `Wallet: ${from}`,
                position: { x: Math.random() * 800, y: Math.random() * 600 },
            });
        }

        if (to && !nodesSet.has(to)) {
            nodesSet.add(to);
            operations.push({
                type: "addNode",
                id: to,
                label: `Wallet: ${to}`,
                position: { x: Math.random() * 800, y: Math.random() * 600 },
            });
        }

        if (from && to) {
            operations.push({
                type: "addEdge",
                id: `e${from}-${to}`,
                source: from,
                target: to,
                label: `${amount} ${token_info?.symbol || ""} (${type})`,
            });
        }
    });

    return operations;
};

const generateOperationsFromAuctionSolution = (data) => {
    const operations = [];
    const nodesSet = new Set();

    // Add nodes and edges for trades
    data.trades.forEach((trade, index) => {
        const tradeId = `trade-${index}`;
        let order;
        if(trade.kind === "fulfillment") {
            order = trade.auction_orders.find((order) => order.uid === trade.order);
        }
        else { // trade.kind === "jit"
            order = trade.order
        }
        let orderKind = order.kind;
        let inToken, outToken, inAmount, outAmount;
        let limitPrice = order.sellAmount / order.buyAmount;
        let execPrice = data.prices[order.sellToken] / data.prices[order.buyToken];
        if(orderKind === "buy") {
            inToken = order.sellToken;
            outToken = order.buyToken;
            inAmount = order.sellAmount;
            outAmount = order.buyAmount;
        }
        else {
            inToken = order.buyToken;
            outToken = order.sellToken;
            inAmount = order.buyAmount;
            outAmount = order.sellAmount;
        }

        // Add order node
        const orderId = order.uid || `order-${index}`;
        if (!nodesSet.has(orderId)) {
            nodesSet.add(orderId);
            operations.push({
                type: "addNode",
                id: orderId,
                label: `${orderKind} order\nLimit Price: ${limitPrice}\nExecuted Price: ${execPrice}`,
                position: { x: Math.random() * 800, y: Math.random() * 600 },
            });
        }

        // Add nodes and edges for the receiver and owner (if applicable)
        if (order.receiver) {
            if (!nodesSet.has(order.receiver)) {
                nodesSet.add(order.receiver);
                operations.push({
                    type: "addNode",
                    id: order.receiver,
                    label: `Address\n${order.receiver}`,
                    position: { x: Math.random() * 800, y: Math.random() * 600 },
                });
            }

            operations.push({
                type: "addEdge",
                id: `receiver-edge-${index}`,
                source: orderId,
                target: order.receiver,
                label: `Token: ${inToken}\nAmount: ${inAmount}`,
            });
        }

        if (order.owner) {
            if (!nodesSet.has(order.owner)) {
                nodesSet.add(order.owner);
                operations.push({
                    type: "addNode",
                    id: order.owner,
                    label: `Owner\n${order.owner}`,
                    position: { x: Math.random() * 800, y: Math.random() * 600 },
                });
            }

            operations.push({
                type: "addEdge",
                id: `owner-edge-${index}`,
                source: orderId,
                target: order.owner,
                label: `Token: ${outToken}\nAmount: ${outAmount}`,
            });
        }
    });

     // Add nodes and edges for liquidity interactions
    data.liquidity_interactions.forEach((interaction, index) => {
        // Add 'from' node (could be an order owner address or liquidity pool)
        if (!nodesSet.has(interaction.from)) {
            nodesSet.add(interaction.from);
            operations.push({
                type: "addNode",
                id: interaction.from,
                label: `${interaction.from}`,
                position: { x: Math.random() * 800, y: Math.random() * 600 },
            });
        }

        // Add 'to' node (could be an order owner address or liquidity pool)
        if (!nodesSet.has(interaction.to)) {
            nodesSet.add(interaction.to);
            operations.push({
                type: "addNode",
                id: interaction.to,
                label: `${interaction.to}`,
                position: { x: Math.random() * 800, y: Math.random() * 600 },
            });
        }

        // Add edge from 'from' to 'to' based on liquidity interaction
        operations.push({
            type: "addEdge",
            id: `liquidity-edge-${index}`,
            source: interaction.from,
            target: interaction.to,
            label: `Token: ${interaction.token}\nAmount: ${interaction.amount}`,
        });
    });

    return operations;
};

// const generateOperationsFromAuctionSolution = (data) => {
//     const operations = [];
//     const nodesSet = new Set();

//     // Add nodes and edges for prices
//     Object.entries(data.prices).forEach(([token, price]) => {
//         if (!nodesSet.has(token)) {
//             nodesSet.add(token);
//             operations.push({
//                 type: "addNode",
//                 id: token,
//                 label: `Token: ${token}`,
//                 position: { x: Math.random() * 800, y: Math.random() * 600 },
//             });
//         }
//         operations.push({
//             type: "addEdge",
//             id: `price-${token}`,
//             source: "root",
//             target: token,
//             label: `Price: ${price}`,
//         });
//     });

//     // Add nodes and edges for trades
//     data.trades.forEach((trade, index) => {
//         const tradeId = `trade-${index}`;
//         operations.push({
//             type: "addNode",
//             id: tradeId,
//             label: `Trade: ${trade.executedAmount}`,
//             position: { x: Math.random() * 800, y: Math.random() * 600 },
//         });

//         operations.push({
//             type: "addEdge",
//             id: `trade-edge-${index}`,
//             source: tradeId,
//             target: trade.order,
//             label: `Fee: ${trade.fee}`,
//         });
//     });

//     // Add nodes and edges for interactions
//     data.interactions.forEach((interaction, index) => {
//         const inputToken = interaction.inputToken;
//         const outputToken = interaction.outputToken;

//         if (!nodesSet.has(inputToken)) {
//             nodesSet.add(inputToken);
//             operations.push({
//                 type: "addNode",
//                 id: inputToken,
//                 label: `Token: ${inputToken}`,
//                 position: { x: Math.random() * 800, y: Math.random() * 600 },
//             });
//         }

//         if (!nodesSet.has(outputToken)) {
//             nodesSet.add(outputToken);
//             operations.push({
//                 type: "addNode",
//                 id: outputToken,
//                 label: `Token: ${outputToken}`,
//                 position: { x: Math.random() * 800, y: Math.random() * 600 },
//             });
//         }

//         operations.push({
//             type: "addEdge",
//             id: `interaction-${index}`,
//             source: inputToken,
//             target: outputToken,
//             label: `Input: ${interaction.inputAmount}, Output: ${interaction.outputAmount}`,
//         });
//     });

//     return operations;
// };
