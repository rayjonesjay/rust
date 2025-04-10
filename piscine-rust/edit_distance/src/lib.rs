pub fn edit_distance(source: &str, target: &str) -> usize {
    let s1: Vec<char> = source.chars().collect();
    let s2: Vec<char> = target.chars().collect();
    let m = s1.len();
    let n = s2.len();

    let mut dp = vec![vec![0; n + 1]; m + 1];

    for i in 0..=m {
        for j in 0..=n {
            if i == 0 {
                dp[i][j] = j;
            } else if j == 0 {
                dp[i][j] = i;
            } else if s1[i - 1] == s2[j - 1] {
                dp[i][j] = dp[i - 1][j - 1];
            } else {
                dp[i][j] = 1 + std::cmp::min(
                    dp[i - 1][j - 1], // replace
                    std::cmp::min(dp[i][j - 1], dp[i - 1][j]) // insert or delete
                );
            }
        }
    }

    dp[m][n]
}
