import React from "react";
import GraphVisualization from "../components/GraphVisualization";

const VisualizationPage = ({ data }) => {
  return (
    <div>
      <h1>Visualization</h1>
      {data ? <GraphVisualization data={data} /> : <p>No data to visualize.</p>}
    </div>
  );
};

export default VisualizationPage;
