use fbc_scrubber::fbc_chunker::ChunkerFBC;
use fbc_scrubber::frequency_analyser::{FrequencyAnalyser, DictRecord};
use std::fs::{self, File};

use std::hash::{DefaultHasher, Hasher};
use std::io::{BufWriter, Write};
use std::str::FromStr;
use dashmap::DashMap;
use std::sync::{Arc, Mutex, MutexGuard};

fn save_map(file_name: &str, saved_map: Arc<DashMap<u64, DictRecord>>) -> std::io::Result<()>{
    let mut string_out = String::new();
    string_out.push_str(saved_map.len().to_string().as_str());

    for element in saved_map.iter() {
        string_out.push_str("1");
    }

    let file = std::fs::write(file_name, string_out)?;
    Ok(())
}

fn f(name: &str, dt: usize, analize_sizes: Vec<usize>) -> Option<(f64, f64, usize)> {
    let mut analyser = FrequencyAnalyser::new_with_sizes(analize_sizes.clone());
    let mut chunker = ChunkerFBC::default();
    let path_string = "../test_files_input/".to_string() + name;
    let path = std::path::Path::new(path_string.as_str());
    let contents = fs::read(&path)
        .expect("Should have been able to read the file");
    analyser.append_dict(&contents);
    
    let mut i = 0;
    while i < contents.len() - dt {
        chunker.add_cdc_chunk(&contents[i..i + dt]);
        i += dt;
    }
    chunker.add_cdc_chunk(&contents[i..contents.len()]);
    let a = analyser.get_dict();

    // for (k, v) in a.iter() {
    //     if v.get_occurrence_num() > 1 {
    //         println!("hash: {k} size: {} occ_num: {} ", v.get_size(), v.get_occurrence_num());
    //     }
    //     // println!("chnk:\n{:?}", v.get_chunk());
    // }

    let dedup = chunker.fbc_dedup(&a, analyser.get_chunck_partitioning());
    let rededup = chunker.reduplicate("out.txt");
    let pure_size = chunker.get_size_pure_chuncks();
    let count_chuncks = chunker.get_count_chuncks();
    
    println!("dedup: {}", rededup as f64 / dedup as f64);
    println!("dedup: {}", rededup as f64 / pure_size as f64);
    print!("name: {}\ndt: {}\nsizes: ", name, dt);
    for it in analize_sizes {
        print!("{it} ");
    }
    
    if fs::read(path)
            .expect("Should have been able to read lowinput")
        != 
        fs::read("out.txt")
            .expect("Should have been able to read out file")
    {
        let mut name = String::new();
        println!("");
        println!("NOT MATCH {} {}", fs::metadata(path).unwrap().len(), fs::read("out.txt").unwrap().len());
        chunker.reduplicate_by_chuncks("_out.txt");
        std::io::stdin().read_line(&mut name);
        println!("");
        println!("");
        None
    } else {
        Some((rededup as f64 / dedup as f64, rededup as f64 / pure_size as f64, count_chuncks))
    }

}

fn main() {
    let names = [
        "fbc_topic_input.txt",
        "lowinput.txt",
        "orient_express_input.txt",
    ];
    let dts = [
        128 * 2, 128 * 3, 128 * 4, 128 * 5, 128 * 6, 128 * 7, 128 * 8
        // 0        1       2         3         4         5       6
    ];

    let all_sizes = [
        vec![32], vec![64], vec![128], vec![256], 
        //    0         1          2          3
        vec![64, 32], vec![128, 64, 32], vec![128, 64], vec![256, 128, 64], vec![256, 128],
        //      4                5               6                 7               8
        vec![256, 64], vec![128, 32]
        //      9             10
    ];

    // f(names[0], dts[1], all_sizes[7].clone());
    // return;

    let mut str_out = String::from_str("file_name\tdt\tsizes\tdedup_coef\tpure_size_ratio\tcount_chunks\n").unwrap();

    for name in names {
        for dt in dts {
            for sizes in all_sizes.iter() {
                print!("name: {}\ndt: {}\nsizes: ", name, dt);
                for it in sizes {
                    print!("{it} ");
                }
                print!("\n\n");
                str_out.push_str(name);
                str_out.push_str("\t");
                str_out.push_str(dt.to_string().as_str());
                str_out.push_str("\t");
                for s in sizes {
                    str_out.push_str(s.to_string().as_str());
                    str_out.push_str(" ");
                }
                str_out.push_str("\t");

                // if name.to_string() == "lowinput.txt" &&
                //     sizes.len() > 1 {
                //     str_out.push_str("STACK OVERFLOW");
                //     println!("STACK OVERFLOW\n");
                // } else {
                // }
                    match f(name, dt, sizes.clone()) {
                        Some(res) => {
                            str_out.push_str(res.0.to_string().as_str());
                            str_out.push_str("\t");
                            str_out.push_str(res.1.to_string().as_str());
                            str_out.push_str("\t");
                            str_out.push_str(res.2.to_string().as_str());
                        }
                        None => {
                            str_out.push_str("NOT MATCH");
                        }
                    }

                str_out.push_str("\n");
            }
        }
    }
    fs::write("experement_result.csv", str_out.as_bytes()).unwrap();
}
