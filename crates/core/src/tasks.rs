use {async_task::Task, futures::Future};

pub fn spawn_on_thread<F, T>(future: F) -> Task<T>
where
    F: Future<Output = T> + Send + 'static,
    T: Send + 'static,
{
    // Create a channel that holds the task when it is scheduled for running.
    let (sender, receiver) = flume::unbounded();
    let schedule_sender = sender.clone();

    // Wrap the future into one that disconnects the channel on completion.
    let future = async move {
        // When the inner future completes, the sender gets dropped and disconnects the channel.
        let _sender = sender.clone();
        future.await
    };

    // Create a task that is scheduled by sending it into the channel.
    let schedule = move |runnable| schedule_sender.send(runnable).unwrap();
    let (runnable, task) = async_task::spawn(future, schedule);

    // Schedule the task by sending it into the channel.
    runnable.schedule();

    // Spawn a thread running the task to completion.
    std::thread::spawn(move || {
        // Keep taking the task from the channel and running it until completion.
        for runnable in receiver {
            runnable.run();
        }
    });

    task
}
