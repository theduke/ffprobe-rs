use std::path::Path;

use ffprobe::ConfigBuilder;

fn download(url: &str) -> std::path::PathBuf {
    let dir = std::path::PathBuf::from(".test_output");
    if !dir.is_dir() {
        std::fs::create_dir(&dir).unwrap();
    }

    let filename = url
        .replace("http:", ":")
        .replace("https", "")
        .replace('/', "__")
        .replace(':', "__");
    let path = dir.join(filename);

    if !path.is_file() {
        let status = std::process::Command::new("curl")
            .args(&[
                "-H",
                "user-agent: Mozilla/5.0 (Windows NT 10.0; rv:78.0) Gecko/20100101 Firefox/78.0",
                "--insecure",
                "-L",
                "-o",
            ])
            .arg(&path)
            .arg(url)
            .spawn()
            .unwrap()
            .wait()
            .unwrap();

        if !status.success() {
            panic!("Download failed");
        }
    }

    path
}

fn check(path: &Path) {
    eprintln!("Testing file {}", path.display());
    ffprobe::ffprobe(path).unwrap();
}

fn check_count_frames(path: &Path) {
    eprintln!("Testing file {}", path.display());
    let out = ConfigBuilder::new().count_frames(true).run(path).unwrap();

    let stream = out
        .streams
        .iter()
        .find(|s| s.codec_type.clone().unwrap_or_default() == "video")
        .unwrap();

    assert!(stream.nb_read_frames.is_some());
}

#[test]
fn download_and_probe() {
    let item_urls = vec![
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

    let item_paths = item_urls
        .iter()
        .map(|url| download(url))
        .collect::<Vec<_>>();

    for path in &item_paths {
        check(path);
    }

    check_count_frames(item_paths.last().unwrap());
}
