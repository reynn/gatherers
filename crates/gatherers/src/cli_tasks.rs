use crate::{config::Config, get_available_gatherers};
use gatherer_core::{
    downloaders::{BatchDownloader, MultiThreadedDownloader},
    gatherers::{self, Gatherer},
    tasks::spawn_on_thread,
    Result,
};
use std::{path::Path, sync::Arc, time::Instant};

pub async fn cli_task_gatherers_run(
    cur_gatherers: Vec<Arc<dyn Gatherer + 'static>>,
    app_config: Arc<Config>,
    worker_count: u8,
    user_names: Vec<String>,
    limit_media: usize,
    limit_subs: usize,
) -> Result<()> {
    if !cur_gatherers.is_empty() {
        println!("Gathering data");
        run_gatherers(
            worker_count,
            limit_media,
            limit_subs,
            user_names,
            app_config,
            cur_gatherers,
        )
        .await;
        Ok(())
    } else {
        Err("No gatherers configured to be used".into())
    }
}

pub async fn cli_task_gatherers_list() -> Result<()> {
    Ok(())
}

pub async fn cli_task_gatherers_like() -> Result<()> {
    Ok(())
}

pub async fn cli_task_gatherers_unlike() -> Result<()> {
    Ok(())
}

async fn run_gatherers(
    worker_count: u8,
    media: usize,
    subscriptions: usize,
    user_names: Vec<String>,
    config: Arc<Config>,
    gatherers: Vec<Arc<dyn Gatherer>>,
) {
    let (tx, rx) = async_channel::unbounded();
    // Start our downloader with our channel receiver
    // TODO: downloader should be configurable, options are there just need codify
    let downloader = MultiThreadedDownloader::new(worker_count, rx);
    // This will be the base path to our downloader, it will be exactly what the user has provided in their config
    let downloads_directory = Path::new(&config.download_dir).to_path_buf();
    // holds our configured tasks, they will start at the same time during
    // the join all which will also wait for them to complete
    let mut primary_threads = Vec::new();

    // For each initialized gatherer start a new thread that will run the gatherer logic from start to finish
    for gatherer in gatherers.into_iter() {
        primary_threads.push(spawn_on_thread({
            let base_path = downloads_directory.clone();
            let download_tx = tx.clone();
            let limits = gatherers::RunLimits {
                media,
                subscriptions,
            };
            let user_names = user_names.clone();
            async move {
                let gatherer_name = gatherer.name();
                let start_time = Instant::now();
                println!(
                    "{}: Starting to gather for all subscriptions.",
                    gatherer_name
                );
                // Now that we have everything setup we can hand off the majority of the logic to the main func
                match gatherers::run_gatherer_for_all(
                    gatherer,
                    base_path,
                    download_tx,
                    limits,
                    &user_names,
                )
                .await
                {
                    Ok(_) => println!(
                        "{}: Finished after {:.2} seconds",
                        gatherer_name,
                        Instant::now().duration_since(start_time).as_secs_f64()
                    ),
                    Err(gatherer_err) => {
                        log::error!("{}: Failed to complete. {:?}", gatherer_name, gatherer_err)
                    }
                }
            }
        }));
    }

    // Spawn a new thread to handle downloading our content as it comes in
    primary_threads.push(spawn_on_thread(async move {
        println!("Starting downloader..");
        let start_time = Instant::now();
        // Start the main process function
        match downloader.process_all_items().await {
            Ok(stats) => log::info!(
                "Successfully completed downloads: {:?}. Took {:.2} seconds",
                stats,
                Instant::now().duration_since(start_time).as_secs_f32()
            ),
            Err(down_err) => log::error!("Failed to process downloads: {:?}", down_err),
        }
    }));

    // drop our initial send chan so the receiver can properly detect the end
    drop(tx);
    // block our program exit until all of our work is complete
    futures::future::join_all(primary_threads).await;
}
