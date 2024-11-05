pub fn os_thread(url_count: usize, worker_count: usize) {
    let stuff = vec!["https://yral.com"; url_count];

    let (mut tx, rx) = spmc::channel();

    std::thread::spawn(move || {
        for item in stuff.into_iter() {
            tx.send(item).expect("Couldn't send item over");
        }
    });

    let mut set = Vec::new();
    for _ in 0..worker_count {
        let rx = rx.clone();
        let handle = std::thread::spawn(move || {
            while let Ok(item) = rx.recv() {
                let res = reqwest::blocking::get(item)
                    .expect("COuldn't connect")
                    .text()
                    .expect("couldn't grab text")
                    .len();
                println!("content length = {res}");
            }
        });

        set.push(handle)
    }

    for handle in set.into_iter() {
        handle.join().expect("Couldn't join with the thread");
    }
}

pub async fn tokio_async(url_count: usize, worker_count: usize) {
    let stuff = vec!["https://yral.com"; url_count];

    let (tx, rx) = async_channel::unbounded();

    tokio::task::spawn(async move {
        for item in stuff.into_iter() {
            tx.send(item).await.expect("COuldn't send item over");
        }
    });

    let mut set = tokio::task::JoinSet::new();
    for _ in 0..worker_count {
        let rx = rx.clone();
        set.spawn(async move {
            while let Ok(item) = rx.recv().await {
                let res = reqwest::get(item)
                    .await
                    .expect("COuldn't connect")
                    .text()
                    .await
                    .expect("couldn't grab text")
                    .len();
                println!("content length = {res}");
            }
        });
    }

    let _result = set.join_all().await;
}

#[cfg(test)]
mod test {
    use crate::*;
    const URL_COUNT: usize = 10;
    const WORKER_COUNT: usize = 30;

    #[test]
    fn run_with_os_thread() {
        os_thread(URL_COUNT, WORKER_COUNT);
    }

    #[tokio::test]
    async fn run_with_tokio_workers() {
        tokio_async(URL_COUNT, WORKER_COUNT).await;
    }
}
