import React from "react";
import DataInput from "../components/DataInput";

const HomePage = ({ onDataSubmit }) => {
  return (
    <div>
      <h1>Ethereum Auction Data Visualization</h1>
      <DataInput onSubmit={onDataSubmit} />
    </div>
  );
};

export default HomePage;
