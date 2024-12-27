fn main() {
    println!("Hello, world!");

    let collection = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let filter = FilterCondition { value: 5 };

    let filtered_result = custom_filter(collection, &filter);

    println!("Filtered result: {:?}", filtered_result);
}

struct FilterCondition {
    value: i32,
}

impl FilterCondition {
    fn is_match(&self, item: &i32) -> bool {
        *item == self.value
    }
}


fn custom_filter(collection: Vec<i32>, filter: &FilterCondition) -> Vec<i32> {
    let mut result = Vec::new();
    for item in collection {
        if filter.is_match(&item) {
            result.push(item);
        }
    }
    result
}