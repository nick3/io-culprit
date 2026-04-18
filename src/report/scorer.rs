pub fn score_candidate(
    samples_in_top3: i32,
    repeated: bool,
    aligned_with_device: bool,
    high_cpu_or_wait: bool,
) -> i32 {
    let mut score = samples_in_top3 * 4;
    if repeated {
        score += 3;
    }
    if aligned_with_device {
        score += 3;
    }
    if high_cpu_or_wait {
        score += 2;
    }
    score
}
