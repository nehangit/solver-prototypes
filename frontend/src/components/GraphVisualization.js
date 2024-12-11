import React from "react";
import { ResponsiveContainer, Sankey } from "recharts";

const GraphVisualization = ({ data }) => {
  const extractGraphData = (inputData) => {
    const nodes = [];
    const links = [];

    const addNode = (id) => {
      if (!nodes.some((node) => node.name === id)) {
        nodes.push({ name: id });
      }
    };

    inputData.solutions.forEach((solution) => {
      solution.trades.forEach((trade) => {
        const source = trade.kind === "jit" ? trade.order.sellToken : trade.order;
        const target = trade.kind === "jit" ? trade.order.buyToken : "Liquidity Pool";
        addNode(source);
        addNode(target);
        links.push({ source, target, value: parseInt(trade.executedAmount, 10) });
      });

      solution.interactions.forEach((interaction) => {
        const source = interaction.inputToken;
        const target = interaction.outputToken;
        addNode(source);
        addNode(target);
        links.push({ source, target, value: parseInt(interaction.inputAmount, 10) });
      });
    });

    return { nodes, links };
  };

  const graphData = extractGraphData(data);

  return (
    <ResponsiveContainer width="100%" height={400}>
      <Sankey
        data={graphData}
        node={{ width: 15, padding: 10 }}
        link={{ stroke: "#8884d8" }}
      />
    </ResponsiveContainer>
  );
};

export default GraphVisualization;
