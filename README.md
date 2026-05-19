## Experiment 2.1
![alt text](<images/Screenshot 2026-05-19 at 14.21.35.png>)
![alt text](<images/Screenshot 2026-05-19 at 14.21.42.png>)
![alt text](<images/Screenshot 2026-05-19 at 14.21.48.png>)
![alt text](<images/Screenshot 2026-05-19 at 14.21.54.png>)

To run this application, I first needed to open multiple terminal windows to act as the different components of the chat system. In the first terminal, I executed 'cargo run --bin server' to start the websocket server. As shown in the server screenshot, it successfully began listening on port 2000. Then, I opened three additional terminal tabs and ran 'cargo run --bin client' in each one. The server terminal immediately registered these new connections, logging the unique IP and port for each client. Meanwhile, each client successfully connected and received the initial "Welcome to chat! Type a message" prompt from the server.
<br>Once all the clients were connected, I tested the broadcast functionality by typing messages into the different client terminals. For example, when I typed "howdyyy" in the first client and pressed enter, the message was captured by the server and immediately broadcasted out. Looking at the client screenshots, you can see that the message "From server: howdyyy" appeared in all the other connected client windows. The same happened when I typed "helloooo" and "heyheyhey" in the subsequent clients. This demonstrates that the 'tokio::select!' loops are working correctly. The server is concurrently listening for incoming text from any individual client and immediately pushing that text out to the shared broadcast channel, updating all clients in real time.

