use super::Sorter;

pub struct InsertionSort;

impl Sorter for InsertionSort {
    fn sort<T>(slice: &mut [T])
    where
        T: Ord,
    {
        for i in 1..(slice.len()) {
            let mut j = i;
            while (j > 0) && (slice[j-1] > slice[j]) {
                slice.swap(j-1, j);
                j = j - 1;
            } // end
        } // end
    }
}

#[test]
fn it_works() {
    let mut things = vec![5, 4, 3, 2, 1];
    super::sort::<_, InsertionSort>(&mut things);
    assert_eq!(things, &[1, 2, 3, 4, 5]);
}