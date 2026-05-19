## Experiment 1.2
![alt text](<images/Screenshot 2026-05-19 at 13.43.05.png>)

For this experiment, the "hey hey" prints before "howdy!", even though the "howdy!" code is written earlier in the file. This happens because Futures in Rust are lazy. They do not execute immediately when created.
<br>When we use 'spawner.spawn(...)', we are simply packaging your code and handing it off to a waiting line, known as a channel. Because the main program does not pause to execute that packaged code, it instantly moves down to the next available instruction. This is exactly why the console prints "hey hey" before anything else happens. The packaged task remains idle in the queue until a dedicated manager, the executor, is explicitly told to start working. Calling 'executor.run()' at the very bottom of your script finally activates this manager. Once activated, the executor pulls your task from the queue and runs it, which is why "howdy!" appears last.

## Experiment 1.3
![alt text](<images/Screenshot 2026-05-19 at 13.57.12.png>)
![alt text](<images/Screenshot 2026-05-19 at 13.57.57.png>)

When we spawn multiple tasks, it allows us to see Rust's single-thread concurrency. The main thread queues all three tasks instantly, printing "hey hey" before waiting for anything. Then, the executor takes over. It pulls the first task, prints "howdy!", hits the 2-second timer, and immediately yields control. Because it yielded, the executor doesn't wait. It pulls the "howdy2!" and "howdy3!", starting their timers too. All three tasks wait simultaneously in the background and wake up to print their "done!" messages at roughly the exact same time.

In the second part of the experiment where the 'drop(spawner)' is removed, the terminal freezes after finishing the tasks. This happens because the executor uses a continuous loop that blocks the thread while waiting for new messages to enter the channel. In Rust, a channel only closes when all of its senders (Spawner objects) are destroyed. By removing the 'drop(spawner)', the main spawner remains active. The executor assumes more tasks might be coming, so it waits forever causing the program to hang. Explicitly calling drop destroys the sender, officially closing the channel and allowing the program to exit cleanly.