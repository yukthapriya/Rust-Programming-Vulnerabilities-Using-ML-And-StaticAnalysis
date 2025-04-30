// src/main.rs

use std::collections::HashMap;
use std::path::Path;
use linfa::prelude::Predict;
use ndarray::ArrayView;
use linfa::metrics::ToConfusionMatrix;
use csv::StringRecord; // Import StringRecord

mod ml;

fn main() {
    let metadata_path = Path::new("dataset.csv");

    if !metadata_path.exists() {
        println!("dataset.csv not found in the project root. Please ensure it exists.");
        return;
    } else {
        println!("dataset.csv found in the project root.");
    }

    if let Ok((features, labels, _records)) = ml::read_data_from_csv(metadata_path.to_str().unwrap()) {
        println!("Features Vec: {:?}", features);
        println!("Labels Vec: {:?}", labels);

        if features.is_empty() || labels.is_empty() {
            eprintln!("Error: Features or labels are empty.");
            return;
        }

        if let Ok((decision_tree_model, logistic_model, linear_model, kmeans_model)) = ml::train_models(features.clone(), labels.clone()) {
            let new_features_array = features.slice(ndarray::s![0..2, ..]).to_owned();
            let new_features_view: ArrayView<f64, ndarray::Dim<[usize; 2]>> = new_features_array.view();

            let decision_tree_prediction = decision_tree_model.predict(new_features_view);
            let logistic_prediction = logistic_model.predict(new_features_view);
            let linear_prediction = linear_model.predict(new_features_view);
            let kmeans_prediction = kmeans_model.predict(new_features_view);

            println!("Decision Tree Predictions: {:?}", decision_tree_prediction.targets());
            println!("Logistic Regression Predictions: {:?}", logistic_prediction.targets());
            println!("Linear Regression Predictions: {:?}", linear_prediction.targets());
            println!("K-Means Cluster Assignments: {:?}", kmeans_prediction.targets());

            // Create vulnerability name mapping
            let vulnerability_names: HashMap<usize, &str> = HashMap::from([
                (0, "Buffer Overflow"),
                (1, "SQL Injection"),
                (2, "Cross-Site Scripting"),
                (3, "Command Injection"),
                (4, "Path Traversal"),
                (5, "Memory Leak"),
                // Add more mappings as needed
            ]);

            println!("Vulnerability Report:");
            for (i, &prediction) in decision_tree_prediction.targets().iter().enumerate() {
                let vulnerability = if prediction == 1 {
                    match vulnerability_names.get(&i) {
                        Some(name) => *name,
                        None => "Unsafe",
                    }
                } else {
                    "Safe"
                };
                println!("Data Point {}: Predicted as {}", i + 1, vulnerability);
            }

            for (i, &prediction) in logistic_prediction.targets().iter().enumerate() {
                let vulnerability = if prediction == 1 {
                    match vulnerability_names.get(&i) {
                        Some(name) => *name,
                        None => "Unsafe",
                    }
                } else {
                    "Safe"
                };
                println!("Data Point {}: Predicted as {}", i + 1, vulnerability);
            }

            for (i, &prediction) in kmeans_prediction.targets().iter().enumerate() {
                println!("Data Point {}: Assigned to Cluster {}", i + 1, prediction);
            }

            println!("Linear Regression Values: {:?}", linear_prediction.targets());

            // Model Evaluation
            println!("\nModel Evaluation:");

            let decision_tree_confusion = decision_tree_prediction.confusion_matrix(&labels.slice(ndarray::s![0..2])).expect("Failed to create confusion matrix");
            println!("Decision Tree: Accuracy: {}, Precision: {}, Recall: {}, F1-score: {}",
                     decision_tree_confusion.accuracy(), decision_tree_confusion.precision(), decision_tree_confusion.recall(), decision_tree_confusion.f1_score());

            let logistic_confusion = logistic_prediction.confusion_matrix(&labels.slice(ndarray::s![0..2])).expect("Failed to create confusion matrix");
            println!("Logistic Regression: Accuracy: {}, Precision: {}, Recall: {}, F1-score: {}",
                     logistic_confusion.accuracy(), logistic_confusion.precision(), logistic_confusion.recall(), logistic_confusion.f1_score());

        } else {
            eprintln!("Failed to train models");
        }
    } else {
        eprintln!("Failed to read data from CSV");
    }
}
