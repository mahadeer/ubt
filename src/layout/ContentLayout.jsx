import React from 'react';
import XMLViewer from 'react-xml-viewer';
import "./ContentLayout.css";

export const ContentLayout = (props) => {
    const properties = JSON.parse(props.properties);
    const examples = JSON.parse(props.examples);
    const xmlCustomTheme = {
        "tagColor": "#4CAF50"
    };
    const requires = JSON.parse(props.requires);
    const related = JSON.parse(props.related);
    return (<div id="content">
        <h1>{props['title']}</h1>
        <p dangerouslySetInnerHTML={{ __html: props.description }}></p>
        <div className="properties">
            <h3>Properties</h3>
            <table className="">
                <tbody>
                    <tr className="thead uk-background-secondary">
                        <th>Attribute</th>
                        <th>Description</th>
                        <th>Options</th>
                        <th>Required</th>
                    </tr>
                    {
                        properties.map(prop => (
                            <tr key={prop.key}>
                                <td>{prop.key}</td>
                                <td dangerouslySetInnerHTML={{ __html: prop.description }}></td>
                                <td>{prop.possibleValues.join(", ")}<br />Default is {prop.default}</td>
                                <td>{prop.required}</td>
                            </tr>
                        ))
                    }
                </tbody>
            </table>
        </div>
        <div className="examples">
            <h3>Examples</h3>
            {
                examples.map((example, idx) => (
                    <div className="example" key={idx}>
                        <p>{example.description}</p>
                        <div className="example-code"><XMLViewer xml={example.content} theme={xmlCustomTheme}></XMLViewer></div>
                        {example.note !== "" ? <p className="example-note">Note: {example.note}</p> : null}
                        {idx !== examples.length - 1 ? <hr /> : null}
                    </div>
                ))
            }
        </div>
        {requires.length > 0 ?
            <div className="requires">
                <h3>Requires</h3>
                <ul className="require-list">
                    {
                        requires.map((req, idx) => (
                            <li key={req.text}><a href={req.route}>{req.text}</a></li>
                        ))
                    }
                </ul>
            </div> : null
        }
        {related.length > 0 ?
            <div className="related">
                <h3>References</h3>
                <ul className="reference-list">
                    {
                        related.map((rel, idx) => (
                            <li key={rel.text}><a href={rel.route}>{rel.text}</a></li>
                        ))
                    }
                </ul>
            </div> : null
        }
    </div>);
};