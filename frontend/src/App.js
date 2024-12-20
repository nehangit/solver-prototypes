import React, { useState } from 'react';
import { BrowserRouter as Router, Routes, Route } from 'react-router-dom';
import HomePage from './pages/HomePage';
import VisualizationPage from './pages/VisualizationPage';
import NavBar from './components/NavBar/NavBar';

const App = () => {
    const [data, setData] = useState(null);

    return (
        <Router>
            <NavBar />
            <Routes style={{ height: '95vh', width: '100vw' }}>
                <Route path="/" element={<HomePage onDataSubmit={setData} />} />
                <Route
                    path="/visualization"
                    element={<VisualizationPage data={data} />}
                />
            </Routes>
        </Router>
    );
};

export default App;
