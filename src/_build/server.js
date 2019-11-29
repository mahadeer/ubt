'use strict';

const Hapi = require('@hapi/hapi');
const Path = require('path');
const fs = require("fs");

const init = async () => {
    const server = Hapi.server({
        port: 1307,
        host: 'localhost',
        routes: {
            files: {
                relativeTo: Path.join(__dirname, 'public')
            }
        }
    });

    await server.register(require('inert'));
    await server.start();

    server.route({
        method: 'GET',
        path: '/',
        handler: (request, h) => {
            return h.file("index.html");
        }
    });

    server.route({
        method: 'GET',
        path: '/JSONFormApp.js',
        handler: (request, h) => {
            return h.file("JSONForm/dist/JSONFormApp.js");
        }
    });

    server.route({
        method: 'GET',
        path: '/references',
        handler: (request, h) => {
            const currentFolder = "reference";
            const directoryPath = Path.join(__dirname, currentFolder);
            let files = fs.readdirSync(directoryPath);
            files = files.reduce(function (acc, file) {
                if (file.indexOf(".json") > -1) {
                    acc.push(file.split(".")[0]);
                }
                return acc;
            }, []);
            return files;
        }
    });

    server.route({
        method: 'GET',
        path: '/reference/{fileName}',
        handler: (request, h) => {
            const currentFolder = "reference";
            const filePath = Path.join(__dirname, currentFolder, request.params.fileName + ".json");
            let docData = require(filePath);
            return docData;
        }
    });

    server.route({
        method: 'POST',
        path: '/reference/{fileName}',
        handler: (request, h) => {
            const currentFolder = "reference";
            const filePath = Path.join(__dirname, currentFolder, request.params.fileName + ".json");
            fs.writeFileSync(filePath, request.payload, { encoding: 'utf8' });
            console.info(`Updated docs data for ${filePath}`);
            return { "status": true };
        }
    });

    console.log('Server running on %s', server.info.uri);
};

process.on('unhandledRejection', (err) => {
    console.log(err);
    process.exit(1);
});

init();