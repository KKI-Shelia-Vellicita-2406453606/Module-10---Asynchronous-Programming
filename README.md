## Experiment 2.1
![alt text](<images/Screenshot 2026-05-19 at 14.21.35.png>)
![alt text](<images/Screenshot 2026-05-19 at 14.21.42.png>)
![alt text](<images/Screenshot 2026-05-19 at 14.21.48.png>)
![alt text](<images/Screenshot 2026-05-19 at 14.21.54.png>)

To run this application, I first needed to open multiple terminal windows to act as the different components of the chat system. In the first terminal, I executed 'cargo run --bin server' to start the websocket server. As shown in the server screenshot, it successfully began listening on port 2000. Then, I opened three additional terminal tabs and ran 'cargo run --bin client' in each one. The server terminal immediately registered these new connections, logging the unique IP and port for each client. Meanwhile, each client successfully connected and received the initial "Welcome to chat! Type a message" prompt from the server.
<br>Once all the clients were connected, I tested the broadcast functionality by typing messages into the different client terminals. For example, when I typed "howdyyy" in the first client and pressed enter, the message was captured by the server and immediately broadcasted out. Looking at the client screenshots, we can see that the message "From server: howdyyy" appeared in all the other connected client windows. The same happened when I typed "helloooo" and "heyheyhey" in the subsequent clients. This demonstrates that the 'tokio::select!' loops are working correctly. The server is concurrently listening for incoming text from any individual client and immediately pushing that text out to the shared broadcast channel, updating all clients in real time.

## Experiment 2.2

The goal is to change the WebSocket communication port from the default 2000 to 8080. To achieve this successfully, we must modify the code in both the server and the client files. In the server file, we need to update the TcpListener::bind function so that it listens on "127.0.0.1:8080". Similarly, in the client file, we must update the ClientBuilder::from_uri string to target "ws://127.0.0.1:8080". If we only update one side, the application will break because the client and server must agree on the exact same location to communicate. A mismatch would result in a "connection refused" error when the client attempts to dial an inactive endpoint. Both files continue to utilize the exact same standard WebSocket protocol, just routed through the newly designated port.

## Experiment 2.3
![alt text](<images/Screenshot 2026-05-19 at 14.48.50.png>)
![alt text](<images/Screenshot 2026-05-19 at 14.48.56.png>)
![alt text](<images/Screenshot 2026-05-19 at 14.49.03.png>)
![alt text](<images/Screenshot 2026-05-19 at 14.49.10.png>)

In this experiment, the goal was to identify who sent a message by attaching their IP address and port number to their chat text. To achieve this, I modified the 'handle_connection function' in the server code to actively use the client's SocketAddr variable. Instead of just broadcasting the raw incoming text, I used the 'format!' macro to prepend the sender's address to their message. The server then sends this newly combined string into the shared broadcast channel for all connected clients to receive. Because of this change, the server no longer sends anonymous messages, ensuring everyone knows exactly where each chat originated. For example, instead of just displaying "hello!", all clients will now clearly see a stamp like "127.0.0.1:54608: hello!" on their screens.
