#![cfg(feature = "async-tokio")]

use ffprobe::ConfigBuilder;
use url::Url;
async fn check_url(url: Url) {
    eprintln!("Testing file {}", url);
    ffprobe::ffprobe_async_url(url).await.unwrap();
}

async fn check_count_frames(url: Url) {
    eprintln!("Testing file {}", url);
    let out = ConfigBuilder::new()
        .count_frames(true)
        .run_async(url)
        .await
        .unwrap();

    let stream = out
        .streams
        .iter()
        .find(|s| s.codec_type.clone().unwrap_or_default() == "video")
        .unwrap();

    assert!(stream.nb_read_frames.is_some());
}

#[tokio::test]
async fn download_and_probe_async() {
    let item_urls = [
        // Images.
        "http://commondatastorage.googleapis.com/gtv-videos-bucket/sample/images/BigBuckBunny.jpg",
        // Videos.
        "http://commondatastorage.googleapis.com/gtv-videos-bucket/sample/BigBuckBunny.mp4",
        "https://www.learningcontainer.com/wp-content/uploads/2020/05/sample-avi-file.avi",
        "https://www.learningcontainer.com/wp-content/uploads/2020/05/sample-avi-file.avi",
        "https://www.learningcontainer.com/wp-content/uploads/2020/05/sample-mov-file.mov",
        "https://www.learningcontainer.com/wp-content/uploads/2020/05/sample-mpg-file.mpg",
        "https://www.learningcontainer.com/wp-content/uploads/2020/05/sample-wmv-file.wmv",
        // Audios.
        // TODO: add some audio files
    ];

    tokio::spawn(async move {
        for url in item_urls.iter() {
            check_url(Url::parse(url).unwrap()).await;
        }
    })
    .await
    .expect("tokio spawn failed");

    check_count_frames(Url::parse(*item_urls.last().unwrap()).unwrap()).await;
}
