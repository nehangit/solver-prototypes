import React, { useState } from "react";

const DataInput = ({ onSubmit }) => {
  const [inputData, setInputData] = useState("");

  const handleSubmit = (e) => {
    e.preventDefault();
    try {
      const parsedData = JSON.parse(inputData);
      console.log(parsedData)
      onSubmit(parsedData);
    } catch (err) {
      alert("Invalid JSON format");
    }
  };

  return (
    <form onSubmit={handleSubmit}>
      <textarea
        rows="10"
        cols="50"
        value={inputData}
        onChange={(e) => setInputData(e.target.value)}
        placeholder="Paste JSON data here"
      ></textarea>
      <br />
      <button type="submit">Visualize</button>
    </form>
  );
};

export default DataInput;
