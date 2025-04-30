

// src/ml.rs

use std::fs::File;
use std::io::BufReader;
use csv::{ReaderBuilder, StringRecord}; // Import StringRecord
use linfa::prelude::*;
use linfa_trees::DecisionTree;
use linfa_logistic::LogisticRegression;
use linfa_logistic::FittedLogisticRegression;
use linfa_linear::FittedLinearRegression;
use linfa_clustering::{KMeans, KMeansParams};
use linfa_nn::distance::L2Dist;
use ndarray::{Array2, Array1};
use rand::thread_rng;
use std::error::Error;

pub fn train_models(features: Array2<f64>, labels: Array1<usize>) -> Result<(DecisionTree<f64, usize>, FittedLogisticRegression<f64, usize>, FittedLinearRegression<f64>, KMeans<f64, L2Dist>), Box<dyn Error>> {
    let continuous_labels: Array1<f64> = labels.mapv(|x| x as f64);

    let dataset_classification = Dataset::from((features.clone(), labels));
    let dataset_regression = Dataset::from((features.clone(), continuous_labels));
    let dataset_clustering = Dataset::from(features.clone());

    let decision_tree_model = DecisionTree::params().fit(&dataset_classification)?;
    let logistic_model = LogisticRegression::default().fit(&dataset_classification)?;
    let linear_model = linfa_linear::LinearRegression::default().fit(&dataset_regression)?;
    let kmeans_model = KMeansParams::new(2, thread_rng(), L2Dist).fit(&dataset_clustering)?;

    Ok((decision_tree_model, logistic_model, linear_model, kmeans_model))
}

pub fn read_data_from_csv(file_path: &str) -> Result<(Array2<f64>, Array1<usize>, Vec<StringRecord>), Box<dyn Error>> { // Modified return type
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let mut rdr = ReaderBuilder::new().from_reader(reader);

    let mut features_vec: Vec<Vec<f64>> = Vec::new();
    let mut labels_vec: Vec<usize> = Vec::new();
    let mut records: Vec<StringRecord> = Vec::new(); // Vec to store StringRecords

    let mut first_row = true;
    for result in rdr.records() {
        match result {
            Ok(record) => {
                println!("Record: {:?}", record); // Debug print
                if first_row {
                    first_row = false;
                    continue;
                }
                let unsafe_block: f64 = record.get(0).unwrap().parse().unwrap();
                let path_traversal: f64 = record.get(1).unwrap().parse().unwrap();
                let command_injection: f64 = record.get(2).unwrap().parse().unwrap();

                // Handle missing columns based on your dataset.csv content.
                // If you are using the dataset.csv file with only 4 columns,
                // remove the function_count and clippy_warnings handling.
                let label: usize = match record.get(3).unwrap() {
                    "safe" => 0,
                    "unsafe" => 1,
                    _ => return Err("Invalid label".into()),
                };

                features_vec.push(vec![unsafe_block, path_traversal, command_injection]);
                labels_vec.push(label);
                records.push(record.clone()); // Store the StringRecord
            }
            Err(e) => {
                eprintln!("Error reading record: {}", e);
            }
        }
    }

    println!("Features Vec: {:?}", features_vec); // Debug print
    println!("Labels Vec: {:?}", labels_vec); // Debug print

    let features_array: Array2<f64> = Array2::from_shape_vec(
        (features_vec.len(), features_vec[0].len()),
        features_vec.into_iter().flatten().collect(),
    )?;

    let labels_array: Array1<usize> = Array1::from_vec(labels_vec);

    return Ok((features_array, labels_array, records)); // Return the StringRecords
}
