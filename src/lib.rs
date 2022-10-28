use std::fmt::Debug;

fn binary_search<T: PartialOrd + Debug>(number: &T, arr: &[T]) -> i32 {
    if arr.len() == 1 {
        if arr[0] == *number {
            return 0;
        }
        return -1;
    } else if arr.len() == 0 {
        return -1;
    }

    let mut left: usize = 0;
    let mut right: usize = arr.len() - 1;
    let mut mid: usize = 0;
    while left <= right {
        mid = (left + right) / 2;
        if arr[mid] > *number {
            right = mid - 1;
        } else if arr[mid] < *number {
            left = mid + 1;
        } else {
            return mid as i32;
        }
    }
    -(mid as i32) - 1_i32
}

/// Help store the data with diff types
#[derive(Clone, Copy, Debug)]
enum Content {
    Int(i32),
    Any,
}

impl PartialEq for Content {
    fn eq(&self, other: &Self) -> bool {
        use Content::*;

        match (self, other) {
            (&Int(ref a), &Int(ref b)) => a == b,
            (&Int(_), &Any) => true,
            (&Any, &Int(_)) => true,
            _ => false,
        }
    }
}

impl PartialOrd for Content {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        use Content::*;

        match (self, other) {
            (&Int(ref a), &Int(ref b)) => Some(a.cmp(b)),
            _ => Some(std::cmp::Ordering::Equal),
        }
    }

    fn lt(&self, other: &Self) -> bool {
        use Content::*;

        match (self, other) {
            (&Int(ref a), &Int(ref b)) => a < b,
            (&Int(_), &Any) => false,
            (&Any, &Int(_)) => false,
            _ => false,
        }
    }

    fn gt(&self, other: &Self) -> bool {
        use Content::*;

        match (self, other) {
            (&Int(ref a), &Int(ref b)) => a > b,
            (&Int(_), &Any) => false,
            (&Any, &Int(_)) => false,
            _ => false,
        }
    }
}

impl From<i32> for Content {
    fn from(value: i32) -> Self {
        Self::Int(value)
    }
}

impl Into<i32> for Content {
    fn into(self) -> i32 {
        match self {
            Content::Int(num) => num,
            _ => panic!(),
        }
    }
}

