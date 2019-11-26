import React from 'react'

export const ContentLayout = ({ children, kind, ...rest }) => (
    <div id="content">
        <div className="header">
            <h1>{rest['title']}</h1>
        </div>
        {children}
    </div>
);

var schema = {
    title: "",
    description: "", // HTML (maybe)
    syntax: "",
    properties: [
        {
            key: "",
            description: "",
            possibleValues: [],
            default: "",
            isMandatory: false
        }
    ],
    examples: [
        {
            description: "",
            content: ""
        }
    ],
    requires: []
}