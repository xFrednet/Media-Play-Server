var app = new Vue({
    el: '#app',
    data: {
        socket: null,
        socketStatus: "<NONE>",
        socketLog: "",
        setTimeTime: 60
    },
    mounted: function() {
        let data = this._data;
        data.socket = new WebSocket("ws://localhost:9001");

        data.socket.onopen = function(e) {
            let msg = "[open] Connection established";
            data.socketStatus = msg;
            data.socketLog += msg + '\n';
        };
        
        data.socket.onmessage = function(event) {
            let msg = `[message]: ${event.data}`;
            data.socketLog += msg + '\n';
        };
        
        data.socket.onclose = function(event) {
            if (event.wasClean) {
                let msg = `[close] Connection closed cleanly, code=${event.code} reason=${event.reason}`;
                data.socketStatus = msg;
                data.socketLog += msg + '\n';
            } else {
                // e.g. server process killed or network down
                // event.code is usually 1006 in this case
                let msg = `[close] Connection died`;
                data.socketStatus = msg;
                data.socketLog += msg + '\n';
            }
        };
        
        data.socket.onerror = function(error) {
            let msg = `[error] ${error.message}`;
            data.socketStatus = msg;
            data.socketLog += msg + '\n';
        };
    },
    methods: {
        sendPlay: function() {
            let action = {
                "action": "play",
                "metadata": null
            };
            this.socket.send(JSON.stringify(action));
        },
        sendStop: function() {
            let action = {
                "action": "stop",
                "metadata": null
            };
            this.socket.send(JSON.stringify(action));
        },
        sendSetTime: function() {
            let action = {
                "action": "setTime",
                "metadata": `${this.setTimeTime}`
            };
            this.socket.send(JSON.stringify(action));
        },
        onSetTimeTextChange: function() {
            let min = parseInt($("#set-time-text-min")[0].value);
            let sec = parseInt($("#set-time-text-sec")[0].value);
            this.setTimeTime = min*60 + sec;
        },
        onSetTimeSliderChange: function() {
            this.setTimeTime = parseInt($("#set-time-slider")[0].value);
        }
    }
});

