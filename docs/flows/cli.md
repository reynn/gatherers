# CLI Flow

```mermaid
sequenceDiagram
  participant usr as User
  participant cli as CLI
  participant cfg as Config
  participant dwn as Downloader
  participant gth as Gatherers
  participant usf as User System
  participant exs as External Service

  cli-->cfg: LoadConfigOrDefault(file_path)
  cfg-->usf: let file_contents = ReadFile(file_path)
  %%{config: { 'fontFamily': 'Hasklig', 'fontSize': 12, 'fontWeight': 400} }%%
  alt is-ok
    cfg-->cli: let cfg = Config::parse(file_contents)
  else is-err
    alt is-recoverable
      cfg-->cli: let cfg = Config::default()
    else is-hard-failure
      cfg-->usr: log_and_exit(err, exit_code = 1)
    end
  end
  cli-->dwn: Downloader::new(cfg)
  alt is-ok
    dwn-->cli: let dwnloader = Arc<impl Downloader>
  else is-err
    alt is-recoverable
      dwn-->cli: initialize a default downloader with values based on system
    else is-hard-failure
      dwn-->usr: log_and_exit(err, exit_code = 1)
    end
  end
  cli-->gth: Gatherers::Init(cfg)
  alt is-ok
    gth-->cli: let gatherers = Vec<Arc<impl Gatherer>>
  else is-err
    alt is-recoverable
      gth-->cli: report error to user
    else is-hard-failure
      gth-->usr: log_and_exit(err, exit_code = 1)
    end
  end
  par start downloader and start waiting for incoming msgs
    rect rgba(0, 0, 255, .1)
      cli--)dwn: dwnloader.start_processing_items()
      Note over cli,dwn: Start running the downloader on a background thread
      dwn--)cli: Downloader will complete based on the implemented strategy
    end # end of rect rgba()
  and get a list of subscriptions from the gatherer source, retrieve data and send to downloader
    rect rgba(0, 0, 255, .1)
      cli--)gth: Gatherer::run_gatherer_for_all_subs(gatherers, tx_chan, cfg)
      Note over cli,gth: Starts running the gatherers on a background thread
      gth-->-cli: Gatherers completed
    end # end of rect rgba()
  end # end of par
```
