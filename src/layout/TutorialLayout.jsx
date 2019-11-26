import React from 'react'

export const TutorialLayout = ({ children, kind, ...rest }) => (
    <div id="content">
        <div className="header">
            <h3>{rest['title']}</h3>
        </div>
        <div id="code"></div>
        <div id="output"></div>
        {children}
    </div>
);