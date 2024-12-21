import React from 'react';
import DataInput from '../components/DataInput';
import { useNavigate } from 'react-router-dom';

const SOLVER_API_URL = 'http://localhost:8080';
// const SOLVER_API_URL = 'https://solverevlos-1edc36e90ed9.herokuapp.com';

const HomePage = ({ onDataSubmit }) => {
    const navigate = useNavigate();
    
    const handleDataSubmit = (data) => {
        onDataSubmit(data);
        navigate('/visualization');
    };

    const handleAuctionDataSubmit = (data) => {
        fetch(`${SOLVER_API_URL}/visualize`, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify(data),
        })
            .then((response) => {
                if (!response.ok) {
                    return response.json().then((errData) => {
                        throw new Error(errData.error);
                    });
                }
                return response.json();
            })
            .then((res) => {
                solution = res.solutions[0];
                solution.auction_orders = data.orders;
                solution.liquidity_interactions = res.graph_edges
                onDataSubmit(solution);
                navigate('/visualization');
            })
            .catch((error) => {
                alert('Error: ' + error.message);
            });
    }

    return (
        <div style={{ display: 'flex', justifyContent: 'space-evenly' }}>
            <div>
                <h1>Ethereum On Chain Transactions Visualization</h1>
                <h3><i>Paste Tenderly API response below</i></h3>
                <DataInput onSubmit={handleDataSubmit} />
            </div>
            <div>
                <h1>Ethereum Auction Solution Visualization</h1>
                <h3><i>Paste auction JSON data below</i></h3>
                <DataInput onSubmit={handleAuctionDataSubmit} />
            </div>
        </div>
    );
};

export default HomePage;
