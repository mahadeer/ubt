const path = require('path');
const fs = require('fs');

const folders = ["reference"];

folders.forEach(function (currentFolder) {
    const destination = path.join(__dirname, "../", currentFolder);
    let templateStr = fs.readFileSync(path.join(__dirname, currentFolder, 'template.txt')).toString();

    const directoryPath = path.join(__dirname, currentFolder);
    const files = fs.readdirSync(directoryPath);
    files.forEach(function (file) {
        if (file.indexOf(".json") > -1) {
            const fileConfig = require(path.join(__dirname, currentFolder, file));
            Object.keys(fileConfig).forEach(function (key) {
                const regex = new RegExp(`{${key}}`, "g");
                let value = fileConfig[key];
                if (typeof value == "object") {
                    value = JSON.stringify(value);
                }
                templateStr = templateStr.replace(regex, value);
            });
            const outputPath = path.join(destination, file.split(".")[0] + ".mdx");
            fs.writeFileSync(outputPath, templateStr);
            console.log(`\t Generated ${file.split(".")[0]}.mdx`)
        }
    });
});