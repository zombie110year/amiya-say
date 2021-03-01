//! # 模糊集匹配相近字符串

/// 从 candidates 中匹配出最接近 target 的字符串
pub fn fuzzymatch(target: String, candidates: Vec<String>) -> String {
    let target_vec: Vec<f32> = target
        .as_bytes()
        .iter()
        .map(|b| b.to_owned() as f32)
        .collect();
    let candidates_vec: Vec<Vec<f32>> = candidates
        .iter()
        .map(|s| s.as_bytes().iter().map(|b| b.to_owned() as f32).collect())
        .collect();

    let mut min_distance = f32::MAX;
    let mut similar_string = String::new();
    for (cand, cand_vec) in candidates.iter().zip(candidates_vec.iter()) {
        let dist = distance_between_vec(cand_vec, &target_vec);
        if dist < min_distance {
            min_distance = dist;
            similar_string = cand.to_owned();
        }
    }
    return similar_string;
}

fn distance_between_vec(v1: &Vec<f32>, v2: &Vec<f32>) -> f32 {
    let delta: Vec<f32> = v1.iter().zip(v2.iter()).map(|(a, b)| a - b).collect();
    let distance = delta.iter().map(|x| x.powf(2.0)).sum::<f32>().sqrt();
    return distance;
}
