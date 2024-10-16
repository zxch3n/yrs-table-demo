use std::fs::File;
use std::path::Path;
use std::time::Instant;

use loro::LoroDoc;
use loro_table::LoroTable;
use yrs::{Doc, ReadTxn, StateVector, Transact};

use crate::table::Table;

mod loro_table;
mod table;

fn main() {
    println!("============================");
    println!("Running Yrs...");
    println!("============================");
    run_yrs("./assets/uber.csv");
    println!("\n\n");

    println!("============================");
    println!("Running Loro...");
    println!("============================");
    run_loro("./assets/uber.csv");
}

fn run_yrs<P>(fpath: P)
where
    P: AsRef<Path>,
{
    let mut reader = csv::Reader::from_path(&fpath).unwrap();
    let doc = Doc::new();
    let table = doc.get_or_insert_map("csv-table");
    let mut txn = doc.transact_mut();
    let table = Table::new(table, &mut txn);

    // import csv
    let start = Instant::now();
    let cell_count = table.import(&mut txn, &mut reader);
    drop(txn);
    let elapsed = start.elapsed();
    println!("imported {} cells in {:?}", cell_count, elapsed);

    // encode document state
    let tx = doc.transact();
    let rows = table.row_count(&tx);
    let cols = table.col_count(&tx);
    let start = Instant::now();
    let data = tx.encode_state_as_update_v2(&StateVector::default());
    let compressed = zstd::encode_all(&data[..], 4).unwrap();
    let elapsed = start.elapsed();
    let original_len = { File::open(&fpath).unwrap().metadata().unwrap().len() };
    let start = Instant::now();
    let _new_table = Table::decode(&data).unwrap();
    let import_elapsed = start.elapsed();
    println!(
        "encoded {} cells ({} rows x {} columns) in {:?}: {} bytes, {} compressed (original file size: {} bytes) decoded in {:?}\n",
        cell_count,
        rows,
        cols,
        elapsed,
        data.len(),
        compressed.len(),
        original_len,
        import_elapsed
    );

    // read some data
    {
        let tx = doc.transact();
        for col in table.columns(&tx) {
            print!("{}\t", col.name);
        }
        println!("");
        for row in table.rows(&tx).take(10) {
            for cell in row.raw() {
                print!("{}\t", cell);
            }
            println!("");
        }
    }
}

fn run_loro<P>(fpath: P)
where
    P: AsRef<Path>,
{
    let doc = LoroDoc::new();
    let table = LoroTable::new(doc);

    // import csv
    let mut reader = csv::Reader::from_path(&fpath).unwrap();
    let start = Instant::now();
    let cell_count = table.import(&mut reader).unwrap();
    let elapsed = start.elapsed();
    println!("imported {} cells in {:?}", cell_count, elapsed);

    // encode document state
    let rows = table.row_count();
    let cols = table.col_count();
    let start = Instant::now();
    let data = table.encode();
    let compressed = zstd::encode_all(&data[..], 4).unwrap();
    let elapsed = start.elapsed();
    let original_len = { File::open(&fpath).unwrap().metadata().unwrap().len() };
    let start = Instant::now();
    let _new_table = LoroTable::decode(&data).unwrap();
    let import_elapsed = start.elapsed();
    println!(
        "encoded {} cells ({} rows x {} columns) in {:?}: {} bytes, {} compressed (original file size: {} bytes) decoded in {:?}\n",
        cell_count,
        rows,
        cols,
        elapsed,
        data.len(),
        compressed.len(),
        original_len,
        import_elapsed
    );

    let start = Instant::now();
    let data = table.encode_shallow();
    let compressed = zstd::encode_all(&data[..], 4).unwrap();
    let elapsed = start.elapsed();
    let start = Instant::now();
    let _new_table = LoroTable::decode(&data).unwrap();
    let import_elapsed = start.elapsed();
    println!(
        "[ShallowMode] encoded {} cells ({} rows x {} columns) in {:?}: {} bytes, {} compressed (original file size: {} bytes) decoded in {:?}\n",
        cell_count,
        rows,
        cols,
        elapsed,
        data.len(),
        compressed.len(),
        original_len,
        import_elapsed
    );

    // read some data
    {
        for col in table.cols() {
            print!("{}\t", col.name);
        }
        println!("");
        for row in table.rows().iter().take(10) {
            for cell in row.read_cells(&table) {
                print!("{:?}\t", cell);
            }
            println!("");
        }
    }
}
