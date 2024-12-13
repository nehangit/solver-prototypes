import React, { useEffect } from 'react';
import ReactFlow, {
    MiniMap,
    Controls,
    Background,
    useNodesState,
    useEdgesState,
} from 'reactflow';
import 'reactflow/dist/style.css';

const GraphVisualFlow = ({ operations }) => {
    const [nodes, setNodes, onNodesChange] = useNodesState([]);
    const [edges, setEdges, onEdgesChange] = useEdgesState([]);

    const delay = (ms) => new Promise((resolve) => setTimeout(resolve, ms));
    useEffect(() => {
        const handleOperations = async () => {
            let updatedNodes = [...nodes];
            let updatedEdges = [...edges];

            for (const op of operations) {
                if (op.type === 'addNode') {
                    const newNode = {
                        id: op.id,
                        position: op.position || { x: 100, y: 100 },
                        data: { label: op.label || `Node ${op.id}` },
                    };
                    updatedNodes.push(newNode);
                } else if (op.type === 'deleteNode') {
                    updatedNodes = updatedNodes.filter(
                        (node) => node.id !== op.id
                    );
                    updatedEdges = updatedEdges.filter(
                        (edge) => edge.source !== op.id && edge.target !== op.id
                    );
                } else if (op.type === 'addEdge') {
                    const newEdge = {
                        id: op.id || `e${op.source}-${op.target}`,
                        source: op.source,
                        target: op.target,
                        label: op.label || '',
                    };
                    updatedEdges.push(newEdge);
                } else if (op.type === 'deleteEdge') {
                    updatedEdges = updatedEdges.filter(
                        (edge) => edge.id !== op.id
                    );
                }

                // Update the state after processing each operation
                setNodes([...updatedNodes]);
                setEdges([...updatedEdges]);

                // Wait for 250ms before processing the next operation
                await delay(500);
            }
        };
        handleOperations();
    }, []);

    return (
        <div style={{ height: '95vh', width: '100vw' }}>
            <ReactFlow
                nodes={nodes}
                edges={edges}
                onNodesChange={onNodesChange}
                onEdgesChange={onEdgesChange}
                fitView
            >
                <MiniMap />
                <Controls />
                <Background />
            </ReactFlow>
        </div>
    );
};

export default GraphVisualFlow;
