# YewChat 💬

> Source code for [Let’s Build a Websocket Chat Project With Rust and Yew 0.19 🦀](#)

## Install

1. Install the required toolchain dependencies:
   '''npm i'''

2. Follow the YewChat post!

## Branches

This repository is divided to branches that correspond to the blog post sections:

* main - The starter code.
* routing - The code at the end of the Routing section.
* components-part1 - The code at the end of the Components-Phase 1 section.
* websockets - The code at the end of the Hello Websockets! section.
* components-part2 - The code at the end of the Components-Phase 2 section.
* websockets-part2 - The code at the end of the WebSockets-Phase 2 section.

# Experiment 3.1
![alt text](<images/Screenshot 2026-05-19 at 15.16.20.png>)
![alt text](<images/Screenshot 2026-05-19 at 15.18.48.png>)

# Experiment 3.2
I changed the app into a game style chat lobby. The welcome page now feels like a small player entry screen with icon tiles and a stronger arcade layout.
The chat room now supports reactions on other people's messages. Reaction counts are sent through the WebSocket server. The server prevents users from reacting to their own messages or repeating the same reaction.

![alt text](<images/Screenshot 2026-05-19 at 17.56.41.png>)
![alt text](<images/Screenshot 2026-05-19 at 17.58.10.png>)

# Bonus

I modified the Rust WebSocket server from Tutorial 2 in '../timer_future/src/bin/server.rs' so it can serve this Yew webchat. The important change is that the server no longer treats each WebSocket message as plain chat text. It now treats each incoming text frame as serialized JSON, deserializes it, checks the 'messageType', and sends back serialized JSON text using the same format expected by the Tutorial 3 frontend.

The Rust server supports:

* 'register' to save the player's nickname.
* 'users' to broadcast the online player list.
* 'message' to broadcast chat messages with 'id', 'from', 'message', 'time', and 'reactions'.
* 'reaction' to update reaction counts on another player's message.

This is a successful change because the Yew client did not need a protocol rewrite. It still connects to 'ws://127.0.0.1:8080', still sends JSON as text, and still receives the same 'users', 'message', and 'reaction' events. I verified it by running the Rust server with the Yew frontend: two players could join, send chat messages, and react to each other's messages.

Between the JavaScript and Rust versions, I prefer the Rust version for the server. JavaScript is faster to write and easier for quick experiments, but Rust feels better for the backend because the message structures, shared state, and invalid cases are more explicit. The compiler helps catch mistakes before the server runs, which is valuable for real-time features like chat.
