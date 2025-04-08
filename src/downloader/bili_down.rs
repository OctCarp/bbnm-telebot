use bilirust::{Client, FNVAL_MP4, VIDEO_QUALITY_720P, VideoUrl};

use anyhow::{Context, Result};
use futures::stream::TryStreamExt;
use reqwest::header::CONTENT_LENGTH;
use std::path::Path;
use tokio::io::{AsyncReadExt, AsyncWriteExt, BufReader, BufWriter};
use tokio_util::io::StreamReader;

const USER_AGENT: &str = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/98.0.4758.80 Safari/537.36";

/// Download Bilibili BV video
pub async fn down_vu(download_urls: &VideoUrl, filename: &str) -> Result<()> {
    if Path::new(&filename).exists() {
        anyhow::bail!("File existed: {}", filename);
    }

    let download_url = &download_urls
        .durl
        .first()
        .context("Unable to find download address")?
        .url;
    log::debug!("Start downloading: {}", filename);
    download_to_file(download_url, &filename, "Downloading").await?;
    log::debug!("Download success: {}", filename);

    Ok(())
}

pub async fn down_bv(bv: String) -> Result<()> {
    let client = Client::new();
    log::info!("Match BV number: {}", bv);
    let info = client
        .bv_info(bv.clone())
        .await
        .context("Get video info failed")?;

    log::info!("Video title: {}", info.title);

    let format = FNVAL_MP4;
    let download_urls = client
        .bv_download_url(bv.clone(), info.cid, format, VIDEO_QUALITY_720P)
        .await
        .context("Get resource url failed")?;

    let filename = format!("./temp/bv/{}.mp4", &bv);

    if Path::new(&filename).exists() {
        anyhow::bail!("File existed: {}", filename);
    }

    let download_url = &download_urls
        .durl
        .first()
        .context("Unable to find download address")?
        .url;
    log::debug!("Start downloading: {}", filename);
    download_to_file(download_url, &filename, "Downloading").await?;
    log::debug!("Download success: {}", filename);

    Ok(())
}

/// Download file from URL
async fn download_to_file(url: &str, path: &str, _title: &str) -> Result<()> {
    let path = Path::new(path);
    let resp = request_resource(url).await?;

    // let checkpoint = 0;
    // let total_size = get_content_length(&resp);

    let (resp, file) = (
        resp,
        tokio::fs::File::create(path)
            .await
            .context("Creat file failed")?,
    );

    let mut writer = BufWriter::with_capacity(1 << 18, file);
    let mut reader = BufReader::with_capacity(
        1 << 18,
        StreamReader::new(resp.bytes_stream().map_err(convert_error)),
    );

    let (sender, mut receiver) = tokio::sync::mpsc::channel::<Vec<u8>>(16);
    let reader_task = tokio::spawn(async move {
        let mut buffer = vec![0u8; 1 << 18];
        while let Ok(read_bytes) = reader.read(&mut buffer).await {
            if read_bytes == 0 {
                break;
            }
            if sender.send(buffer[..read_bytes].to_vec()).await.is_err() {
                break;
            }
        }
    });

    let writer_task = tokio::spawn(async move {
        while let Some(chunk) = receiver.recv().await {
            writer
                .write_all(&chunk)
                .await
                .context("Failed to write file")?;
        }
        writer.flush().await.context("Failed to flush file")
    });

    let (read_res, write_res) = tokio::join!(reader_task, writer_task);
    read_res.context("Read thread exception")?;
    write_res.context("Write thread exception")?;

    Ok(())
}

/// Request resource
async fn request_resource(url: &str) -> Result<reqwest::Response> {
    reqwest::Client::new()
        .get(url)
        .header("user-agent", USER_AGENT)
        .header("referer", "https://www.bilibili.com")
        .send()
        .await?
        .error_for_status()
        .context("Resource request failed")
}

/// Request resource with range (resume download)
async fn request_resource_range(url: &str, start: u64) -> Result<reqwest::Response> {
    reqwest::Client::new()
        .get(url)
        .header("user-agent", USER_AGENT)
        .header("referer", "https://www.bilibili.com")
        .header("Range", format!("bytes={}-", start))
        .send()
        .await?
        .error_for_status()
        .context("resume download failed")
}

/// Extract Content-Length
fn get_content_length(resp: &reqwest::Response) -> u64 {
    resp.headers()
        .get(CONTENT_LENGTH)
        .and_then(|v| v.to_str().ok())
        .and_then(|s| s.parse().ok())
        .unwrap_or(0)
}

/// Convert reqwest error to std::io::Error
fn convert_error(err: reqwest::Error) -> std::io::Error {
    std::io::Error::new(std::io::ErrorKind::Other, err)
}
