
var app = new Vue({
    el: '#app',
    data: {
        selectedMov: '',
        options: { text: 'LOADING...', value: '' },
        socketStatus: "<NONE>"
    }
});

$.get(
    '/mov-files',
    function(data) {
        app._data.options = data;
    },
    "json"
);

let socket = new WebSocket("ws://localhost:9001");

socket.onopen = function(e) {
    app._data.socketStatus = "[open] Connection established";
};

socket.onmessage = function(event) {
    app._data.socketStatus = `[message] Data received from server: ${event.data}`;
    
    let player = $("#video-player")[0];
    if (player.currentSrc === "") {
        return;
    }

    let msg = JSON.parse(event.data);
    if (msg.action === "play") {
        player.play();
    } else if (msg.action === "stop") {
        player.pause();
    } else if (msg.action === "setTime") {
        let time = parseInt(msg.metadata);
        player.currentTime = time;
    }
};

socket.onclose = function(event) {
    if (event.wasClean) {
        app._data.socketStatus = `[close] Connection closed cleanly, code=${event.code} reason=${event.reason}`;
    } else {
        // e.g. server process killed or network down
        // event.code is usually 1006 in this case
        app._data.socketStatus = '[close] Connection died';
    }
};

socket.onerror = function(error) {
    app._data.socketStatus = `[error] ${error.message}`;
};