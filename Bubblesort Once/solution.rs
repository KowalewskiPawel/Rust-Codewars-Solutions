fn bubblesort_once(lst: &[u32]) -> Vec<u32> {
    let mut copied_array: Vec<u32> = lst.clone().to_vec();
    
    let length = lst.len() - 1;
    for i in 0..length {
        let val0 = copied_array[i];
        let val1 = copied_array[i+1];
        
        if val0 > val1 {
            copied_array[i] = val1;
            copied_array[i+1] = val0;
        }
    }
    copied_array
}