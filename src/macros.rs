#[macro_export]
macro_rules! add_gatherer {
    ($gatherers:expr, $crat_path:path, $cfg:expr) => {
        match <$crat_path>::new($cfg).await {
            Ok(gatherer) => $gatherers.push(Arc::new(gatherer)),
            Err(err) => log::error!(
                "Failed to initialize: {}. {:?}",
                stringify!($crat_path),
                err
            ),
        }
    };
}
