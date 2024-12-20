import React, { Component } from 'react';
import { Link } from 'react-router-dom';
import './NavBar.css';

class NavBar extends Component {
    render() {
        return (
            <nav className="navbar">
                <div className="navbar-logo">
                    <Link to="/" className="navbar-logo-link">
                        solverevlos
                    </Link>
                </div>
            </nav>
        );
    }
}

export default NavBar;
