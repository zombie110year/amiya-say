//! # 模糊集匹配相近字符串

use triple_accel::levenshtein_exp;

/// 从 candidates 中匹配出最接近 target 的字符串
pub fn fuzzymatch(target: &str, candidates: Vec<&str>) -> String {
    let target_bytes = target.as_bytes();
    let candidates_bytes: Vec<&[u8]> = candidates.iter().map(|b| b.as_bytes()).collect();

    let mut min_distance = u32::MAX;
    let mut similar_string = String::new();
    for (cand, cand_bytes) in candidates.iter().zip(candidates_bytes.iter()) {
        let dist = levenshtein_exp(cand_bytes, &target_bytes);
        if dist < min_distance {
            min_distance = dist;
            similar_string = cand.to_string();
        }
    }
    return similar_string;
}
