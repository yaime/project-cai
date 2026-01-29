use anyhow::{Context, Result};
use image::{imageops::FilterType, GenericImageView, Pixel};
use ndarray::Array4;
use ort::session::{Session, builder::GraphOptimizationLevel};
use std::fs;

pub struct OcrService {
    session: Session,
    keys: Vec<String>,
}

#[derive(serde::Serialize, Debug)]
pub struct TextResult {
    pub text: String,
    pub confidence: f32,
    pub coordinates: Vec<Vec<i32>>, // [[x1, y1], [x2, y1], [x2, y2], [x1, y2]]
}

impl OcrService {
    pub fn new(model_path: &str, keys_path: &str) -> Result<Self> {
        let session = Session::builder()?
            .with_optimization_level(GraphOptimizationLevel::Level3)?
            .with_intra_threads(4)?
            .commit_from_file(model_path)
            .context("Failed to load ONNX model")?;

        let keys_content = fs::read_to_string(keys_path).unwrap_or_default();
        let keys: Vec<String> = keys_content.lines().map(|s| s.to_string()).collect();

        Ok(Self { session, keys })
    }

    pub fn infer(&mut self, image_path: &str) -> Result<TextResult> {
        let img = image::open(image_path).context("Failed to open image")?;
        let (width, height) = img.dimensions();

        // PP-OCR Rec model expects Height 48. Width can be dynamic or fixed.
        let target_height = 48;
        let target_width = (width as f32 / height as f32 * target_height as f32) as u32;
        // Ensure minimum width
        let target_width = target_width.max(32);

        let resized = img.resize_exact(target_width, target_height, FilterType::Triangle);

        let mut input = Array4::<f32>::zeros((1, 3, target_height as usize, target_width as usize));

        for (x, y, pixel) in resized.pixels() {
            let channels = pixel.channels();
            // Normalize: (pixel / 255.0 - 0.5) / 0.5  => (pixel / 255.0) * 2.0 - 1.0
            input[[0, 0, y as usize, x as usize]] = (channels[0] as f32 / 255.0 - 0.5) / 0.5;
            input[[0, 1, y as usize, x as usize]] = (channels[1] as f32 / 255.0 - 0.5) / 0.5;
            input[[0, 2, y as usize, x as usize]] = (channels[2] as f32 / 255.0 - 0.5) / 0.5;
        }

        let shape = vec![1, 3, target_height as usize, target_width as usize];
        let data = input.as_slice().ok_or(anyhow::anyhow!("Input tensor not contiguous"))?;
        let tensor = ort::value::Tensor::from_array((shape, data.to_vec()))?;
        let inputs = ort::inputs![tensor];
        let outputs = self.session.run(inputs)?;

        // Output shape is typically [batch, sequence_length, num_classes]
        let (output_shape, output_data) = outputs[0].try_extract_tensor::<f32>()?;
        let seq_len = output_shape[1] as usize;
        let num_classes = output_shape[2] as usize;

        let mut text = String::new();
        let mut last_index = 0; // 0 is usually blank in CTC
        let mut total_conf = 0.0;
        let mut char_count = 0;

        // Simple greedy decoder
        for t in 0..seq_len {
            // Find argmax for this time step
            let mut max_score = -f32::INFINITY;
            let mut max_idx = 0;

            for c in 0..num_classes {
                let idx = t * num_classes + c;
                let score = output_data[idx];
                if score > max_score {
                    max_score = score;
                    max_idx = c;
                }
            }

            if max_idx != 0 && max_idx != last_index {
                // If keys are available, map index to char.
                // Note: PP-OCR keys file excludes the blank token (0). So index 1 in output maps to keys[0].
                // But check if model includes blank at 0 or end. Usually 0.
                let char_idx = max_idx - 1;
                if char_idx < self.keys.len() {
                    text.push_str(&self.keys[char_idx]);
                } else {
                    // Fallback if keys missing or index out of bounds
                    text.push('?');
                }
                total_conf += max_score; // Note: this is logit or prob? Rec output is usually softmax prob.
                // If it's raw logits, we should apply softmax. PP-OCR output is usually Softmax.
                char_count += 1;
            }
            last_index = max_idx;
        }

        let confidence = if char_count > 0 {
            total_conf / char_count as f32
        } else {
            0.0
        };

        // Dummy coordinates for Rec model (full image box)
        let coordinates = vec![
            vec![0, 0],
            vec![width as i32, 0],
            vec![width as i32, height as i32],
            vec![0, height as i32],
        ];

        Ok(TextResult {
            text,
            confidence,
            coordinates,
        })
    }
}
