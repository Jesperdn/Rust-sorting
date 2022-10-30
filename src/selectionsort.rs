use super::Sorter;

pub struct SelectionSort;

impl Sorter for SelectionSort {
    fn sort<T>(slice: &mut [T])
    where
        T: Ord,
    {
        for i in 0..(slice.len()-1) {
            let mut k = i;
            for j in (i+1)..(slice.len()) {
                if slice[j] < slice[k] {
                    k = j;
                }
            }
            if i != k {
                slice.swap(i, k);
            }
        }
    }
}

#[test]
fn it_works() {
    let mut things = vec![5, 4, 3, 2, 1];
    super::sort::<_, SelectionSort>(&mut things);
    assert_eq!(things, &[1, 2, 3, 4, 5]);
}