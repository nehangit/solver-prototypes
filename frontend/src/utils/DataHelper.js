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
    if (data["asset_changes"]){
        return generateOperationsFromAssetChanges(data);
    }
    // Check if data is auction solution here and generate operations in a separate function
    
    return defOps;
};

const generateOperationsFromAssetChanges = (assetChanges) => {
    const nodesSet = new Set(); 
    const operations = [];
    assetChanges  = assetChanges["asset_changes"];
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


