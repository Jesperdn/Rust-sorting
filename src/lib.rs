pub trait Sorter {
    fn sort<T>(slice: &mut [T]) 
    where 
        T: Ord;
}

pub fn sort<T, S>(slice: &mut [T])
where
    T: Ord,
    S: Sorter,
{
    S::sort(slice)
}



mod bubblesort;
mod selectionsort;
mod insertionsort;




#[cfg(test)]
mod tests {
    use super::*;


    struct StdSorter;
    impl Sorter for StdSorter {
        fn sort<T>(slice: &mut [T]) 
        where 
            T: Ord,
        {
            slice.sort();        
        }
    }


    #[test]
    fn std_works() {
        let mut things = vec![4, 3, 2, 1];
        sort::<_, StdSorter>(&mut things);
        assert_eq!(things, &[1, 2, 3, 4]);
    }

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
