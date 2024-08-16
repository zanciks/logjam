function startSearches() {
    const infoContainer = document.getElementById('info-container');
    window.__TAURI__.invoke('parse_xml')
        .then((trios) => {
            trios.forEach(trio => {
                const [name, regex, logFile] = trio;
                const folderPath = "/home/zanciks/.factorio";

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

                window.__TAURI__.invoke('begin_search', { name, re: regex, fileName: logFile, folderPath });
            });
        });
}

startSearches();

window.__TAURI__.event.listen('search-event', event => {
  console.log("Received event:", event.payload);
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
