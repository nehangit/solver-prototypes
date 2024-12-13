import React from 'react';
import GraphVisualFlow from '../components/GraphVisualFlow';
import { translateData } from '../utils/DataHelper';


const VisualizationPage = ({ data }) => {

    
    const operations = translateData(data);
    return (
        <div>
            <h1>Visualization</h1>
            {operations ? (
                <GraphVisualFlow operations={operations} />
            ) : (
                <p>No data to visualize.</p>
            )}
        </div>
    );
};

export default VisualizationPage;
