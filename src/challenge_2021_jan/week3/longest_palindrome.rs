// Longest Palindromic Substring
// https://leetcode.com/explore/challenge/card/january-leetcoding-challenge-2021/581/week-3-january-15th-january-21st/3609/

struct Solution;

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let src: Vec<char> = s.chars().collect();
        let mut radii = vec![0; 2 * src.len() - 1];
        let mut center = 0;
        let mut radius = 0;
        let mut max_radius = 0;
        let mut max_center = 0;
        while center < radii.len() {
            if (center - radius) % 2 != 0 {
                radius += 1;
            }
            while center >= radius
                && center + radius < radii.len()
                && src[(center - radius + 1) / 2]
                    == src[(center + radius + 1) / 2]
            {
                radius += 2;
            }
            radii[center] = radius;
            if radius > max_radius {
                max_radius = radius;
                max_center = center;
            }
            let mut inner_radius = 1;
            while center >= inner_radius
                && center + inner_radius < radii.len()
                && inner_radius + radii[center - inner_radius] < radius
            {
                radii[center + inner_radius] = radii[center - inner_radius];
                inner_radius += 1;
            }
            center += inner_radius;
            radius -= inner_radius;
        }
        src[(max_center + 2 - max_radius) / 2
            ..(max_center + 1 + max_radius) / 2]
            .into_iter()
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn check(src: &str, res: &str) {
        assert_eq!(
            Solution::longest_palindrome(String::from(src)),
            String::from(res)
        );
    }

    #[test]
    fn example1() {
        check("babad", "bab");
    }

    #[test]
    fn example2() {
        check("cbbd", "bb");
    }

    #[test]
    fn example3() {
        check("a", "a");
    }

    #[test]
    fn example4() {
        check("ac", "a");
    }

    #[test]
    fn test1() {
        check("aa", "aa");
    }

    #[test]
    fn test2() {
        check("aab", "aa");
    }

    #[test]
    fn test3() {
        check("baa", "aa");
    }

    #[test]
    fn test4() {
        check("xaba", "aba");
    }

    #[test]
    fn test5() {
        check("dqmvxouqesajlmksdawfenyaqtnnfhmqbdcniynwhuywucbjzqxhofdzvposbegkvqqrdehxzgikgtibimupumaetjknrjjuygxvncvjlahdbibatmlobctclgbmihiphshfpymgtmpeneldeygmzlpkwzouvwvqkunihmzzzrqodtepgtnljribmqneumbzusgppodmqdvxjhqwqcztcuoqlqenvuuvgxljcnwqfnvilgqrkibuehactsxphxkiwnubszjflvvuhyfwmkgkmlhmvhygncrtcttioxndbszxsyettklotadmudcybhamlcjhjpsmfvvchduxjngoajclmkxiugdtryzinivuuwlkejcgrscldgmwujfygqrximksecmfzathdytauogffxcmfjsczaxnfzvqmylujfevjwuwwaqwtcllrilyncmkjdztleictdebpkzcdilgdmzmvcllnmuwpqxqjmyoageisiaeknbwzxxezfbfejdfausfproowsyyberhiznfmrtzqtgjkyhutieyqgrzlcfvfvxawbcdaawbeqmzjrnbidnzuxfwnfiqspjtrszetubnjbznnjfjxfwtzhzejahravwmkakqsmuynklmeffangwicghckrcjwtusfpdyxxqqmfcxeurnsrmqyameuvouqspahkvouhsjqvimznbkvmtqqzpqzyqivsmznnyoauezmrgvproomvqiuzjolejptuwbdzwalfcmweqqmvdhejguwlmvkaydjrjkijtrkdezbipxoccicygmmibflxdeoxvudzeobyyrutbcydusjhmlwnfncahxgswxiupgxgvktwkdxumqp", "mupum")
    }
}
