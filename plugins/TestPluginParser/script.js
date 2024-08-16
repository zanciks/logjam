const folderPath = "/folder/with/logs";
const pluginName = 'TestPluginParser'
const infoContainer = document.getElementById('info-container');

function startSearches() {
    window.__TAURI__.invoke('parse_xml', { pluginName })
        .then((trios) => {
            trios.forEach(trio => {
                const [name, re, logFile] = trio;

                const infoDiv = document.createElement('div');
                infoDiv.className = 'info';

                const infoName = document.createElement('div');
                infoName.className = 'infoName';
                infoName.id = name;
                infoName.innerHTML = `${name}`;

                const infoValue = document.createElement('div');
                infoValue.className = 'infoValue';
                infoValue.id = name;

                infoDiv.appendChild(infoName);
                infoDiv.appendChild(infoValue);
                infoContainer.appendChild(infoDiv);

                window.__TAURI__.invoke('begin_search', { name, re, fileName: logFile, folderPath });
            });
        });
}

window.__TAURI__.event.listen('search-event', event => {
    const match = event.payload.match(/^\[(.*?)\] \| \[(.*?)\]$/);
    if (match) {
        const name = match[1];
        const value = match[2];

        const infoValue = document.querySelector(`.infoValue#${name}`);
        if (infoValue) {
            infoValue.innerHTML = value;
        }
    }
});

startSearches();