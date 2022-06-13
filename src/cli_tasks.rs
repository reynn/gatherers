use eyre::eyre;
use {
    crate::{cli::TransactionFormat, config::Config},
    gatherer_core::{
        downloaders::{BatchDownloader, MultiThreadedDownloader},
        gatherers::{self, Gatherer},
        tasks::spawn_on_thread,
        Result,
    },
    std::{collections::HashMap, path::Path, sync::Arc, time::Instant},
};

pub async fn start(
    cur_gatherers: Vec<Arc<dyn Gatherer + 'static>>,
    app_config: &'_ Config,
    worker_count: u8,
    user_names: Vec<String>,
    limit_media: Option<usize>,
    limit_subs: Option<usize>,
    ignored_user_names: Vec<String>,
) -> Result<()> {
    if !cur_gatherers.is_empty() {
        let (tx, rx) = async_channel::unbounded();
        // Start our downloader with our channel receiver
        // TODO: downloader should be configurable, options are there just need codify
        let downloader = MultiThreadedDownloader::new(worker_count, rx);
        // This will be the base path to our downloader, it will be exactly what the user has provided in their config
        let downloads_directory = Path::new(&app_config.download_dir).to_path_buf();
        // holds our configured tasks, they will start at the same time during
        // the join all which will also wait for them to complete
        let mut primary_threads = Vec::new();

        // For each initialized gatherer start a new thread that will run the gatherer logic from start to finish
        for gatherer in cur_gatherers.into_iter() {
            primary_threads.push(spawn_on_thread({
                let base_path = downloads_directory.clone();
                let download_tx = tx.clone();
                let limits = gatherers::RunLimits {
                    media: limit_media,
                    subscriptions: limit_subs,
                };
                let user_names = user_names.clone();
                let mut fansly_ignored_users = app_config.fansly.ignore_lists.clone();
                let mut onlyfans_ignored_users = app_config.onlyfans.ignore_lists.clone();
                let mut ignored_user_names = ignored_user_names.clone();
                ignored_user_names.append(&mut fansly_ignored_users);
                ignored_user_names.append(&mut onlyfans_ignored_users);
                async move {
                    let gatherer_name = gatherer.name();
                    let start_time = Instant::now();
                    // Now that we have everything setup we can hand off the majority of the logic to the main func
                    match gatherers::run_gatherer_for_all(
                        gatherer,
                        base_path,
                        download_tx,
                        limits,
                        &user_names,
                        &ignored_user_names,
                    )
                    .await
                    {
                        Ok(_) => println!(
                            "{gatherer_name}: Finished after {:.2} seconds",
                            Instant::now().duration_since(start_time).as_secs_f64()
                        ),
                        Err(gatherer_err) => {
                            log::error!("{gatherer_name}: Failed to complete. {:?}", gatherer_err)
                        }
                    }
                }
            }));
        }

        // Spawn a new thread to handle downloading our content as it comes in
        primary_threads.push(spawn_on_thread(async move {
            println!("Starting {} downloader..", downloader);
            let start_time = Instant::now();
            // Start the main process function
            match downloader.process_all_items().await {
                Ok(stats) => {
                    let time_taken = Instant::now().duration_since(start_time);
                    log::info!(
                        "Successfully completed downloads: {:?}. Took {:?}",
                        stats,
                        time_taken
                    );
                }
                Err(down_err) => log::error!("Failed to process downloads: {:?}", down_err),
            }
        }));

        // drop our initial send chan so the receiver can properly detect the end
        drop(tx);
        // block our program exit until all of our work is complete
        futures::future::join_all(primary_threads).await;

        Ok(())
    } else {
        Err(eyre!("No gatherers available"))
    }
}

