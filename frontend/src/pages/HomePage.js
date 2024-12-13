import React from 'react';
import DataInput from '../components/DataInput';
import { useNavigate } from 'react-router-dom';

const HomePage = ({ onDataSubmit }) => {
    const navigate = useNavigate();
    const handleDataSubmit = (data) => {
        onDataSubmit(data);
        navigate('/visualization');
    };
    return (
        <div>
            <h1>Ethereum Auction Data Visualization</h1>
            <DataInput onSubmit={handleDataSubmit} />
        </div>
    );
};

export default HomePage;
