# Media play server
A project to start several videos at once over several clients with one click of a button. 

## Background
My brother is working on a film project. He wants to use a film effect that requires the ability to start and stop multiple videos at once over several screens and possibly devices. I therefor created a frontend which consists out of a pretty ugly HTML page which simply includes a video play. A HTML frontend in combination with JS allows me to focus on the synchronization while the browser takes care of video buffering, playing and so on.  

The backend is written in Rust, mostly because I love Rust and I wanted to improve my prototyping ability. It uses a mixture of [Rocket](rocket.rs) as an HTTP client to serve the pages and [tungstenite-rs](https://github.com/snapview/tungstenite-rs) for websockets and exchanging message among the connected instances. The backend doesn't validate anything and just passes received string messages to all other connected clients. This could therefor also be used for other synchronization. 

The last part that might need some explanation might be the weird naming of the video players called _Watchers_ and the video control called _Player_. This is purely for fun as I was reminded of the film _Nerve_. The CSS on `/static/index.html` was also inspired by it.

And this is where my journey of hacking together a syncing backend ends for now. There are some changes which probably have to be made, but that's a to do for another time. Have a great day stranger and keep on smiling!

## Usage
1. Add the media files which should be available to the watchers in [`static/mov/`](/static/mov/).
2. Start the backend with `cargo run`
3. Visit `localhost:8000/static/index.html`
4. Done! you are up in running.

## TODOs
* [ ] Add a timestamp to actions to avoid asynchronous behavior for clients with a longer ping
* [ ] Maybe add a connected client overview to the _Player_ (Controller) view
* [ ] Maybe Save the metadata which video is being played in the browser 
* [ ] Maybe Improve CSS

## Disclaimer
This code is neither clean nor efficient and probably pretty insecure. It is not meant to be used as an example or guide but more a testament of how several awesome crates can be hacked together to create a simple prototype. This project was also the first time that I used threads and websockets in Rust. Multi threading in Rust is just awesome, I had no race conditions or anything problematic during the development. So, if you use it, use it at your own risk and please don't use judge me based on this project ^^ 