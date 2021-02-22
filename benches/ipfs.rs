use fuzzr::data::{
    content::ContentItem,
    ipfs_client::{IpfsClient, IpfsClientRef},
};
use fuzzr::data::{
    content::TextContent,
    ipfs_ops::{load_file, store_file},
};

use async_std::{
    sync::{Arc, RwLock},
    task::block_on,
};
use criterion::{
    criterion_group, criterion_main, AxisScale, Criterion, PlotConfiguration, Throughput,
};
use tempfile::tempdir;

use std::{
    error::Error,
    fs::File,
    io::Write,
    path::{Path, PathBuf},
};

/// Helper to create file in a directory and return full path.
fn write_file<P>(dir: P, data: &[u8], file_name: &str) -> Result<PathBuf, Box<dyn Error>>
where
    P: AsRef<Path>,
{
    let path = dir.as_ref().join(file_name);
    let mut file = File::create(&path)?;
    file.write_all(data)?;
    Ok(path)
}

fn new_client() -> Result<IpfsClientRef, Box<dyn Error>> {
    block_on(async {
        Ok(Arc::new(RwLock::new(
            IpfsClient::new()
                .await
                .map_err(|e| Arc::try_unwrap(e).unwrap())?,
        )))
    })
}

fn criterion_benchmark_ipfs_text(c: &mut Criterion) {
    const KB: usize = 1024;
    let dir = tempdir().unwrap();
    let client_ref = new_client().unwrap();

    {
        let plot_config = PlotConfiguration::default().summary_scale(AxisScale::Logarithmic);
        let mut group = c.benchmark_group("load_text_throughput");
        group.plot_config(plot_config);

        for size in [KB, 16 * KB, 256 * KB, 1000 * KB].iter() {
            let bees = vec![66; *size];
            let file_name = format!("B_{}.txt", size);
            let path = write_file(dir.path(), bees.as_slice(), &file_name).unwrap();

            let client_ref_c = client_ref.clone();
            let cid = block_on(store_file(path, client_ref_c))
                .unwrap()
                .unwrap()
                .to_string();

            let client_ref = client_ref.clone();
            group.throughput(Throughput::Bytes(*size as u64));
            group.bench_with_input(format!("{}_bytes", size), &cid, |b, cid| {
                b.iter(|| {
                    if let ContentItem::Text(TextContent { string }, _) =
                        block_on(load_file(cid.clone(), client_ref.clone())).unwrap()
                    {
                        assert!(string.starts_with('B'));
                        assert_eq!(string.len(), *size);
                    }
                })
            });
        }
        group.finish();
    }

    {
        let plot_config = PlotConfiguration::default().summary_scale(AxisScale::Logarithmic);
        let mut group = c.benchmark_group("store_text_throughput");
        group.plot_config(plot_config);

        for size in [KB, 16 * KB, 256 * KB, 1000 * KB].iter() {
            let tees = vec![84; *size];
            let file_name = format!("T_{}.txt", size);
            let path = write_file(dir.path(), tees.as_slice(), &file_name).unwrap();

            group.throughput(Throughput::Bytes(*size as u64));
            group.bench_with_input(format!("{}_bytes", size), &path, |b, path| {
                b.iter(|| {
                    let cid = block_on(store_file(path.clone(), client_ref.clone()))
                        .unwrap()
                        .unwrap()
                        .to_string();
                    assert!(!cid.is_empty());
                })
            });
        }
        group.finish();
    }
}

criterion_group!(benches, criterion_benchmark_ipfs_text);
criterion_main!(benches);
