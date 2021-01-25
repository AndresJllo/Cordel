fn longps(s: &str) -> Vec<usize> {
    let mut pi = vec![0; s.len()];
    let mut i = 1;
    let mut j = 0;
    let strarr = s.as_bytes();
    while i < strarr.len() {
        if strarr[i] == strarr[j] {
            pi[i] = j + 1;
            j += 1;
            i += 1;
        } else {
            if j == 0 {
                pi[i] = 0;
                i += 1;
            } else {
                j = pi[j - 1];
            }
        }
    }

    pi
}

pub fn kmp(needle: &str, haystack: &str) -> usize {
    let psarr = longps(needle);
    let needle = needle.as_bytes();
    let haystack = haystack.as_bytes();

    let mut i: usize = 0;
    let mut j: usize = 0;

    while i < haystack.len() {
        if j == needle.len() - 1 {
            return i - needle.len() + 1;
        }

        if needle[j] != haystack[i] {
            if j == 0 {
                i += 1;
            } else {
                j = psarr[j - 1];
            }
        } else {
            i += 1;
            j += 1;
        }
    }

    return i;
}