pub async fn purchased(
    cur_gatherers: Vec<Arc<dyn Gatherer + 'static>>,
    app_config: &'_ Config,
) -> Result<()> {
    let (tx, rx) = async_channel::unbounded();
    // Start our downloader with our channel receiver
    // TODO: downloader should be configurable, options are there just need codify
    let downloader = MultiThreadedDownloader::new(app_config.workers, rx);
    // This will be the base path to our downloader, it will be exactly what the user has provided in their config
    let downloads_directory = Path::new(&app_config.download_dir).to_path_buf();
    // holds our configured tasks, they will start at the same time during
    // the join all which will also wait for them to complete
    let mut primary_threads = Vec::new();

    for gatherer in cur_gatherers.into_iter() {
        primary_threads.push(spawn_on_thread({
            let base_path = downloads_directory.clone();
            let download_tx = tx.clone();
            async move {
                let gatherer_name = gatherer.name();
                let start_time = Instant::now();
                // Start building output directory for our gatherer
                let base_path = base_path.clone().join(gatherer.name().to_ascii_lowercase());
                // Now that we have everything setup we can hand off the majority of the logic to the main func
                match gatherers::run_gatherer(gatherers::GathererInfo {
                    base_path,
                    gather_type: gatherers::GatherType::Purchased,
                    gatherer,
                    subscription: Default::default(),
                    downloader: download_tx,
                    name: gatherer_name.to_string(),
                })
                .await
                {
                    Ok(_) => println!(
                        "{gatherer_name}: Finished after {:.2} seconds",
                        Instant::now().duration_since(start_time).as_secs_f64()
                    ),
                    Err(gatherer_err) => {
                        log::error!("{gatherer_name}: Failed to complete. {:?}", gatherer_err)
                    }
                }
            }
        }));
    }

    // Spawn a new thread to handle downloading our content as it comes in
    primary_threads.push(spawn_on_thread(async move {
        println!("Starting {} downloader..", downloader);
        let start_time = Instant::now();
        // Start the main process function
        match downloader.process_all_items().await {
            Ok(stats) => {
                let time_taken = Instant::now().duration_since(start_time);
                log::info!(
                    "Successfully completed downloads: {:?}. Took {:?}",
                    stats,
                    time_taken
                );
            }
            Err(down_err) => log::error!("Failed to process downloads: {:?}", down_err),
        }
    }));

    // drop our initial send chan so the receiver can properly detect the end
    drop(tx);
    futures::future::join_all(primary_threads).await;
    Ok(())
}

pub async fn list(cur_gatherers: Vec<Arc<dyn Gatherer + 'static>>) -> Result<()> {
    if !cur_gatherers.is_empty() {
        let mut primary_threads = Vec::new();
        for gatherer in cur_gatherers.into_iter() {
            primary_threads.push(spawn_on_thread(async move {
                let gatherer_name = gatherer.name();
                match gatherer.gather_subscriptions().await {
                    Ok(subs) => {
                        for sub in subs.into_iter() {
                            println!("{:>12}: {}", gatherer_name, sub.name.username);
                        }
                    }
                    Err(subs_err) => {
                        log::error!(
                            "{:>12}: failed to get subscriptions. {:?}",
                            gatherer_name,
                            subs_err
                        );
                    }
                }
            }))
        }
        futures::future::join_all(primary_threads).await;
        Ok(())
    } else {
        Err(eyre!("No gatherers available"))
    }
}

#[allow(dead_code)]
pub async fn like(_cur_gatherers: Vec<Arc<dyn Gatherer + 'static>>) -> Result<()> {
    Ok(())
}

pub async fn unlike(_cur_gatherers: Vec<Arc<dyn Gatherer + 'static>>) -> Result<()> {
    Ok(())
}

pub async fn transactions(
    cur_gatherers: Vec<Arc<dyn Gatherer + 'static>>,
    user_names: Vec<String>,
    transaction_format: TransactionFormat,
) -> Result<()> {
    if !cur_gatherers.is_empty() {
        let mut gatherer_totals: HashMap<&str, HashMap<String, f64>> = HashMap::new();

        for gatherer in cur_gatherers.into_iter() {
            let mut user_total: HashMap<String, f64> = HashMap::new();
            let transactions = gatherer.gather_transaction_details(&user_names).await;
            match transactions {
                Ok(transactions) => {
                    for transaction in transactions.into_iter() {
                        *user_total
                            .entry(transaction.user_name)
                            .or_insert(transaction.total_amount) += transaction.total_amount;
                    }
                }
                Err(err) => {
                    log::error!(
                        "[{}]: Failed to get transactions. {:?}",
                        gatherer.name(),
                        err
                    )
                }
            }
            gatherer_totals.insert(gatherer.name(), user_total);
        }
        match transaction_format {
            TransactionFormat::Json => {
                println!("{}", serde_json::to_string(&gatherer_totals).unwrap());
            }
            TransactionFormat::PlainText => {
                for (gatherer, totals) in gatherer_totals.into_iter() {
                    println!("Total transaction costs for {gatherer}");
                    for (user_name, total) in totals.into_iter() {
                        println!("{}: {:.2}", user_name, total);
                    }
                }
            }
            TransactionFormat::Table => {}
        };
        Ok(())
    } else {
        Err(eyre!("No gatherers available"))
    }
}
