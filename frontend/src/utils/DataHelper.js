export const translateData = (data) => {
    const operations = [
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
    return operations;
};
