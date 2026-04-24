fn merge(n: usize, mut nums1: Vec<u32>, nums2: Vec<u32>) -> Vec<u32> {
    let nums1_size = nums1.len();
    let nums2_size = nums2.len();
    
    assert_eq!(nums1_size, n + nums2_size);
    for i in 0..nums2_size {
        nums1[i+n] = nums2[i];
    }
    return nums1;
}

fn main() {
    let nums1: Vec<u32> = Vec::from([1,2,3,0,0,0]);
    let nums2: Vec<u32> = Vec::from([2,5,6]);

    dbg!(merge(3, nums1, nums2));
}