/// Select preferred values from available if they allowed
/// If couldnt select preferred value:
/// - select first greater value, but if couldnt, should select first lower
/// Select value if in available and in allowed
/// If couldnt select any value, return empty vector
/// All values in result uniq
/// All input data sorted! ;)
fn attempt(available: &[Content], allowed: &[Content], preferred: &[Content]) -> Vec<Content> {
    let mut result: Vec<Content> = Vec::new();
    if available.len() == 0 || allowed.len() == 0 || preferred.len() == 0 {
        return result;
    }

    let mut append_if_available = |value: &Content| -> bool {
        let avail_ind = binary_search::<Content>(value, available);
        if avail_ind >= 0 {
            // Success, add this value to result vector
            let ind = binary_search::<Content>(&available[avail_ind as usize], &result);
            if ind < 0 {
                result.push(available[avail_ind as usize]);
                return true;
            }
        }
        false
    };

    for prefer_value in preferred {
        // I think this algorithm are not perfect, for case with Any must be some better solution
        // but its work and I`m tired, so "маємо, що маємо"
        match prefer_value {
            Content::Int(_) => {
                // Check if preferred value in allowed array
                let allow_ind = binary_search::<Content>(prefer_value, allowed);
                if allow_ind >= 0 {
                    let mut allow_value = match &allowed[allow_ind as usize] {
                        // If in allowed meet Any should pick all available prefers
                        Content::Int(_) => &allowed[allow_ind as usize],
                        Content::Any => prefer_value,
                    };
                    if !append_if_available(allow_value) {
                        // Value not available
                        // let check nearest right value
                        let nearest_right = allow_ind + 1;
                        if (nearest_right as usize) < allowed.len() && nearest_right >= 0 {
                            allow_value = match &allowed[nearest_right as usize] {
                                Content::Int(_) => &allowed[nearest_right as usize],
                                Content::Any => prefer_value,
                            };
                            if !append_if_available(allow_value) {
                                // Nearest right value not available
                                // try check nearest left value
                                let nearest_left = nearest_right - 2;
                                if nearest_left > 0 {
                                    allow_value = match &allowed[nearest_left as usize] {
                                        Content::Int(_) => &allowed[nearest_left as usize],
                                        Content::Any => prefer_value,
                                    };
                                    append_if_available(allow_value);
                                }
                            }
                        }
                    }
                } else {
                    // Value not allowed
                    // let check nearest right value
                    let nearest_right = (allow_ind + 1) * -1_i32;
                    if (nearest_right as usize) < allowed.len() && nearest_right >= 0 {
                        let mut allow_value = match &allowed[nearest_right as usize] {
                            Content::Int(_) => &allowed[nearest_right as usize],
                            Content::Any => prefer_value,
                        };
                        if !append_if_available(allow_value) {
                            // Nearest right value not available
                            // try check nearest left value
                            let nearest_left = nearest_right - 1;
                            if nearest_left > 0 {
                                allow_value = match &allowed[nearest_left as usize] {
                                    Content::Int(_) => &allowed[nearest_left as usize],
                                    Content::Any => prefer_value,
                                };
                                append_if_available(allow_value);
                            }
                        }
                    }
                }
            }
            Content::Any => {
                for allow_value in allowed {
                    match allow_value {
                        // For Any-Any return all available results
                        Content::Any => {
                            return Vec::from(available);
                        }
                        _ => {}
                    }
                    // If preferred Any value, return all allowed values
                    append_if_available(allow_value);
                }
            }
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let available = [Content::Int(240), Content::Int(360), Content::Int(720)];
        let allowed = [Content::Int(240), Content::Int(360)]; // in example was 360 ,720 but in that case answer is 720
        let preferred = [Content::Int(1080)];

        assert_eq!(
            vec![Content::Int(360)],
            attempt(&available, &allowed, &preferred)
        );
    }

    #[test]
    fn test_2() {
        let available = [Content::Int(240), Content::Int(720)];
        let allowed = [Content::Int(360), Content::Int(720)];
        let preferred = [Content::Int(1080)];

        assert_eq!(
            vec![Content::Int(720)],
            attempt(&available, &allowed, &preferred)
        );
    }

    #[test]
    fn test_3() {
        let available = [Content::Int(240)];
        let allowed = [Content::Int(360), Content::Int(720)];
        let preferred = [Content::Int(1080)];

        assert_eq!(
            Vec::<Content>::new(),
            attempt(&available, &allowed, &preferred)
        );
    }

    #[test]
    fn test_4() {
        let available = [Content::Int(240), Content::Int(360), Content::Int(720)];
        let allowed = [
            Content::Int(240),
            Content::Int(360),
            Content::Int(720),
            Content::Int(1080),
        ];
        let preferred = [Content::Int(240), Content::Int(360)];

        assert_eq!(
            vec![Content::Int(240), Content::Int(360)],
            attempt(&available, &allowed, &preferred)
        );
    }

    #[test]
    fn test_5() {
        let available = [Content::Int(240), Content::Int(720)];
        let allowed = [
            Content::Int(240),
            Content::Int(360),
            Content::Int(720),
            Content::Int(1080),
        ];
        let preferred = [Content::Int(240), Content::Int(360)];

        assert_eq!(
            vec![Content::Int(240), Content::Int(720)],
            attempt(&available, &allowed, &preferred)
        );
    }

    #[test]
    fn test_6() {
        let available = [Content::Int(240), Content::Int(720)];
        let allowed = [Content::Int(240), Content::Int(360), Content::Int(1080)];
        let preferred = [Content::Int(240), Content::Int(360)];

        assert_eq!(
            vec![Content::Int(240)],
            attempt(&available, &allowed, &preferred)
        );
    }

    #[test]
    fn test_7() {
        let available = [Content::Int(720)];
        let allowed = [Content::Int(240), Content::Int(360), Content::Int(1080)];
        let preferred = [Content::Int(240), Content::Int(360)];

        assert_eq!(
            Vec::<Content>::new(),
            attempt(&available, &allowed, &preferred)
        );
    }

    #[test]
    fn test_8() {
        let available = [Content::Int(240), Content::Int(360)];
        let allowed = [Content::Int(240), Content::Int(360)];
        let preferred = [Content::Int(720), Content::Int(1080)];

        assert_eq!(
            vec![Content::Int(360)],
            attempt(&available, &allowed, &preferred)
        );
    }

    #[test]
    fn test_9() {
        let available = [Content::Int(240), Content::Int(360), Content::Int(720)];
        let allowed = [Content::Int(360), Content::Any];
        let preferred = [Content::Int(360), Content::Int(720)];

        assert_eq!(
            vec![Content::Int(360), Content::Int(720)],
            attempt(&available, &allowed, &preferred)
        );
    }

    #[test]
    fn test_10() {
        let available = [Content::Int(240), Content::Int(360), Content::Int(720)];
        let allowed = [Content::Int(240), Content::Int(360), Content::Int(720)];
        let preferred = [Content::Any, Content::Int(720)];

        assert_eq!(
            vec![Content::Int(240), Content::Int(360), Content::Int(720)],
            attempt(&available, &allowed, &preferred)
        );
    }

    #[test]
    fn test_11() {
        let available = [Content::Int(240), Content::Int(360), Content::Int(720)];
        let allowed = [Content::Int(360), Content::Int(1080)];
        let preferred = [Content::Any, Content::Int(720)];

        assert_eq!(
            vec![Content::Int(360)],
            attempt(&available, &allowed, &preferred)
        );
    }

    #[test]
    fn test_12() {
        let available = [Content::Int(240), Content::Int(360), Content::Int(720)];
        let allowed = [Content::Int(1080)];
        let preferred = [Content::Any, Content::Int(720)];

        assert_eq!(
            Vec::<Content>::new(),
            attempt(&available, &allowed, &preferred)
        );
    }

    #[test]
    fn test_13() {
        // extra test
        let available = [Content::Int(240), Content::Int(360), Content::Int(720)];
        let allowed = [Content::Int(1080), Content::Any];
        let preferred = [Content::Any, Content::Int(720)];

        assert_eq!(
            vec![Content::Int(240), Content::Int(360), Content::Int(720)],
            attempt(&available, &allowed, &preferred)
        );
    }
}
